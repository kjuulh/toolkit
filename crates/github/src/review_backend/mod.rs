pub mod models;

use std::io::Write;

use self::models::{MenuChoice, MergeStrategy, PullRequest, ReviewMenuChoice};

use comfy_table::{presets::UTF8_HORIZONTAL_ONLY, Cell, Table};
#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait ReviewBackend {
    fn get_prs(&self, review_request: Option<String>) -> eyre::Result<Vec<PullRequest>>;
    fn present_prs(&self, table: String) -> eyre::Result<()>;
    fn present_menu(&self) -> eyre::Result<MenuChoice>;
    fn present_diff(&self, pr: &PullRequest) -> eyre::Result<()>;
    fn present_review_menu(&self, pr: &PullRequest) -> eyre::Result<ReviewMenuChoice>;
    fn approve(&self, pr: &PullRequest) -> eyre::Result<()>;
    fn pr_open_browser(&self, pr: &PullRequest) -> eyre::Result<()>;
    fn clear(&self) -> eyre::Result<()>;
    fn enable_auto_merge(
        &self,
        pr: &PullRequest,
        merge_strategy: &Option<MergeStrategy>,
    ) -> eyre::Result<()>;
    fn present_pr(&self, pr: &PullRequest) -> eyre::Result<()>;
    fn present_status_checks(&self, pr: &PullRequest) -> eyre::Result<()>;
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
                review_request.unwrap_or("@me".into()).as_str(),
                "--label",
                "dependencies",
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
        println!("{table}");
        Ok(())
    }

    fn present_menu(&self) -> eyre::Result<MenuChoice> {
        println!("Menu");
        println!("Begin (b), Exit (q),  Menu (m), Search (s), List (l)");
        print!("> ");
        std::io::stdout().flush()?;

        let mut raw_choice = String::new();
        std::io::stdin().read_line(&mut raw_choice)?;
        let choice = match raw_choice.chars().take(1).next() {
            None => models::MenuChoice::Exit,
            Some(raw_choice) => match raw_choice {
                'b' => models::MenuChoice::Begin,
                'q' => models::MenuChoice::Exit,
                'm' => self.present_menu()?,
                's' => models::MenuChoice::Search,
                'l' => models::MenuChoice::List,
                _ => self.present_menu()?,
            },
        };

        Ok(choice)
    }

    fn present_diff(&self, pr: &PullRequest) -> eyre::Result<()> {
        util::shell::run(
            &[
                "gh",
                "pr",
                "diff",
                pr.number.to_string().as_str(),
                "--repo",
                pr.repository.name.as_str(),
            ],
            None,
        )?;

        Ok(())
    }

    fn present_review_menu(&self, pr: &PullRequest) -> eyre::Result<ReviewMenuChoice> {
        println!("");
        println!("Review - Menu");
        println!("Approve (a), Merge (m), Approve and auto-merge (c), Diff (d), Status Checks (sc), Skip (s), List (l), Open in browser (o), Exit (q)");
        print!("> ");
        std::io::stdout().flush()?;

        let mut raw_choice = String::new();
        std::io::stdin().read_line(&mut raw_choice)?;
        let choice = match raw_choice.trim() {
            "q" => ReviewMenuChoice::Exit,
            "l" => ReviewMenuChoice::List,
            "a" => ReviewMenuChoice::Approve,
            "o" => ReviewMenuChoice::Open,
            "s" | "n" => ReviewMenuChoice::Skip,
            "m" => ReviewMenuChoice::Merge,
            "c" => ReviewMenuChoice::ApproveAndMerge,
            "d" => ReviewMenuChoice::Diff,
            "sc" => ReviewMenuChoice::StatusChecks,
            _ => self.present_review_menu(pr)?,
        };

        Ok(choice)
    }

    fn approve(&self, pr: &PullRequest) -> eyre::Result<()> {
        util::shell::run(
            &[
                "gh",
                "pr",
                "review",
                pr.number.to_string().as_str(),
                "--approve",
                "--repo",
                pr.repository.name.as_str(),
            ],
            None,
        )?;

        Ok(())
    }

    fn pr_open_browser(&self, pr: &PullRequest) -> eyre::Result<()> {
        util::shell::run(
            &[
                "gh",
                "pr",
                "view",
                pr.number.to_string().as_str(),
                "-w",
                "--repo",
                pr.repository.name.as_str(),
            ],
            None,
        )?;

        Ok(())
    }

    fn clear(&self) -> eyre::Result<()> {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        std::io::stdout().flush()?;

        Ok(())
    }

    fn enable_auto_merge(
        &self,
        pr: &PullRequest,
        merge_strategy: &Option<MergeStrategy>,
    ) -> eyre::Result<()> {
        let number = pr.number.to_string();
        let mut args = vec![
            "gh",
            "pr",
            "merge",
            number.as_str(),
            "--auto",
            "--repo",
            pr.repository.name.as_str(),
        ];

        if let Some(merge_strategy) = merge_strategy {
            match merge_strategy {
                MergeStrategy::Squash => args.push("--squash"),
                MergeStrategy::MergeCommit => args.push("--merge"),
            }
        }

        util::shell::run(args.as_slice(), None)?;

        Ok(())
    }

    fn present_pr(&self, pr: &PullRequest) -> eyre::Result<()> {
        println!();
        println!("---");
        println!("repo: {} - title: {}", pr.repository.name, pr.title);

        Ok(())
    }

    fn present_status_checks(&self, pr: &PullRequest) -> eyre::Result<()> {
        let output = util::shell::run_with_input_and_output(
            &[
                "gh",
                "pr",
                "view",
                pr.number.to_string().as_str(),
                "--json",
                "statusCheckRollup",
                "--repo",
                pr.repository.name.as_str(),
            ],
            "".into(),
        )?;

        let parsed_output = std::str::from_utf8(&output.stdout)?;
        let checks: models::StatusChecks = serde_json::from_str(parsed_output)?;

        let mut table = Table::new();
        let table = table
            .load_preset(UTF8_HORIZONTAL_ONLY)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("status").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("conclusion").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("url").add_attribute(comfy_table::Attribute::Bold),
            ])
            .add_rows(checks.checks.iter().take(20).map(|pr| {
                let pr = pr.clone();
                match pr {
                    models::StatusCheck::CheckRun(check) => vec![
                        Cell::new(check.name).fg(comfy_table::Color::Green),
                        Cell::new(check.status),
                        Cell::new(check.conclusion),
                        Cell::new(check.details_url),
                    ],
                    models::StatusCheck::StatusContext(check) => vec![
                        Cell::new(check.context).fg(comfy_table::Color::Green),
                        Cell::new(""),
                        Cell::new(check.state),
                        Cell::new(check.target_url),
                    ],
                }
            }));

        println!("{}", table.to_string());

        Ok(())
    }
}

impl DefaultReviewBackend {
    pub fn new() -> Self {
        Self {}
    }
}
