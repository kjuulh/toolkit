use util::Cmd;

mod prereqs;

fn main() -> eyre::Result<()> {
    let matches = clap::Command::new("toolkit")
        .subcommands([
            prereqs::prereqs()?,
            tldr::Tldr::cmd()?,
            sourcegraph::Sourcegraph::cmd()?,
            github::GitHub::cmd()?,
            stats::Stats::cmd()?,
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("prereqs", subcmd)) => prereqs::prereqs_exec(subcmd),
        Some(("tldr", subcmd)) => tldr::Tldr::exec(subcmd),
        Some(("sourcegraph", subcmd)) => sourcegraph::Sourcegraph::exec(subcmd),
        Some(("github", subcmd)) => github::GitHub::exec(subcmd),
        Some(("stats", subcmd)) => stats::Stats::exec(subcmd),
        _ => Err(eyre::anyhow!("no command selected!")),
    }
}
