use std::{ffi::OsString, path::Path};

use clap::ValueEnum;
use target_lexicon::{
    Aarch64Architecture, Architecture, BinaryFormat, Environment, OperatingSystem,
    Riscv64Architecture, Triple, Vendor,
};

#[derive(Clone)]
pub enum SupportedTriple {
    Aarch64AppleDarwin,
    Aarch64UnknownLinuxGnu,
    Aarch64UnknownLinuxMusl,
    Riscv64GcUnknownLinuxGnu,
    X86_64AppleDarwin,
    X86_64UnknownLinuxGnu,
    X86_64UnknownLinuxMusl,
}

impl ValueEnum for SupportedTriple {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Aarch64AppleDarwin,
            Self::Aarch64UnknownLinuxGnu,
            Self::Aarch64UnknownLinuxMusl,
            Self::Riscv64GcUnknownLinuxGnu,
            Self::X86_64AppleDarwin,
            Self::X86_64UnknownLinuxGnu,
            Self::X86_64UnknownLinuxMusl,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Aarch64AppleDarwin => clap::builder::PossibleValue::new("aarch64-apple-darwin"),
            Self::Aarch64UnknownLinuxGnu => {
                clap::builder::PossibleValue::new("aarch64-unknown-linux-gnu")
            }
            Self::Aarch64UnknownLinuxMusl => {
                clap::builder::PossibleValue::new("aarch64-unknown-linux-musl")
            }
            Self::Riscv64GcUnknownLinuxGnu => {
                clap::builder::PossibleValue::new("riscv64gc-unknown-linux-gnu")
            }
            Self::X86_64AppleDarwin => clap::builder::PossibleValue::new("x86_64-apple-darwin"),
            Self::X86_64UnknownLinuxGnu => {
                clap::builder::PossibleValue::new("x86_64-unknown-linux-gnu")
            }
            Self::X86_64UnknownLinuxMusl => {
                clap::builder::PossibleValue::new("x86_64-unknown-linux-musl")
            }
        })
    }
}

impl From<SupportedTriple> for Triple {
    fn from(value: SupportedTriple) -> Self {
        match value {
            SupportedTriple::Aarch64AppleDarwin => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Apple,
                operating_system: OperatingSystem::Darwin,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            },
            SupportedTriple::Aarch64UnknownLinuxGnu => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Elf,
            },
            SupportedTriple::Aarch64UnknownLinuxMusl => Triple {
                architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Musl,
                binary_format: BinaryFormat::Elf,
            },
            SupportedTriple::Riscv64GcUnknownLinuxGnu => Triple {
                architecture: Architecture::Riscv64(Riscv64Architecture::Riscv64gc),
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Elf,
            },
            SupportedTriple::X86_64AppleDarwin => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Apple,
                operating_system: OperatingSystem::Darwin,
                environment: Environment::Unknown,
                binary_format: BinaryFormat::Macho,
            },
            SupportedTriple::X86_64UnknownLinuxGnu => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Gnu,
                binary_format: BinaryFormat::Elf,
            },
            SupportedTriple::X86_64UnknownLinuxMusl => Triple {
                architecture: Architecture::X86_64,
                vendor: Vendor::Unknown,
                operating_system: OperatingSystem::Linux,
                environment: Environment::Musl,
                binary_format: BinaryFormat::Elf,
            },
        }
    }
}

pub trait TripleExt {
    fn default_container_tag(&self) -> OsString;
    fn dockerfile(&self) -> Option<OsString>;
    fn is_cross(&self) -> bool;
}

impl TripleExt for Triple {
    fn default_container_tag(&self) -> OsString {
        let mut tag = OsString::from("ghcr.io/exein-io/cross-llvm/");
        let prefix = if self.is_cross() { "cross" } else { "native" };
        tag.push(prefix);
        tag.push("-");
        tag.push(self.to_string());

        tag
    }

    fn dockerfile(&self) -> Option<OsString> {
        let mut dockerfile = OsString::from("containers/Dockerfile.");
        let prefix = if self.is_cross() { "cross" } else { "native" };
        dockerfile.push(prefix);
        dockerfile.push("-");
        dockerfile.push(self.to_string());

        if Path::new(&dockerfile).exists() {
            Some(dockerfile)
        } else {
            None
        }
    }

    fn is_cross(&self) -> bool {
        self.architecture != target_lexicon::HOST.architecture
    }
}
