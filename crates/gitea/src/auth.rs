pub struct Auth;

impl util::Cmd for Auth {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("auth"))
    }

    fn exec(_: &clap::ArgMatches) -> eyre::Result<()> {
        util::shell::run(&["coffee", "auth", "login"], None)?;

        Ok(())
    }
}
