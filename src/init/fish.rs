pub struct Fish;

impl util::Cmd for Fish {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("fish").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        Ok(())
    }
}
