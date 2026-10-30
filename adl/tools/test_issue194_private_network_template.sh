#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TEMPLATE="${REPO_ROOT}/adl/tools/issue194_private_network.cloudformation.json"

cd "${REPO_ROOT}"

python3 - "${TEMPLATE}" <<'PY'
import json
import pathlib
import sys

template = json.loads(pathlib.Path(sys.argv[1]).read_text())
resources = template["Resources"]
parameters = template["Parameters"]

assert parameters["LaunchVoters"]["Default"] == "false"
assert parameters["LaunchVoterA"]["Default"] == "true"
assert parameters["LaunchVoterB"]["Default"] == "true"
assert parameters["S3PrefixListId"]["Type"] == "String"
assert template["Conditions"]["ShouldLaunchVoters"] == {"Fn::Equals": [{"Ref": "LaunchVoters"}, "true"]}
assert "ShouldLaunchVoterA" in template["Conditions"]
assert "ShouldLaunchVoterB" in template["Conditions"]

for forbidden in (
    "AWS::EC2::InternetGateway",
    "AWS::EC2::NatGateway",
    "AWS::EC2::EIP",
):
    assert all(resource["Type"] != forbidden for resource in resources.values()), forbidden

subnets = [
    resource
    for resource in resources.values()
    if resource["Type"] == "AWS::EC2::Subnet"
]
assert len(subnets) == 2
assert all(resource["Properties"]["MapPublicIpOnLaunch"] is False for resource in subnets)

routes = [
    resource
    for resource in resources.values()
    if resource["Type"] == "AWS::EC2::Route"
]
assert routes == []

instance_sg = resources["InstanceSecurityGroup"]["Properties"]
assert instance_sg["SecurityGroupIngress"] == []
assert instance_sg["SecurityGroupEgress"] == []

egress = resources["InstanceToEndpointEgress"]
assert egress["Type"] == "AWS::EC2::SecurityGroupEgress"
egress_properties = egress["Properties"]
assert egress_properties["IpProtocol"] == "tcp"
assert egress_properties["FromPort"] == 443
assert "DestinationSecurityGroupId" in egress_properties
assert "CidrIp" not in egress_properties

mesh_ingress = resources["VoterMeshIngress"]
assert mesh_ingress["Type"] == "AWS::EC2::SecurityGroupIngress"
mesh_ingress_properties = mesh_ingress["Properties"]
assert mesh_ingress_properties["IpProtocol"] == "-1"
assert mesh_ingress_properties["SourceSecurityGroupId"] == {"Ref": "InstanceSecurityGroup"}
assert "CidrIp" not in mesh_ingress_properties

mesh_egress = resources["VoterMeshEgress"]
assert mesh_egress["Type"] == "AWS::EC2::SecurityGroupEgress"
mesh_egress_properties = mesh_egress["Properties"]
assert mesh_egress_properties["IpProtocol"] == "-1"
assert mesh_egress_properties["DestinationSecurityGroupId"] == {"Ref": "InstanceSecurityGroup"}
assert "CidrIp" not in mesh_egress_properties

s3_egress = resources["InstanceToS3GatewayEgress"]
assert s3_egress["Type"] == "AWS::EC2::SecurityGroupEgress"
s3_egress_properties = s3_egress["Properties"]
assert s3_egress_properties["IpProtocol"] == "tcp"
assert s3_egress_properties["FromPort"] == 443
assert s3_egress_properties["ToPort"] == 443
assert s3_egress_properties["DestinationPrefixListId"] == {"Ref": "S3PrefixListId"}
assert "CidrIp" not in s3_egress_properties

endpoint_sg = resources["EndpointSecurityGroup"]["Properties"]
assert endpoint_sg["SecurityGroupEgress"] == []
assert endpoint_sg["SecurityGroupIngress"] == []

ingress = resources["EndpointFromInstanceIngress"]
assert ingress["Type"] == "AWS::EC2::SecurityGroupIngress"
ingress_properties = ingress["Properties"]
assert ingress_properties["SourceSecurityGroupId"] == {"Ref": "InstanceSecurityGroup"}
assert "CidrIp" not in ingress_properties

endpoint_services = {
    resource["Properties"]["ServiceName"]["Fn::Sub"]
    for resource in resources.values()
    if resource["Type"] == "AWS::EC2::VPCEndpoint"
}
assert endpoint_services == {
    "com.amazonaws.${AWS::Region}.ssm",
    "com.amazonaws.${AWS::Region}.ssmmessages",
    "com.amazonaws.${AWS::Region}.ec2messages",
    "com.amazonaws.${AWS::Region}.s3",
}
s3_policy = resources["S3GatewayEndpoint"]["Properties"]["PolicyDocument"]
statements = {statement["Sid"]: statement for statement in s3_policy["Statement"]}
assert statements["AllowShepherdArtifactReads"]["Action"] == ["s3:GetObject", "s3:GetObjectVersion"]
assert "Condition" not in statements["AllowShepherdArtifactReads"]
assert statements["AllowShepherdArtifactListing"]["Action"] == ["s3:ListBucket", "s3:ListBucketVersions"]
assert statements["AllowShepherdArtifactListing"]["Condition"]["StringLike"]["s3:prefix"] == "shepherd/*"

for resource in resources.values():
    if resource["Type"] == "AWS::EC2::VPCEndpoint":
        tags = {tag["Key"]: tag["Value"] for tag in resource["Properties"]["Tags"]}
        assert tags["adl:issue"] == "194"
        assert tags["adl:run_id"] == {"Ref": "RunId"}
        assert tags["adl:cleanup_required"] == "true"

voters = [resources["AwsVoterA"], resources["AwsVoterB"]]
assert all(voter["Type"] == "AWS::EC2::Instance" for voter in voters)
assert resources["AwsVoterA"]["Condition"] == "ShouldLaunchVoterA"
assert resources["AwsVoterB"]["Condition"] == "ShouldLaunchVoterB"
assert {voter["Properties"]["SubnetId"]["Ref"] for voter in voters} == {"PrivateSubnetA", "PrivateSubnetB"}
assert all(voter["Properties"]["SecurityGroupIds"] == [{"Ref": "InstanceSecurityGroup"}] for voter in voters)
assert all(voter["Properties"]["IamInstanceProfile"] == {"Ref": "InstanceProfileName"} for voter in voters)
assert all(
    voter["Properties"]["BlockDeviceMappings"][0]["Ebs"]["DeleteOnTermination"] is True
    for voter in voters
)
assert all(voter["Properties"]["PropagateTagsToVolumeOnCreation"] is True for voter in voters)
for voter in voters:
    tags = {tag["Key"]: tag["Value"] for tag in voter["Properties"]["Tags"]}
    assert tags["adl:issue"] == "194"
    assert tags["adl:cleanup_required"] == "true"
    assert tags["adl:public_runtime_exposure"] == "false"
    assert tags["adl:hosted_model_fallback"] == "false"

for resource in resources.values():
    properties = resource.get("Properties", {})
    tags = {tag["Key"]: tag["Value"] for tag in properties.get("Tags", [])}
    if resource["Type"] in {
        "AWS::EC2::VPC",
        "AWS::EC2::Subnet",
        "AWS::EC2::RouteTable",
        "AWS::EC2::SecurityGroup",
    }:
        assert tags.get("adl:issue") == "194"
        assert tags.get("adl:cleanup_required") == "true"
        assert tags.get("adl:run_id") == {"Ref": "RunId"}
        assert tags.get("adl:ttl_expires_at") == {"Ref": "TtlExpiresAt"}

print("#194 private network template contract: PASS")
PY
