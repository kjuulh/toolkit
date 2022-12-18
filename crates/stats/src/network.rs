use eyre::Context;

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

pub struct Network;

impl Network {
    fn run() -> eyre::Result<()> {
        if Settings::new().prefer_docker {
            // let current_dir = std::env::current_dir()?;
            // let current_dir_str = current_dir
            //     .to_str()
            //     .ok_or(eyre::anyhow!("could not parse path as string"))?;
            //util::shell::run(
            //    &[
            //        "docker",
            //        "run",
            //        "-v",
            //        &format!("{current_dir_str}:/mnt"),
            //        "kasperhermansen/tokei:12.1-amd64",
            //    ],
            //    None,
            //)?;
        } else {
        }
        if let Err(_) =
            util::shell::run_with_input_and_output(&["bandwhich", "--version"], "".into())
        {
            return Err(eyre::anyhow!(
                "could not find bandwhich, please install or add to PATH"
            ));
        }

        util::shell::run(&["bandwhich"], None)?;

        Ok(())
    }
}

impl util::Cmd for Network {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("network").subcommands(&[]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            _ => Network::run(),
        }
    }
}
