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
}
