use std::{borrow::Borrow, ffi::OsString};

pub struct Search;

impl util::Cmd for Search {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("search")
            .allow_external_subcommands(true)
            .allow_missing_positional(true))
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some((external, args)) => {
                let mut raw = args
                    .get_many::<OsString>("")
                    .ok_or(eyre::anyhow!("please pass some args to search"))?
                    .map(|s| s.as_os_str())
                    .map(|s| s.to_string_lossy().to_string())
                    .collect::<Vec<String>>();
                let cmd = format!("src search {external} {}", raw.join(" "));
                println!("{cmd}");

                let mut cmd_args = vec!["src", "search", external];
                cmd_args.append(&mut raw.iter().map(|s| &**s).collect());

                util::shell::run(cmd_args.as_slice())?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}
