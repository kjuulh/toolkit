use crate::review_backend::{models::PullRequest, DefaultReviewBackend, DynReviewBackend};

use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Cell, Row, Table};
#[cfg(test)]
use mockall::{automock, mock, predicate::*};

pub struct Review {
    backend: DynReviewBackend,
}

impl Default for Review {
    fn default() -> Self {
        Self::new(std::sync::Arc::new(DefaultReviewBackend::new()))
    }
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
    fn run(&self, review_requested: Option<String>) -> eyre::Result<()> {
        let prs = self.backend.get_prs(review_requested)?;

        let prs_table = Self::generate_prs_table(&prs);

        self.backend.present_prs(prs_table)?;

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
}

impl util::Cmd for Review {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("review"))
    }

    fn exec(_: &clap::ArgMatches) -> eyre::Result<()> {
        Self::default().run(Some("lunarway/squad-aura".into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::review_backend::{models::Repository, MockReviewBackend};

    use super::*;

    use base64::Engine;
    use pretty_assertions::{assert_eq, assert_ne};

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

        backend.expect_present_prs().times(1).returning(|_| Ok(()));

        let review = Review::new(std::sync::Arc::new(backend));
        review
            .run(Some("kjuulh".into()))
            .expect("to return a list of pull requests");
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
}
