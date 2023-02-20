use crate::review_backend::{
    models::{FilterStatus, MenuChoice, MergeStrategy, PullRequest, ReviewMenuChoice},
    DefaultReviewBackend, DynReviewBackend,
};

use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Cell, Table};
use thiserror::Error;

pub struct Review {
    backend: DynReviewBackend,
}

impl Default for Review {
    fn default() -> Self {
        Self::new(std::sync::Arc::new(DefaultReviewBackend::new()))
    }
}

#[derive(Debug, Error)]
pub enum ReviewErrors {
    #[error("user chose to exit")]
    UserExit,
}

impl Review {
    fn new(backend: DynReviewBackend) -> Self {
        Self { backend }
    }

    /// Workflow
    /// 1. Fetch list of repos
    /// 2. Present menu
    /// 3. Choose begin quick review
    /// 4. Present pr and use delta to view changes
    /// 5. Approve, open, skip or quit
    /// 6. Repeat from 4
    fn run(
        &self,
        review_requested: Option<String>,
        merge_strategy: &Option<MergeStrategy>,
        filter_status: &Option<FilterStatus>,
    ) -> eyre::Result<()> {
        let prs = self.backend.get_prs(review_requested.clone())?;

        let prs_table = Self::generate_prs_table(&prs);
        self.backend.present_prs(prs_table)?;

        match self.backend.present_menu()? {
            MenuChoice::Exit => eyre::bail!(ReviewErrors::UserExit),
            MenuChoice::Begin => match self.review(&prs, &merge_strategy)? {
                Some(choice) => match choice {
                    MenuChoice::Exit => eyre::bail!(ReviewErrors::UserExit),
                    MenuChoice::List => {
                        return self.run(review_requested.clone(), merge_strategy, filter_status)
                    }
                    _ => eyre::bail!("invalid choice"),
                },
                None => {}
            },
            MenuChoice::Search => todo!(),
            MenuChoice::List => {
                return self.run(review_requested.clone(), merge_strategy, filter_status)
            }
        }

        Ok(())
    }

    fn generate_prs_table(prs: &[PullRequest]) -> String {
        let mut table = Table::new();
        let table = table
            .load_preset(UTF8_HORIZONTAL_ONLY)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("repo").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("title").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("number").add_attribute(comfy_table::Attribute::Bold),
            ])
            .add_rows(prs.iter().take(20).map(|pr| {
                let pr = pr.clone();
                vec![
                    Cell::new(pr.repository.name).fg(comfy_table::Color::Green),
                    Cell::new(pr.title),
                    Cell::new(pr.number.to_string()),
                ]
            }));

        table.to_string()
    }

    fn review(
        &self,
        prs: &Vec<PullRequest>,
        merge_strategy: &Option<MergeStrategy>,
    ) -> eyre::Result<Option<MenuChoice>> {
        for pr in prs {
            self.backend.clear()?;
            self.backend.present_pr(pr)?;
            self.review_pr(pr)?;
            if let Some(choice) = self.present_pr_menu(pr, merge_strategy)? {
                return Ok(Some(choice));
            }
        }

        Ok(None)
    }

    fn review_pr(&self, pr: &PullRequest) -> eyre::Result<()> {
        self.backend.present_diff(pr)?;
        Ok(())
    }

    fn approve(&self, pr: &PullRequest) -> eyre::Result<()> {
        self.backend.approve(pr)?;

        Ok(())
    }

    fn open_browser(
        &self,
        pr: &PullRequest,
        merge_strategy: &Option<MergeStrategy>,
    ) -> eyre::Result<Option<MenuChoice>> {
        self.backend.pr_open_browser(pr)?;

        self.present_pr_menu(pr, merge_strategy)
    }

    fn present_pr_menu(
        &self,
        pr: &PullRequest,
        merge_strategy: &Option<MergeStrategy>,
    ) -> eyre::Result<Option<MenuChoice>> {
        self.backend.present_pr(pr)?;

        self.present_status_checks(pr)?;

        match self.backend.present_review_menu(pr)? {
            ReviewMenuChoice::Exit => return Ok(Some(MenuChoice::Exit)),
            ReviewMenuChoice::List => return Ok(Some(MenuChoice::List)),
            ReviewMenuChoice::Approve => {
                self.approve(pr)?;
                return self.present_pr_menu(pr, merge_strategy);
            }
            ReviewMenuChoice::Open => return self.open_browser(pr, merge_strategy),
            ReviewMenuChoice::Skip => {}
            ReviewMenuChoice::Merge => {
                if let Err(e) = self.merge(pr, merge_strategy) {
                    println!("could not merge: {}", e);
                    return self.present_pr_menu(pr, merge_strategy);
                }
            }
            ReviewMenuChoice::ApproveAndMerge => {
                self.approve(pr)?;
                if let Err(e) = self.merge(pr, merge_strategy) {
                    println!("could not merge: {}", e);
                    return self.present_pr_menu(pr, merge_strategy);
                }
            }
            ReviewMenuChoice::Diff => {
                self.review_pr(pr)?;
                return self.present_pr_menu(pr, merge_strategy);
            }
            ReviewMenuChoice::StatusChecks => {
                self.present_status_checks(pr)?;
                return self.present_pr_menu(pr, merge_strategy);
            }
        }

        Ok(None)
    }

    fn merge(&self, pr: &PullRequest, merge_strategy: &Option<MergeStrategy>) -> eyre::Result<()> {
        if let Err(e) = self.backend.enable_auto_merge(pr, merge_strategy) {
            println!("could not enable auto-merge merge because: {}", e);
            if let Err(e) = self.backend.merge(pr, merge_strategy) {
                println!("could not merge because: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    fn present_status_checks(&self, pr: &PullRequest) -> eyre::Result<()> {
        self.backend.present_status_checks(pr)?;

        Ok(())
    }
}

impl util::Cmd for Review {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("review")
            .arg(
                clap::Arg::new("review-requested")
                    .long("review-requested")
                    .default_value("@me")
                    .help("which user or team to pull reviews from"),
            )
            .arg(
                clap::Arg::new("merge-strategy")
                    .long("merge-strategy")
                    .help(
                    "when merging which merge strategy to use, possible values: [squash, merge]",
                ),
            )
            .arg(
                clap::Arg::new("filter-status")
                    .long("filter-status")
                    .alias("fs")
                    .help("filter status, these include [SUCCESS, FAILURE, INCOMPLETE]"),
            ))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        let request_requested = args
            .get_one::<String>("review-requested")
            .map(|r| r.clone());

        let squash = args
            .get_one::<String>("merge-strategy")
            .and_then(|s| match s.as_str() {
                "squash" => Some(MergeStrategy::Squash),
                "merge" => Some(MergeStrategy::MergeCommit),
                _ => None,
            });

        let filter_status =
            args.get_one::<String>("filter-status")
                .and_then(|s| match s.as_str() {
                    "SUCCESS" => Some(FilterStatus::Success),
                    "FAILURE" => Some(FilterStatus::Failure),
                    "INCOMPLETE" => Some(FilterStatus::Incomplete),
                    _ => None,
                });

        Self::default().run(request_requested, &squash, &filter_status)
    }
}

