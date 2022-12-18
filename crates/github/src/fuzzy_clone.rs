use std::io::Write;

use clap::value_parser;
use eyre::Context;

pub struct FuzzyClone;

#[derive(Debug)]
struct GitHubEntry {
    org: String,
    repo: String,
}

impl GitHubEntry {
    fn from(raw: String) -> Option<Self> {
        let (org, repo) = raw.split_once("/")?;

        Some(GitHubEntry {
            org: org.trim().to_string(),
            repo: repo.trim().to_string(),
        })
    }
}

struct Settings {
    orgs: Vec<String>,
    git_root: String,
    cache_dir: String,
    cache_file_path: String,
    auto_update: bool,
}

impl Settings {
    fn new() -> eyre::Result<Self> {
        let mut cache_dir =
            dirs::cache_dir().ok_or(eyre::anyhow!("could not find a valid cache dir"))?;
        cache_dir.push("kah-toolkit/github/fc");

        let mut file_path = std::path::Path::new(&cache_dir).to_path_buf();
        file_path.push("entries");

        Ok(Self {
            orgs: std::env::var("GITHUB_FC_ORGS")
                .context("GITHUB_FC_ORGS is not set")?
                .split(",")
                .map(|s| s.to_string())
                .collect(),
            git_root: std::env::var("GITHUB_FC_ROOT").context("GITHUB_FC_ROOT is not set")?,
            auto_update: std::env::var("GITHUB_FC_AUTO_UPDATE")
                .unwrap_or_else(|_| "false".into())
                .parse()
                .unwrap_or_default(),
            cache_dir: cache_dir.to_string_lossy().to_string(),
            cache_file_path: file_path.to_string_lossy().to_string(),
        })
    }
}

impl FuzzyClone {
    fn get_settings() -> eyre::Result<Settings> {
        Settings::new()
    }

    fn get_cache() -> eyre::Result<Vec<GitHubEntry>> {
        let settings = Self::get_settings()?;
        let entries = std::fs::read_to_string(settings.cache_file_path)?;

        Self::parse_entries(entries)
    }

    fn parse_entries(raw: String) -> eyre::Result<Vec<GitHubEntry>> {
        let entries = raw
            .replace("\r\n", "\n")
            .split("\n")
            .map(|s| {
                s.split_once("/").map(|(org, repo)| {
                    if let Some((repo, _)) = repo.split_once("\t") {
                        GitHubEntry {
                            org: org.to_string(),
                            repo: repo.to_string(),
                        }
                    } else {
                        GitHubEntry {
                            org: org.to_string(),
                            repo: repo.to_string(),
                        }
                    }
                })
            })
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .collect::<Vec<GitHubEntry>>();

        Ok(entries)
    }

    fn cache_exists() -> eyre::Result<bool> {
        let settings = Self::get_settings()?;
        let cf_path = std::path::Path::new(&settings.cache_file_path);
        if !cf_path.exists() {
            return Ok(false);
        }
        let metadata_time = cf_path.metadata()?.modified()?;
        // Update at least once a day
        let cur_time = std::time::SystemTime::now() - std::time::Duration::new(60 * 60 * 24, 0);

        if metadata_time < cur_time {
            return Ok(false);
        }

        Ok(true)
    }

    fn set_cache(input: &Vec<GitHubEntry>) -> eyre::Result<()> {
        let settings = Self::get_settings()?;
        let cache_file_path = std::path::Path::new(&settings.cache_file_path);
        let mut fs = if cache_file_path.exists() {
            std::fs::File::create(cache_file_path)?
        } else {
            let cache_dir_path = std::path::Path::new(&settings.cache_dir);
            if !cache_dir_path.exists() {
                std::fs::create_dir_all(cache_dir_path)?;
            }
            std::fs::File::create(cache_file_path)?
        };

        fs.write_all(
            input
                .iter()
                .map(|ge| format!("{}/{}", ge.org, ge.repo))
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )?;

        Ok(())
    }

