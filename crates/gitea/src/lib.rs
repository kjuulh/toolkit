mod auth;
mod coffee;
mod fuzzy_clone;
mod review;
pub(crate) mod review_backend;

pub struct Gitea;

impl Gitea {
    fn run(external: &str, args: &clap::ArgMatches) -> eyre::Result<()> {
        let raw = args
            .get_many::<std::ffi::OsString>("")
            .ok_or(eyre::anyhow!("please pass some args to search"))?
            .map(|s| s.as_os_str())
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<String>>();
        let mut cmd_args = vec!["coffee", external];
        cmd_args.append(&mut raw.iter().map(|s| &**s).collect());

        util::shell::run(cmd_args.as_slice(), None)?;

        Ok(())
    }
}

impl util::Cmd for Gitea {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("gitea")
            .subcommands(&[
                auth::Auth::cmd()?,
                coffee::Coffee::cmd()?,
                fuzzy_clone::FuzzyClone::cmd()?,
                review::Review::cmd()?,
            ])
            .allow_external_subcommands(true))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("auth", subm)) => auth::Auth::exec(subm),
            Some(("fuzzy-clone", subm)) => fuzzy_clone::FuzzyClone::exec(subm),
            Some(("fc", subm)) => fuzzy_clone::FuzzyClone::exec(subm),
            Some(("coffee", subm)) => coffee::Coffee::exec(subm),
            //Some(("review", subm)) => review::Review::exec(subm),
            Some((external, args)) => Self::run(external, args),
            _ => Err(eyre::anyhow!("missing argument")),
        }
    }
}
