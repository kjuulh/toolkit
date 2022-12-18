pub struct Update;

impl util::Cmd for Update {
    fn cmd() -> eyre::Result<clap::Command> {
        Ok(clap::Command::new("update"))
    }

    fn exec(_: &clap::ArgMatches) -> eyre::Result<()> {
        let cache_dir =
            dirs::cache_dir().ok_or(eyre::anyhow!("could not find a valid cache dir"))?;

        let mut tldr_cache_dir = cache_dir.clone();
        tldr_cache_dir.push("kah-toolkit/tldr/store/");

        std::fs::remove_dir_all(&tldr_cache_dir)?;
        std::fs::create_dir_all(&tldr_cache_dir)?;

        util::shell::run(
            format!(
                "gh repo clone tldr-pages/tldr {}",
                &tldr_cache_dir
                    .to_str()
                    .ok_or(eyre::anyhow!("pathstring contains non ascii-characters"))?
            )
            .split(" ")
            .collect::<Vec<&str>>()
            .as_slice(),
            None,
        )?;

        Ok(())
    }
}
