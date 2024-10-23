use std::{
    env,
    ffi::{OsStr, OsString},
    process::{Command, Stdio},
};

use clap::Parser;
use target_lexicon::Triple;

use crate::{
    containers::ContainerEngine,
    target::{SupportedTriple, TripleExt},
};

#[derive(Parser)]
pub struct Run {
    /// Container engine (if not provided, is going to be autodetected)
    #[arg(long)]
    container_engine: Option<ContainerEngine>,

    /// Container image to use.
    #[arg(long)]
    container_image: Option<OsString>,

    /// The command to run inside the container.
    #[arg(trailing_var_arg = true)]
    cmd: Vec<String>,

    /// Target triple (optional)
    #[arg(long)]
    target: Option<SupportedTriple>,
}

pub fn run(args: Run) -> anyhow::Result<()> {
    let Run {
        container_engine,
        container_image,
        cmd,
        target,
    } = args;

    let triple: Triple = match target {
        Some(target) => target.into(),
        None => target_lexicon::HOST,
    };

    let container_engine = container_engine.unwrap_or(ContainerEngine::autodetect()?);
    let container_image = container_image.unwrap_or(triple.default_container_tag());

    let mut bind_mount = env::current_dir()?.into_os_string();
    bind_mount.push(":/src");

    let mut container = Command::new(container_engine.to_string());
    container
        .args([
            OsStr::new("run"),
            OsStr::new("--rm"),
            OsStr::new("-it"),
            OsStr::new("-v"),
            &bind_mount,
            OsStr::new("-w"),
            OsStr::new("/src"),
            &container_image,
        ])
        .args(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    println!("{container:?}");

    let mut child = container.spawn()?;
    child.wait()?;

    Ok(())
}
