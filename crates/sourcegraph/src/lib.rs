mod auth;
mod search;

pub struct Sourcegraph;

impl Sourcegraph {
    fn run() -> eyre::Result<()> {
        Ok(())
    }
}

impl util::Cmd for Sourcegraph {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("sourcegraph")
            .subcommands(&[auth::Auth::cmd()?, search::Search::cmd()?]))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("auth", subm)) => auth::Auth::exec(subm),
            Some(("search", subm)) => search::Search::exec(subm),
            _ => Self::run(),
        }
    }
}
