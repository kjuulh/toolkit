pub struct Procs;

impl Procs {
    fn run() -> eyre::Result<()> {
        if let Err(_) = util::shell::run_with_input_and_output(&["procs", "--version"], "".into()) {
            return Err(eyre::anyhow!(
                "could not find procs, please install or add to PATH"
            ));
        }

        util::shell::run_with_input(&["procs"], "".into())?;

        Ok(())
    }
}

impl util::Cmd for Procs {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("procs").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            _ => Procs::run(),
        }
    }
}
