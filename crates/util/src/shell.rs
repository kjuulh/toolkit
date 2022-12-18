use std::io::Write;

pub struct RunOptions {
    pub path: std::path::PathBuf,
}

pub fn run(args: &[&str], opts: Option<RunOptions>) -> eyre::Result<()> {
    let mut cmd = std::process::Command::new(
        args.first()
            .ok_or(eyre::anyhow!("could not find first arg"))?,
    );

    if let Some(opts) = opts {
        cmd.current_dir(opts.path);
    }

    let output = cmd
        .args(
            args.to_vec()
                .into_iter()
                .skip(1)
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::inherit())
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                Ok(())
            } else {
                Err(eyre::anyhow!(
                    "command failed with statuscode: {}",
                    o.status
                        .code()
                        .ok_or(eyre::anyhow!("could not get a status code from process"))?
                ))
            }
        }
        Err(e) => Err(eyre::anyhow!(e)),
    }
}
pub fn run_with_input(args: &[&str], input: String) -> eyre::Result<()> {
    let output = std::process::Command::new(
        args.first()
            .ok_or(eyre::anyhow!("could not find first arg"))?,
    )
    .args(
        args.to_vec()
            .into_iter()
            .skip(1)
            .collect::<Vec<&str>>()
            .as_slice(),
    )
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .stdin(std::process::Stdio::piped())
    .spawn();

    match output {
        Ok(mut o) => {
            let stdin = o
                .stdin
                .as_mut()
                .ok_or(eyre::anyhow!("could not acquire stdin"))?;

            stdin.write_all(input.as_bytes())?;
            drop(stdin);

            let o = o.wait_with_output()?;
            if o.status.success() {
                Ok(())
            } else {
                Err(eyre::anyhow!(
                    "command failed with statuscode: {}",
                    o.status
                        .code()
                        .ok_or(eyre::anyhow!("could not get a status code from process"))?
                ))
            }
        }
        Err(e) => Err(eyre::anyhow!(e)),
    }
}

pub fn run_with_input_and_output(
    args: &[&str],
    input: String,
) -> eyre::Result<std::process::Output> {
    let output = std::process::Command::new(
        args.first()
            .ok_or(eyre::anyhow!("could not find first arg"))?,
    )
    .args(
        args.to_vec()
            .into_iter()
            .skip(1)
            .collect::<Vec<&str>>()
            .as_slice(),
    )
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::inherit())
    .stdin(std::process::Stdio::piped())
    .spawn();

    match output {
        Ok(mut o) => {
            let stdin = o
                .stdin
                .as_mut()
                .ok_or(eyre::anyhow!("could not acquire stdin"))?;

            stdin.write_all(input.as_bytes())?;
            drop(stdin);

            let o = o.wait_with_output()?;
            if o.status.success() {
                Ok(o)
            } else {
                Err(eyre::anyhow!(
                    "command failed with statuscode: {}",
                    o.status
                        .code()
                        .ok_or(eyre::anyhow!("could not get a status code from process"))?
                ))
            }
        }
        Err(e) => Err(eyre::anyhow!(e)),
    }
}
