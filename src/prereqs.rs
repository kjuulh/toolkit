use clap::ArgMatches;

const DEPS: &[&str] = &["gh", "fzf", "sourcegraph-cli"];

// -- List
//

pub fn ls() -> clap::Command {
    clap::Command::new("ls")
}

pub fn ls_exec() -> eyre::Result<()> {
    println!("Required packages\n---");

    let mut deps: Vec<&str> = DEPS.into_iter().map(|d| d.clone()).collect();
    deps.sort();
    for dep in deps {
        println!("{}", dep)
    }
    Ok(())
}

// -- Prereqs
//

pub fn prereqs() -> eyre::Result<clap::Command> {
    let cmd = clap::Command::new("prereqs").subcommands([ls()]);

    Ok(cmd)
}

pub fn prereqs_exec(matches: &ArgMatches) -> eyre::Result<()> {
    match matches.subcommand() {
        Some(("ls", _)) => ls_exec(),
        _ => Err(eyre::anyhow!("not implemented")),
    }
}
