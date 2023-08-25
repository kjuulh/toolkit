pub struct Coffee;

impl Coffee {
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

impl util::Cmd for Coffee {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("coffee").allow_external_subcommands(true))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some((external, args)) => Self::run(external, args),
            _ => {
                util::shell::run(&["coffee"], None)?;

                Err(eyre::anyhow!("missing argument"))
            }
        }
    }
}