    fn get_entries() -> eyre::Result<Vec<GitHubEntry>> {
        let mut entries = Vec::new();

        for org in Self::get_settings()?.orgs {
            let private_entires = util::shell::run_with_input_and_output(
                &[
                    "gh",
                    "repo",
                    "list",
                    &org,
                    "--visibility",
                    "private",
                    "--limit",
                    "1000",
                ],
                "".into(),
            )?;

            let public_entires = util::shell::run_with_input_and_output(
                &[
                    "gh",
                    "repo",
                    "list",
                    &org,
                    "--visibility",
                    "public",
                    "--limit",
                    "1000",
                ],
                "".into(),
            )?;

            let private = std::str::from_utf8(private_entires.stdout.as_slice())?;
            let public = std::str::from_utf8(public_entires.stdout.as_slice())?;
            let raw_entries = format!("{private}{public}");

            entries.push(Self::parse_entries(raw_entries)?);
        }

        Ok(entries.into_iter().flat_map(|s| s).collect())
    }

    fn clone(chosen: GitHubEntry) -> eyre::Result<std::path::PathBuf> {
        let mut git_path = std::path::Path::new(&Self::get_settings()?.git_root).to_path_buf();
        git_path.push(&chosen.org);
        if !git_path.exists() {
            std::fs::create_dir_all(&git_path)?;
        }
        let mut git_repo_path = git_path.clone();
        git_repo_path.push(&chosen.repo);
        if !git_repo_path.exists() {
            util::shell::run(
                &[
                    "gh",
                    "repo",
                    "clone",
                    format!("{}/{}", &chosen.org, &chosen.repo).as_str(),
                    git_repo_path
                        .to_str()
                        .ok_or(eyre::anyhow!("could you not transform to path"))?,
                ],
                Some(util::shell::RunOptions {
                    path: git_path.clone(),
                }),
            )?;
        } else {
            util::shell::run(
                &["git", "pull"],
                Some(util::shell::RunOptions {
                    path: git_repo_path.clone(),
                }),
            )?;
        }

        Ok(git_repo_path)
    }

    fn run(print_dest: &bool) -> eyre::Result<()> {
        let settings = Self::get_settings()?;
        if settings.auto_update {
            println!("running auto update");
            util::shell::run(
                &["nohup", "toolkit", "github", "fuzzy-clone", "update"],
                None,
            )?;
        }

        let entries = if !Self::cache_exists()? {
            let entries = Self::get_entries()?;
            Self::set_cache(&entries)?;
            entries
        } else {
            Self::get_cache()?
        };

        let entries_str = entries
            .iter()
            .map(|ge| format!("{}/{}", ge.org, ge.repo))
            .collect::<Vec<String>>()
            .join("\n");

        let chosen = util::shell::run_with_input_and_output(&["fzf"], entries_str)?;
        let chosen = std::str::from_utf8(&chosen.stdout)?;

        let path = Self::clone(GitHubEntry::from(chosen.to_string()).ok_or(eyre::anyhow!(
            "could not parse choice as github entry <org>/<repo>"
        ))?)?;

        if *print_dest {
            print!(
                "{}",
                path.to_str().ok_or(eyre::anyhow!("path was not found"))?
            );
        }

        Ok(())
    }

    fn update() -> eyre::Result<()> {
        println!("Updating...\nThis may take a while");
        let entries = Self::get_entries()?;
        Self::set_cache(&entries)?;

        Ok(())
    }
}

impl util::Cmd for FuzzyClone {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("fuzzy-clone")
            .alias("fc")
            .alias("c")
            .arg(
                clap::Arg::new("print-dest")
                    .long("print-dest")
                    .value_name("print-dest")
                    .value_parser(value_parser!(bool))
                    .num_args(0..=1)
                    .require_equals(true)
                    .default_missing_value("true"),
            )
            .subcommand(clap::Command::new("update")))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        let print_dest = args.get_one::<bool>("print-dest").unwrap_or(&false);

        match args.subcommand() {
            Some(("update", _)) => Self::update()?,
            _ => Self::run(print_dest)?,
        }

        Ok(())
    }
}
