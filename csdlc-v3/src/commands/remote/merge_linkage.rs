//! #849: exact-review linkage and authenticated qualified issue observations.
use super::*;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationLinkage {
    /// Issue repository; it may differ from the code/PR repository.
    pub repository: String,
    pub issue: u64,
    pub mode: RemotePublicationMode,
}

fn repository_parts(repository: &str) -> Option<(&str, &str)> {
    let (owner, name) = repository.split_once('/')?;
    [owner, name]
        .iter()
        .all(|s| {
            !s.is_empty()
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        })
        .then_some((owner, name))
}

/// The adapter accepts only a numeric PR and qualified issue, never query text.
pub fn merge_linkage_query(repository: &str, target: &str) -> Option<String> {
    let (number, issue_target) = target.split_once(':')?;
    let (issue_repository, issue) = issue_target.split_once('#')?;
    let (owner, name) = repository_parts(repository)?;
    let (issue_owner, issue_name) = repository_parts(issue_repository)?;
    if number.parse::<u64>().ok()? == 0 || issue.parse::<u64>().ok()? == 0 {
        return None;
    }
    let mut query = merge_state_query(owner, name, number);
    // Add a separately qualified issue observation in the same authenticated response.
    let end = query.rfind('}')?;
    query.insert_str(end, &format!(r#" linkedRepository: repository(owner:"{issue_owner}", name:"{issue_name}") {{ nameWithOwner issue(number:{issue}) {{ number url state }} }} "#));
    Some(query)
}

impl PublicationLinkage {
    pub(super) fn valid_for(&self, request: &GithubMutationRequest) -> bool {
        repository_parts(&self.repository).is_some()
            && self.issue > 0
            && self.issue == request.issue
    }

    pub(super) fn observation_target(&self, request: &GithubMutationRequest) -> String {
        format!(
            "{}:{}#{}",
            request.pull_request.unwrap_or_default(),
            self.repository,
            self.issue
        )
    }

    pub(super) fn validate(
        &self,
        value: &Value,
        request: &GithubMutationRequest,
        merged: bool,
    ) -> Result<String, RemoteRouteFinding> {
        let reject = || {
            remote_finding(
                "github_merge_linkage_ineligible",
                "reviewed publication linkage or authenticated issue state mismatch",
            )
        };
        let pr = &value["data"]["repository"]["pullRequest"];
        let body = pr["body"].as_str().ok_or_else(reject)?;
        let qualified = format!("{}#{}", self.repository, self.issue);
        let short = format!("#{}", self.issue);
        let reference_matches = |reference: &str| {
            reference == qualified || (self.repository == request.repository && reference == short)
        };
        let mut closing = 0;
        let mut part_of = 0;
        let mut matched = false;
        // Canonical whole-line relations are deliberate: ambiguous Markdown is rejected.
        for line in body.lines() {
            let lower = line.trim().to_ascii_lowercase();
            let words: Vec<_> = lower.split_whitespace().collect();
            for (index, word) in words.iter().enumerate() {
                let word = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
                if matches!(
                    word,
                    "close"
                        | "closes"
                        | "closed"
                        | "fix"
                        | "fixes"
                        | "fixed"
                        | "resolve"
                        | "resolves"
                        | "resolved"
                ) && words.get(index + 1).is_some_and(|reference| {
                    reference.contains('#') || reference.starts_with("https://github.com/")
                }) {
                    closing += 1;
                }
            }
            // Detection is broader than admission: URL forms (including Markdown
            // wrappers) must count as directives even though only canonical
            // issue references below can authorize a merge.
            let part_of_reference = |references: &[&str]| {
                let reference = references[0];
                reference.contains('#')
                    || reference.contains("://")
                    || (reference.starts_with('[')
                        && references
                            .join(" ")
                            .split_once("](")
                            .is_some_and(|(_, url)| url.contains("://")))
            };
            part_of += words
                .windows(3)
                .enumerate()
                .filter(|(i, w)| w[..2] == ["part", "of"] && part_of_reference(&words[i + 2..]))
                .count()
                + words
                    .windows(2)
                    .enumerate()
                    .filter(|(i, w)| w[0] == "part-of" && part_of_reference(&words[i + 1..]))
                    .count();
            let original: Vec<_> = line.split_whitespace().collect();
            let relation = match self.mode {
                RemotePublicationMode::Closing
                    if words.len() == 2
                        && matches!(
                            words[0],
                            "close"
                                | "closes"
                                | "closed"
                                | "fix"
                                | "fixes"
                                | "fixed"
                                | "resolve"
                                | "resolves"
                                | "resolved"
                        ) =>
                {
                    Some(original[1])
                }
                RemotePublicationMode::PartOf
                    if words.len() == 3 && words[..2] == ["part", "of"] =>
                {
                    Some(original[2])
                }
                RemotePublicationMode::PartOf if words.len() == 2 && words[0] == "part-of" => {
                    Some(original[1])
                }
                _ => None,
            };
            matched |= relation.is_some_and(reference_matches);
        }
        let expected_counts = match self.mode {
            RemotePublicationMode::Closing => (1, 0),
            RemotePublicationMode::PartOf => (0, 1),
        };
        if !matched || (closing, part_of) != expected_counts {
            return Err(reject());
        }
        let links = &pr["closingIssuesReferences"];
        if links["pageInfo"]["hasNextPage"] != false {
            return Err(reject());
        }
        let nodes = links["nodes"].as_array().ok_or_else(reject)?;
        let issue_matches = |node: &Value| {
            node["number"] == self.issue
                && node["repository"]["nameWithOwner"] == self.repository
                && node["url"]
                    == format!(
                        "https://github.com/{}/issues/{}",
                        self.repository, self.issue
                    )
        };
        match self.mode {
            RemotePublicationMode::Closing if nodes.len() == 1 && issue_matches(&nodes[0]) => {}
            RemotePublicationMode::PartOf if nodes.is_empty() => {}
            _ => return Err(reject()),
        }
        let repo = &value["data"]["linkedRepository"];
        let issue = &repo["issue"];
        let expected_state = if merged && self.mode == RemotePublicationMode::Closing {
            "CLOSED"
        } else {
            "OPEN"
        };
        if repo["nameWithOwner"] != self.repository
            || issue["number"] != self.issue
            || issue["url"]
                != format!(
                    "https://github.com/{}/issues/{}",
                    self.repository, self.issue
                )
            || issue["state"] != expected_state
        {
            return Err(reject());
        }
        Ok(expected_state.into())
    }
}
