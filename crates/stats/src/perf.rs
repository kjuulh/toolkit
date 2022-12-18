pub struct Perf;

impl Perf {
    fn run() -> eyre::Result<()> {
        if let Err(_) = util::shell::run_with_input_and_output(&["ytop", "--version"], "".into()) {
            return Err(eyre::anyhow!(
                "could not find ytop, please install or add to PATH"
            ));
        }

        util::shell::run(&["ytop"], None)?;

        Ok(())
    }
}

impl util::Cmd for Perf {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("perf").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            _ => Perf::run(),
        }
    }
}