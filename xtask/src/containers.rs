use std::{
    ffi::{OsStr, OsString},
    process::{Command, Stdio},
};

use clap::Parser;
use target_lexicon::Triple;
use thiserror::Error;

use cross_llvm::{
    containers::ContainerEngine,
    target::{SupportedTriple, TripleExt},
};

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("containerized builds are not supported for target {0}")]
    UnsupportedTarget(String),
    #[error("failed to build a container image")]
    ContainerImageBuild,
    #[error("failed to push a container image")]
    ContainerImagePush,
}

#[derive(Parser)]
pub struct BuildContainerImageArgs {
    /// Container engine (if not provided, is going to be autodetected)
    #[arg(long)]
    container_engine: Option<ContainerEngine>,

    /// Do not use existing cached images for the container build. Build from
    /// the start with a new set of cached layers.
    #[arg(long)]
    no_cache: bool,

    /// Push the image after build.
    #[arg(long)]
    push: bool,

    /// Container image tag.
    #[arg(short, long = "tag", name = "tag")]
    tags: Vec<OsString>,

    /// Target triple (optional)
    #[arg(long)]
    target: Option<SupportedTriple>,
}

fn push_image(container_engine: &ContainerEngine, tag: &OsStr) -> anyhow::Result<()> {
    let mut cmd = Command::new(container_engine.to_string());
    cmd.args([OsStr::new("push"), tag])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    println!("{cmd:?}");
    if !cmd.status()?.success() {
        return Err(ContainerError::ContainerImagePush.into());
    }

    Ok(())
}

pub fn build_container_image(args: BuildContainerImageArgs) -> anyhow::Result<()> {
    let BuildContainerImageArgs {
        container_engine,
        no_cache,
        push,
        tags,
        target,
    } = args;

    let triple: Triple = match target {
        Some(target) => target.into(),
        None => target_lexicon::HOST,
    };

    let tags = if tags.is_empty() {
        vec![triple.default_container_tag()]
    } else {
        tags
    };

    match triple.dockerfile() {
        Some(dockerfile) => {
            let container_engine = container_engine.unwrap_or(ContainerEngine::autodetect()?);

            let mut cmd = Command::new(container_engine.to_string());
            cmd.args([OsStr::new("buildx"), OsStr::new("build")]);
            for tag in tags.iter() {
                cmd.args([OsStr::new("-t"), tag]);
            }
            cmd.args([OsStr::new("-f"), &dockerfile, OsStr::new(".")])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
            if no_cache {
                cmd.arg(OsStr::new("--no-cache"));
            }
            println!("{cmd:?}");
            if !cmd.status()?.success() {
                return Err(ContainerError::ContainerImageBuild.into());
            }

            if push {
                for tag in tags.iter() {
                    push_image(&container_engine, tag)?;
                }
            }

            Ok(())
        }
        None => Err(ContainerError::UnsupportedTarget(triple.to_string()).into()),
    }
}
