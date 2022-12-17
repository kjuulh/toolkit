mod auth;
mod gh;

pub struct GitHub;

impl GitHub {
    fn run(external: &str, args: &clap::ArgMatches) -> eyre::Result<()> {
        let raw = args
            .get_many::<std::ffi::OsString>("")
            .ok_or(eyre::anyhow!("please pass some args to search"))?
            .map(|s| s.as_os_str())
            .map(|s| s.to_string_lossy().to_string())
            .collect::<Vec<String>>();
        let cmd = format!("src search {external} {}", raw.join(" "));
        println!("{cmd}");

        let mut cmd_args = vec!["gh", external];
        cmd_args.append(&mut raw.iter().map(|s| &**s).collect());

        util::shell::run(cmd_args.as_slice())?;

        Ok(())
    }
}

impl util::Cmd for GitHub {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("github")
            .subcommands(&[auth::Auth::cmd()?, gh::Gh::cmd()?])
            .allow_external_subcommands(true))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("auth", subm)) => auth::Auth::exec(subm),
            Some(("gh", subm)) => gh::Gh::exec(subm),
            Some((external, args)) => Self::run(external, args),
            _ => Err(eyre::anyhow!("missing argument")),
        }
    }
}
