pub struct Storage;

impl Storage {
    fn run() -> eyre::Result<()> {
        if let Err(_) = util::shell::run_with_input_and_output(&["dust", "--version"], "".into()) {
            return Err(eyre::anyhow!(
                "could not find dust, please install or add to PATH"
            ));
        }

        util::shell::run(&["dust"], None)?;

        Ok(())
    }
}

impl util::Cmd for Storage {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("storage").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            _ => Storage::run(),
        }
    }
}
