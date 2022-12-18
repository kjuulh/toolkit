use eyre::Context;

pub struct Code;

struct Settings {
    prefer_docker: bool,
}

impl Settings {
    fn new() -> Self {
        Self {
            prefer_docker: std::env::var("TOOLKIT_PREFER_DOCKER")
                .unwrap_or("false".into())
                .parse()
                .context("TOOLKIT_PREFER_DOCKER could not be parsed as a bool")
                .unwrap(),
        }
    }
}

impl Code {
    fn run() -> eyre::Result<()> {
        if Settings::new().prefer_docker {
            let current_dir = std::env::current_dir()?;
            let current_dir_str = current_dir
                .to_str()
                .ok_or(eyre::anyhow!("could not parse path as string"))?;
            util::shell::run(
                &[
                    "docker",
                    "run",
                    "-v",
                    &format!("{current_dir_str}:/mnt"),
                    "kasperhermansen/tokei:12.1-amd64",
                ],
                None,
            )?;
        } else {
            util::shell::run(&["tokei"], None)?;
        }

        Ok(())
    }
}

impl util::Cmd for Code {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("code").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            _ => Code::run(),
        }
    }
}
