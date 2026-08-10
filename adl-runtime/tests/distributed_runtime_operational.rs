use std::{
    collections::BTreeMap,
    net::{Ipv4Addr, SocketAddr, TcpListener},
    path::Path,
    time::Duration,
};

use adl_runtime::distributed::polis_runtime::{
    NodeId, PolisAuthorityConfig, PolisCommand, PolisLogStore, PolisRuntime, PolisRuntimeConfig,
};
use adl_runtime_kernel::{
    read_distributed_observatory_projection, DistributedObservatoryProjection,
};
use ed25519_dalek::SigningKey;
use openraft::BasicNode;
use tempfile::TempDir;

fn reserve_address() -> SocketAddr {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("reserve loopback port");
    listener.local_addr().expect("reserved address")
}

fn signing_key(node_id: NodeId) -> SigningKey {
    SigningKey::from_bytes(&[node_id as u8; 32])
}

const LOCAL_KERNEL_TOKEN: &str = "test-local-kernel-token-000000000001";

fn node_map(addresses: &BTreeMap<NodeId, SocketAddr>) -> BTreeMap<NodeId, BasicNode> {
    addresses
        .iter()
        .map(|(node_id, address)| (*node_id, BasicNode::new(address)))
        .collect()
}

fn runtime_config(
    node_id: NodeId,
    bootstrap: bool,
    addresses: &BTreeMap<NodeId, SocketAddr>,
    state_root: &Path,
) -> PolisRuntimeConfig {
    let peer_keys = (1..=3)
        .filter(|peer_id| *peer_id != node_id)
        .map(|peer_id| (peer_id, signing_key(peer_id).verifying_key()))
        .collect();
    PolisRuntimeConfig {
        polis_id: "operational-test-polis".to_owned(),
        trust_domain: "operational-test-domain".to_owned(),
        local_id: node_id,
        listen_address: addresses[&node_id],
        nodes: node_map(addresses),
        bootstrap,
        state_root: state_root.join(format!("node-{node_id}")),
        signing_key: signing_key(node_id),
        peer_keys,
        local_kernel_token: LOCAL_KERNEL_TOKEN.to_owned(),
    }
}

