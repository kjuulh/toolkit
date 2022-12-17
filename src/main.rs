use prereqs::prereqs_exec;
use util::Cmd;

mod prereqs;

fn main() -> eyre::Result<()> {
    let matches = clap::Command::new("toolkit")
        .subcommands([
            prereqs::prereqs()?,
            tldr::Tldr::cmd()?,
            sourcegraph::Sourcegraph::cmd()?,
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("prereqs", subcmd)) => prereqs_exec(subcmd),
        Some(("tldr", subcmd)) => tldr::Tldr::exec(subcmd),
        Some(("sourcegraph", subcmd)) => sourcegraph::Sourcegraph::exec(subcmd),
        _ => Err(eyre::anyhow!("no command selected!")),
    }
}
