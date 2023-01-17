use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Repository {
    #[serde(rename(deserialize = "nameWithOwner"))]
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PullRequest {
    pub title: String,
    pub number: usize,
    pub repository: Repository,
}

#[derive(Debug, Clone)]
pub enum MergeStrategy {
    Squash,
    MergeCommit,
}

pub enum MenuChoice {
    Exit,
    Begin,
    Search,
    List,
}

pub enum ReviewMenuChoice {
    Exit,
    List,
    Approve,
    Open,
    Skip,
    Merge,
    ApproveAndMerge,
    Diff,
    StatusChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusChecks {
    #[serde(rename(deserialize = "statusCheckRollup"))]
    pub checks: Vec<StatusCheck>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "__typename")]
pub enum StatusCheck {
    CheckRun(CheckRun),
    StatusContext(StatusContext),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckRun {
    pub conclusion: String,
    #[serde(alias = "detailsUrl")]
    pub details_url: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusContext {
    #[serde(alias = "targetUrl")]
    pub target_url: String,
    pub context: String,
    pub state: String,
}