#[cfg(test)]
mod tests {
    use crate::review_backend::{
        models::{self, Repository},
        MockReviewBackend,
    };

    use super::*;

    use base64::Engine;
    use mockall::predicate::eq;
    use pretty_assertions::assert_eq;

    #[test]
    fn can_fetch_prs() {
        let mut backend = MockReviewBackend::new();
        let prs = vec![
            PullRequest {
                title: "some-title".into(),
                number: 0,
                repository: Repository {
                    name: "some-name".into(),
                },
            },
            PullRequest {
                title: "some-other-title".into(),
                number: 1,
                repository: Repository {
                    name: "some-other-name".into(),
                },
            },
        ];
        let backendprs = prs.clone();
        backend
            .expect_get_prs()
            .with(eq(Some("kjuulh".into())))
            .times(1)
            .returning(move |_| Ok(backendprs.clone()));

        backend
            .expect_present_menu()
            .times(1)
            .returning(|| Ok(models::MenuChoice::Exit));

        backend.expect_present_prs().times(1).returning(|_| Ok(()));

        let review = Review::new(std::sync::Arc::new(backend));
        let res = review.run(None, &None);

        assert_err::<ReviewErrors, _>(res)
    }

    #[test]
    fn can_generate_table() {
        let prs = vec![
            PullRequest {
                title: "some-title".into(),
                number: 0,
                repository: Repository {
                    name: "some-name".into(),
                },
            },
            PullRequest {
                title: "some-other-title".into(),
                number: 1,
                repository: Repository {
                    name: "some-other-name".into(),
                },
            },
        ];

        let expected_table = "4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSAChtbMW0gcmVwbyAgICAgICAgICAgIBtbMG0gG1sxbSB0aXRsZSAgICAgICAgICAgIBtbMG0gG1sxbSBudW1iZXIgG1swbQrilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZDilZAKG1szODs1OzEwbSBzb21lLW5hbWUgICAgICAgG1szOW0gIHNvbWUtdGl0bGUgICAgICAgICAwICAgICAgCuKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgOKUgAobWzM4OzU7MTBtIHNvbWUtb3RoZXItbmFtZSAbWzM5bSAgc29tZS1vdGhlci10aXRsZSAgIDEgICAgICAK4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA4pSA";
        let output = Review::generate_prs_table(&prs);

        compare_tables(output, expected_table)
    }

    fn compare_tables(actual: String, snapshot: &str) {
        let b64 = base64::engine::general_purpose::STANDARD_NO_PAD;
        let snapshot = snapshot.clone().replace("\n", "").replace(" ", "");
        println!("expected");
        println!(
            "{}",
            std::str::from_utf8(
                b64.decode(&snapshot)
                    .expect("table to be decodeable")
                    .as_slice()
            )
            .expect("to be utf8")
        );

        println!("actual");
        println!("{actual}");

        assert_eq!(b64.encode(actual), snapshot);
    }

    fn assert_err<TExpected, TVal>(res: eyre::Result<TVal>) {
        match res {
            Err(e) => {
                if !e.is::<ReviewErrors>() {
                    panic!("invalid error: {}", e)
                }
            }
            _ => panic!("error not thrown"),
        }
    }
}
