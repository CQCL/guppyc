use std::path::PathBuf;

use clap::{Args, Parser, crate_version};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use semver::Version;

use crate::compile::guppy::GuppyStage;
use crate::compile::{CompilationStage, Stage};

/// CLI arguments.
#[derive(Parser, Debug)]
#[clap(version = crate_version!(), long_about = None)]
#[clap(about = "Guppy compilation tools.")]
#[non_exhaustive]
pub struct CliArgs {
    /// Input guppy file.
    pub input: PathBuf,
    /// Guppy language version to use.
    #[clap(flatten)]
    pub guppy_version: GuppyVersion,
    /// Output llvm text file.
    #[clap(short, long)]
    pub llvm: Option<PathBuf>,
    /// Output the llvm bitcode file.
    #[clap(long)]
    pub bitcode: Option<PathBuf>,
    /// Optional path to output the HUGR json.
    #[clap(long)]
    pub hugr: Option<PathBuf>,
    /// Optional path to output the mermaid rendering of the HUGR.
    #[clap(short, long)]
    pub mermaid: Option<PathBuf>,
    /// The function name to use as entrypoint.
    #[clap(short, long)]
    pub entrypoint: Option<String>,
    /// Optimisation level.
    #[clap(short, long, default_value = "2", value_parser = OptimisationLevel::from_str)]
    pub opt: OptimisationLevel,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}

/// Argument to specify the guppy language version, either using semver or a git ref.
#[derive(Args, Debug, Default, Clone)]
pub struct GuppyVersion {
    /// The guppy version to use.
    /// Defaults to the latest published version.
    /// Incompatible with `guppy_git` and `guppy_ref`.
    #[clap(long)]
    pub guppy_version: Option<Version>,
    /// The git repository to fetch guppy from.
    /// Incompatible with `guppy_version`.
    #[clap(long)]
    pub guppy_git: Option<String>,
    /// The git commit or branch to use.
    /// Incompatible with `guppy_version`.
    #[clap(long)]
    pub guppy_ref: Option<String>,
}

/// Optimisation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimisationLevel {
    /// No optimisation.
    O0 = 0,
    /// Less optimisation.
    O1 = 1,
    /// Default level of optimisation.
    O2 = 2,
    /// Aggressive optimisation.
    O3 = 3,
}

impl CliArgs {
    /// Run the CLI.
    pub fn run(&self) -> anyhow::Result<()> {
        self.validate()?;

        let mut stage = GuppyStage::new(&self.guppy_version, &self.input).wrap();
        let last = self.last_stage();

        while stage.stage() < last {
            stage = stage.compile(&self)?;
            stage.store(&self)?;
        }

        Ok(())
    }

    /// Validate the CLI arguments.
    pub fn validate(&self) -> anyhow::Result<()> {
        self.guppy_version.validate()?;
        Ok(())
    }

    /// Return the latest compilation stage required to produce the artifacts specified by the CLI arguments.
    pub fn last_stage(&self) -> Stage {
        if self.llvm.is_some() || self.bitcode.is_some() {
            Stage::LLVM
        } else if self.hugr.is_some() || self.mermaid.is_some() {
            Stage::Hugr
        } else {
            Stage::GuppyProgram
        }
    }
}

impl GuppyVersion {
    /// Check that no incompatible options are set.
    pub fn validate(&self) -> anyhow::Result<()> {
        let fixed_version = self.guppy_version.is_some();
        let git = self.guppy_git.is_some() || self.guppy_ref.is_some();
        if fixed_version && git {
            anyhow::bail!("Cannot specify both `guppy_version` and `guppy_git` or `guppy_ref`");
        }

        Ok(())
    }
}

impl OptimisationLevel {
    /// Parse an optimisation level from a string.
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "0" => Ok(Self::O0),
            "1" => Ok(Self::O1),
            "2" => Ok(Self::O2),
            "3" => Ok(Self::O3),
            _ => Err(format!("Invalid optimisation level: {s}")),
        }
    }
}
