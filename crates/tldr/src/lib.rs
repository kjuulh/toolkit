pub(crate) mod update;

pub struct Tldr;

impl Tldr {
    fn run() -> eyre::Result<()> {
        let cache_dir =
            dirs::cache_dir().ok_or(eyre::anyhow!("could not find a valid cache dir"))?;

        let mut tldr_cache_dir = cache_dir.clone();
        tldr_cache_dir.push("kah-toolkit/tldr/store/");

        if !tldr_cache_dir.exists() {
            return Err(eyre::anyhow!("you need to run <toolkit tldr update> first"));
        }

        let mut tldr_pages_path = tldr_cache_dir.clone();
        tldr_pages_path.push("pages");

        if !tldr_pages_path.exists() {
            return Err(eyre::anyhow!("you need to run <toolkit tldr update> first"));
        }

        let mut entries: Vec<String> = Vec::new();
        for entry in walkdir::WalkDir::new(&tldr_pages_path) {
            let entry = entry?;

            let path = entry.path().to_path_buf();

            match path.extension() {
                None => continue,
                Some(ext) => {
                    if ext != "md" {
                        continue;
                    }
                }
            }

            let parent_str = path
                .parent()
                .ok_or(eyre::anyhow!("could not find parent for file"))?
                .to_string_lossy();

            let parent = parent_str.split("/").last();
            let file_name = entry.file_name();

            entries.push(format!(
                "{}/{}",
                parent.ok_or(eyre::anyhow!("path contains non ascii characters"))?,
                file_name
                    .to_str()
                    .ok_or(eyre::anyhow!("path contains non ascii characters"))?,
            ))
        }

        let paths = entries.join("\n");

        let output = util::shell::run_with_input_and_output(&["fzf"], paths)?;

        let choice = std::str::from_utf8(output.stdout.as_slice())?.trim();

        let mut tldr_choice_path = tldr_pages_path;
        tldr_choice_path.push(choice);

        let contents = std::fs::read_to_string(tldr_choice_path)?;

        util::shell::run_with_input(&["glow", "-"], contents)?;

        Ok(())
    }
}

impl util::Cmd for Tldr {
    fn cmd() -> eyre::Result<clap::Command> {
        let cmd = clap::Command::new("tldr").subcommands([update::Update::cmd()?]);

        Ok(cmd)
    }

    fn exec(args: &clap::ArgMatches) -> eyre::Result<()> {
        match args.subcommand() {
            Some(("update", subcmd)) => update::Update::exec(subcmd),
            _ => Tldr::run(),
        }
    }
}
