use std::path::PathBuf;

use clap::{Args, Parser, ValueEnum, crate_version};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use hugr::package::Package;
use semver::Version;

use crate::compile::guppy::GuppyStage;
use crate::compile::hugr::HugrStage;
use crate::compile::{CompilationStage, GenericStage, Stage};

/// CLI arguments.
#[derive(Parser, Debug)]
#[clap(version = crate_version!(), long_about = None)]
#[clap(about = "Guppy compilation tools.")]
#[non_exhaustive]
pub struct CliArgs {
    /// Input file.
    #[clap(flatten)]
    pub input: InputFile,
    /// Function name to use as entrypoint.
    #[clap(short, long)]
    pub entrypoint: Option<String>,
    /// Optimisation level.
    #[clap(short, long, default_value = "2")]
    pub opt: OptimisationLevel,
    /// Verbosity level.
    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
    /// Output format options.
    #[clap(flatten)]
    pub output: OutputFormat,
    /// Guppy language version to use.
    #[clap(flatten)]
    pub guppy_version: GuppyVersion,
}

/// Input format options
#[derive(Args, Debug, Clone)]
#[group(multiple = false, required = true)]
pub struct InputFile {
    /// A guppy program definition.
    #[clap(name = "input", help_heading = "Input format")]
    pub guppy_input: Option<PathBuf>,
    /// A `.hugr` file.
    #[clap(long, help_heading = "Input format")]
    pub hugr_input: Option<PathBuf>,
}

/// Output format options
#[derive(Args, Debug, Clone)]
pub struct OutputFormat {
    /// Store the intermediate HUGR as json.
    #[clap(long, help_heading = "Output artifacts")]
    pub hugr: Option<PathBuf>,
    /// Store the intermediate HUGR as an S-expression.
    #[clap(long, help_heading = "Output artifacts")]
    pub sexpr: Option<PathBuf>,
    /// Store the mermaid diagram for the HUGR.
    #[clap(short, long, help_heading = "Output artifacts")]
    pub mermaid: Option<PathBuf>,
    /// LLVM IR (text) output
    #[clap(short, long, help_heading = "Output artifacts")]
    pub llvm: Option<PathBuf>,
    /// LLVM Bitcode output
    #[clap(short, long, help_heading = "Output artifacts")]
    pub bitcode: Option<PathBuf>,
}

/// Argument to specify the guppy language version, either using semver or a git ref.
#[derive(Args, Debug, Default, Clone)]
pub struct GuppyVersion {
    /// The guppy version to use.
    ///
    /// Defaults to the latest published version.
    /// Incompatible with `guppy_git` and `guppy_ref`.
    #[clap(long, help_heading = "Guppy version")]
    pub guppy_version: Option<Version>,
    /// The git repository to fetch guppy from.
    ///
    /// Incompatible with `guppy_version`.
    #[clap(long, help_heading = "Guppy version")]
    pub guppy_git: Option<String>,
    /// The git commit or branch to use.
    ///
    /// Incompatible with `guppy_version`.
    #[clap(long, help_heading = "Guppy version")]
    pub guppy_ref: Option<String>,
}

/// Optimisation level.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptimisationLevel {
    /// No optimisation.
    #[clap(name = "0")]
    O0 = 0,
    /// Less optimisation.
    #[clap(name = "1")]
    O1 = 1,
    /// Default level of optimisation.
    #[clap(name = "2")]
    O2 = 2,
    /// Aggressive optimisation.
    #[clap(name = "3")]
    O3 = 3,
}

impl CliArgs {
    /// Run the CLI.
    pub fn run(&self) -> anyhow::Result<()> {
        self.validate()?;

        let mut stage = self.init_stage()?;
        let last = Stage::last_required(self);

        stage.store(self)?;
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

    /// Returns the initial stage based on the input file.
    pub fn init_stage(&self) -> anyhow::Result<GenericStage> {
        if let Some(guppy_input) = &self.input.guppy_input {
            Ok(GuppyStage::new(&self.guppy_version, guppy_input).wrap())
        } else if let Some(hugr_input) = &self.input.hugr_input {
            let pkg = Package::from_json_file(hugr_input, &hugr::std_extensions::std_reg())?;
            Ok(HugrStage { pkg }.wrap())
        } else {
            anyhow::bail!("No input file specified")
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
