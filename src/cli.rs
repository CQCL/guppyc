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
    /// The function name to use as entrypoint.
    #[clap(short, long)]
    pub entrypoint: Option<String>,
    /// Optimisation level.
    #[clap(short, long, default_value = "2", value_parser = OptimisationLevel::parse)]
    pub opt: OptimisationLevel,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
    /// Guppy language version to use.
    #[clap(flatten)]
    pub guppy_version: GuppyVersion,
    /// Output format options.
    #[clap(flatten)]
    pub output: OutputFormat,
}

/// Output format options
#[derive(Args, Debug, Clone)]
pub struct OutputFormat {
    /// Optional output path for the HUGR json.
    #[clap(long, help_heading = "Output artifacts")]
    pub hugr: Option<PathBuf>,
    /// Optional output path for the S-expression representation of the HUGR.
    #[clap(long, help_heading = "Output artifacts")]
    pub sexpr: Option<PathBuf>,
    /// Optional output path for the mermaid rendering of the HUGR.
    #[clap(short, long, help_heading = "Output artifacts")]
    pub mermaid: Option<PathBuf>,
    /// Output llvm text file.
    #[clap(short, long, help_heading = "Output artifacts")]
    pub llvm: Option<PathBuf>,
    /// Output the llvm bitcode file.
    #[clap(short, long, help_heading = "Output artifacts")]
    pub bitcode: Option<PathBuf>,
}

/// Argument to specify the guppy language version, either using semver or a git ref.
#[derive(Args, Debug, Default, Clone)]
pub struct GuppyVersion {
    /// The guppy version to use.
    /// Defaults to the latest published version.
    /// Incompatible with `guppy_git` and `guppy_ref`.
    #[clap(long, help_heading = "Guppy version")]
    pub guppy_version: Option<Version>,
    /// The git repository to fetch guppy from.
    /// Incompatible with `guppy_version`.
    #[clap(long, help_heading = "Guppy version")]
    pub guppy_git: Option<String>,
    /// The git commit or branch to use.
    /// Incompatible with `guppy_version`.
    #[clap(long, help_heading = "Guppy version")]
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
        let last = Stage::last_required(self);

        while stage.stage() < last {
            stage = stage.compile(self)?;
            stage.store(self)?;
        }

        Ok(())
    }

    /// Validate the CLI arguments.
    pub fn validate(&self) -> anyhow::Result<()> {
        self.guppy_version.validate()?;
        Ok(())
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
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "0" => Ok(Self::O0),
            "1" => Ok(Self::O1),
            "2" => Ok(Self::O2),
            "3" => Ok(Self::O3),
            _ => Err(format!("Invalid optimisation level: {s}")),
        }
    }
}