async fn wait_for_leader(nodes: &BTreeMap<NodeId, Option<PolisRuntime>>) -> NodeId {
    tokio::time::timeout(Duration::from_secs(12), async {
        loop {
            let observed = nodes
                .values()
                .filter_map(Option::as_ref)
                .filter_map(|runtime| runtime.raft.metrics().borrow().current_leader)
                .collect::<Vec<_>>();
            if let Some(leader) = observed.first().copied() {
                if observed.iter().all(|candidate| *candidate == leader) {
                    return leader;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("three-node cluster elects one leader")
}

async fn start_cluster(
    root: &Path,
) -> (
    BTreeMap<NodeId, SocketAddr>,
    BTreeMap<NodeId, Option<PolisRuntime>>,
) {
    let addresses = (1..=3)
        .map(|node_id| (node_id, reserve_address()))
        .collect();
    let mut nodes = BTreeMap::new();
    for node_id in [2, 3] {
        let runtime = PolisRuntime::start(runtime_config(node_id, false, &addresses, root))
            .await
            .expect("start follower");
        nodes.insert(node_id, Some(runtime));
    }
    let bootstrap = PolisRuntime::start(runtime_config(1, true, &addresses, root))
        .await
        .expect("start bootstrap node");
    nodes.insert(1, Some(bootstrap));
    (addresses, nodes)
}

async fn stop_node(nodes: &mut BTreeMap<NodeId, Option<PolisRuntime>>, node_id: NodeId) {
    nodes
        .get_mut(&node_id)
        .expect("known node")
        .take()
        .expect("running node")
        .shutdown()
        .await
        .expect("clean node shutdown");
}

fn authority_config(node_id: NodeId, root: &Path) -> PolisAuthorityConfig {
    PolisAuthorityConfig {
        polis_id: "operational-test-polis".to_owned(),
        trust_domain: "operational-test-domain".to_owned(),
        guardian_id: format!("guardian-{node_id}"),
        shepherd_identity_ref: "shepherd-identity".to_owned(),
        voter_ids: (1..=3)
            .map(|voter_id| (voter_id, format!("voter-{voter_id}")))
            .collect(),
        projection_path: root.join(format!("observatory-{node_id}.json")),
        lease_millis: 3_000,
    }
}

async fn wait_for_single_observatory_owner(root: &Path, previous: Option<&str>) -> String {
    let result = tokio::time::timeout(Duration::from_secs(12), async {
        loop {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let mut owners = Vec::new();
            for node_id in 1..=3 {
                let Ok(bytes) = std::fs::read(root.join(format!("observatory-{node_id}.json")))
                else {
                    continue;
                };
                let Ok(projection) =
                    serde_json::from_slice::<DistributedObservatoryProjection>(&bytes)
                else {
                    continue;
                };
                if projection.owner_guardian_id == format!("guardian-{node_id}")
                    && projection.expires_unix_millis > now
                {
                    owners.push(projection.owner_guardian_id);
                }
            }
            owners.sort();
            owners.dedup();
            if owners.len() == 1 && previous.is_none_or(|owner| owners[0] != owner) {
                return owners.remove(0);
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await;
    if let Ok(owner) = result {
        return owner;
    }
    for node_id in 1..=3 {
        let path = root.join(format!("observatory-{node_id}.json"));
        eprintln!("{}: {:?}", path.display(), std::fs::read_to_string(&path));
    }
    panic!("exactly one non-expired Observatory owner");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn three_voters_commit_halt_without_quorum_and_recover_durable_state() {
    let cwd = std::env::current_dir().expect("current directory");
    let root = TempDir::new_in(cwd).expect("ordinary test root");
    let (addresses, mut nodes) = start_cluster(root.path()).await;
    let leader = wait_for_leader(&nodes).await;

    let first = nodes[&leader]
        .as_ref()
        .expect("leader running")
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "before-partition".to_owned(),
            payload_sha256: "11".repeat(32),
        })
        .await
        .expect("quorum commits governed mutation");
    assert!(first.data.accepted);

    let client = reqwest::Client::new();
    let endpoint = format!(
        "http://{}/internal/client/governed-mutation",
        addresses[&leader]
    );
    let request = serde_json::json!({
        "schema": "adl.distributed.local_governed_mutation.v1",
        "mutation_id": "kernel-authorized-result",
        "payload_sha256": "55".repeat(32),
    });
    assert_eq!(
        client
            .post(&endpoint)
            .json(&request)
            .send()
            .await
            .expect("unauthenticated local request completes")
            .status(),
        reqwest::StatusCode::UNAUTHORIZED
    );
    let authorized = client
        .post(&endpoint)
        .bearer_auth(LOCAL_KERNEL_TOKEN)
        .json(&request)
        .send()
        .await
        .expect("local kernel reaches consensus endpoint");
    assert!(authorized.status().is_success());
    assert_eq!(
        authorized
            .json::<serde_json::Value>()
            .await
            .expect("commit response")["accepted"],
        true
    );

    let followers = [1, 2, 3]
        .into_iter()
        .filter(|node_id| *node_id != leader)
        .collect::<Vec<_>>();
    stop_node(&mut nodes, followers[0]).await;
    let with_two = nodes[&leader]
        .as_ref()
        .expect("leader running")
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "two-voters-still-commit".to_owned(),
            payload_sha256: "22".repeat(32),
        })
        .await
        .expect("two of three voters retain quorum");
    assert!(with_two.data.accepted);

    stop_node(&mut nodes, followers[1]).await;
    let no_quorum = tokio::time::timeout(
        Duration::from_secs(3),
        nodes[&leader]
            .as_ref()
            .expect("isolated leader running")
            .client_write(PolisCommand::GovernedMutation {
                mutation_id: "must-not-commit".to_owned(),
                payload_sha256: "33".repeat(32),
            }),
    )
    .await;
    assert!(
        no_quorum.is_err() || no_quorum.expect("bounded write result").is_err(),
        "one of three voters must not authorize mutation"
    );
    let isolated_state = nodes[&leader]
        .as_ref()
        .expect("isolated leader running")
        .state_machine
        .application_state()
        .await;
    assert!(
        !isolated_state.mutation_ids.contains("must-not-commit"),
        "the isolated voter must not apply an uncommitted write"
    );

    let restarted =
        PolisRuntime::start(runtime_config(followers[0], false, &addresses, root.path()))
            .await
            .expect("restart voter from durable store");
    nodes.insert(followers[0], Some(restarted));
    let recovered_leader = wait_for_leader(&nodes).await;
    let recovered = nodes[&recovered_leader]
        .as_ref()
        .expect("recovered leader running")
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "after-recovery".to_owned(),
            payload_sha256: "44".repeat(32),
        })
        .await
        .expect("restored quorum commits");
    assert!(recovered.data.accepted);

    nodes[&recovered_leader]
        .as_ref()
        .expect("leader running")
        .raft
        .trigger()
        .snapshot()
        .await
        .expect("build durable snapshot");
    let state_before = nodes[&recovered_leader]
        .as_ref()
        .expect("leader running")
        .state_machine
        .application_state()
        .await;
    assert!(state_before.mutation_ids.contains("before-partition"));
    assert!(state_before
        .mutation_ids
        .contains("two-voters-still-commit"));
    assert!(state_before.mutation_ids.contains("after-recovery"));
    let replay = nodes[&recovered_leader]
        .as_ref()
        .expect("leader running")
        .client_write(PolisCommand::GovernedMutation {
            mutation_id: "after-recovery".to_owned(),
            payload_sha256: "44".repeat(32),
        })
        .await
        .expect("duplicate request reaches the committed state machine");
    assert!(!replay.data.accepted);
    assert_eq!(replay.data.reason_code, "governed_rejection");

    for node_id in [1, 2, 3] {
        if nodes[&node_id].is_some() {
            stop_node(&mut nodes, node_id).await;
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn observatory_lease_moves_only_after_the_previous_owner_expires() {
    let cwd = std::env::current_dir().expect("current directory");
    let root = TempDir::new_in(cwd).expect("ordinary test root");
    let (_addresses, mut nodes) = start_cluster(root.path()).await;
    for node_id in 1..=3 {
        nodes
            .get_mut(&node_id)
            .and_then(Option::as_mut)
            .expect("running node")
            .start_authority_loop(authority_config(node_id, root.path()))
            .expect("start authority loop");
    }
    let first_owner = wait_for_single_observatory_owner(root.path(), None).await;
    let first_owner_node: NodeId = first_owner
        .strip_prefix("guardian-")
        .expect("guardian prefix")
        .parse()
        .expect("numeric guardian");
    let projection_path = root
        .path()
        .join(format!("observatory-{first_owner_node}.json"));
    assert!(read_distributed_observatory_projection(
        &projection_path,
        &first_owner,
        LOCAL_KERNEL_TOKEN,
    )
    .is_some());
    let mut forged: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&projection_path).expect("read signed projection"))
            .expect("projection JSON");
    forged["committed_index"] = serde_json::json!(u64::MAX);
    std::fs::write(
        &projection_path,
        serde_json::to_vec(&forged).expect("forged projection JSON"),
    )
    .expect("write forged projection");
    assert!(read_distributed_observatory_projection(
        &projection_path,
        &first_owner,
        LOCAL_KERNEL_TOKEN,
    )
    .is_none());
    let first_owner = wait_for_single_observatory_owner(root.path(), None).await;
    assert_eq!(first_owner, format!("guardian-{first_owner_node}"));
    stop_node(&mut nodes, first_owner_node).await;
    let second_owner =
        wait_for_single_observatory_owner(root.path(), Some(first_owner.as_str())).await;
    assert_ne!(first_owner, second_owner);

    for node_id in 1..=3 {
        if nodes[&node_id].is_some() {
            stop_node(&mut nodes, node_id).await;
        }
    }
}

#[cfg(unix)]
#[tokio::test]
async fn distributed_store_rejects_a_symlinked_path_component() {
    use std::os::unix::fs::symlink;

    let cwd = std::env::current_dir().expect("current directory");
    let root = TempDir::new_in(cwd).expect("ordinary test root");
    let real = root.path().join("real");
    std::fs::create_dir(&real).expect("real state parent");
    let linked = root.path().join("linked");
    symlink(&real, &linked).expect("test symlink");
    let result = PolisLogStore::open(&linked.join("node-1"));
    assert!(
        result.is_err(),
        "a symlinked state ancestor must fail closed"
    );
}
