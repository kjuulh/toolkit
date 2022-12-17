pub mod shell;

pub trait Cmd {
    fn cmd() -> eyre::Result<clap::Command>;
    fn exec(args: &clap::ArgMatches) -> eyre::Result<()>;
}

