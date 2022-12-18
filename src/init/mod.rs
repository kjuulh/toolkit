mod fish;

pub struct Init;

impl util::Cmd for Init {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("init")
            .subcommands(&[fish::Fish::cmd()?])
            .subcommand_required(true);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("fish", args)) => fish::Fish::exec(args),
            _ => Err(eyre::anyhow!("missing command!")),
        }
    }
}
