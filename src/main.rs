use util::Cmd;

mod init;
mod prereqs;

fn main() -> eyre::Result<()> {
    color_eyre::install().unwrap();

    let matches = clap::Command::new("toolkit")
        .subcommands([
            prereqs::prereqs()?,
            tldr::Tldr::cmd()?,
            sourcegraph::Sourcegraph::cmd()?,
            github::GitHub::cmd()?,
            gitea::Gitea::cmd()?,
            stats::Stats::cmd()?,
            init::Init::cmd()?,
        ])
        .subcommand_required(true)
        .get_matches();

    match matches.subcommand() {
        Some(("prereqs", subcmd)) => prereqs::prereqs_exec(subcmd),
        Some(("tldr", subcmd)) => tldr::Tldr::exec(subcmd),
        Some(("sourcegraph", subcmd)) => sourcegraph::Sourcegraph::exec(subcmd),
        Some(("github", subcmd)) => github::GitHub::exec(subcmd),
        Some(("gitea", subcmd)) => gitea::Gitea::exec(subcmd),
        Some(("stats", subcmd)) => stats::Stats::exec(subcmd),
        Some(("init", subcmd)) => init::Init::exec(subcmd),
        _ => Err(eyre::anyhow!("no command selected!")),
    }
}
