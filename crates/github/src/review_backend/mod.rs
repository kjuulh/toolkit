pub mod models;

use self::models::PullRequest;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait ReviewBackend {
    fn get_prs(&self, review_request: Option<String>) -> eyre::Result<Vec<PullRequest>>;
    fn present_prs(&self, table: String) -> eyre::Result<()>;
}

pub type DynReviewBackend = std::sync::Arc<dyn ReviewBackend + Send + Sync>;

#[derive(Default)]
pub struct DefaultReviewBackend;

impl ReviewBackend for DefaultReviewBackend {
    fn get_prs(&self, review_request: Option<String>) -> eyre::Result<Vec<PullRequest>> {
        let raw_prs = util::shell::run_with_input_and_output(
            &[
                "gh",
                "search",
                "prs",
                "--state=open",
                "--review-requested",
                review_request.unwrap().as_str(),
                "--label",
                "dependencies",
                "--checks=pending",
                "--json",
                "repository,number,title",
            ],
            "".into(),
        )?;

        let prs_json = std::str::from_utf8(raw_prs.stdout.as_slice())?;

        let prs: Vec<PullRequest> = serde_json::from_str(prs_json)?;

        Ok(prs)
    }

    fn present_prs(&self, table: String) -> eyre::Result<()> {
        Ok(())
    }
}

impl DefaultReviewBackend {
    pub fn new() -> Self {
        Self {}
    }
}
