mod code;
mod network;
mod procs;

pub struct Stats;

impl Stats {
    fn run() -> eyre::Result<()> {
        Ok(())
    }
}

impl util::Cmd for Stats {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("stats")
            .subcommands(&[
                code::Code::cmd()?,
                network::Network::cmd()?,
                procs::Procs::cmd()?,
            ])
            .subcommand_required(true);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("code", args)) => code::Code::exec(args),
            Some(("network", args)) => network::Network::exec(args),
            Some(("procs", args)) => procs::Procs::exec(args),
            _ => Stats::run(),
        }
    }
}
