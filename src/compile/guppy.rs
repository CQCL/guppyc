//! Guppy files. Main starting point for the guppy compilation.

use std::path::{Path, PathBuf};

use itertools::Itertools;

use crate::cli::{CliArgs, GuppyVersion};

use super::hugr::HugrStage;
use super::{CompilationStage, GenericStage, Stage};

const GUPPY_COMPILER_SCRIPT: &str = include_str!("../../script/compile_guppy.py");

/// A guppy file.
#[derive(Debug, Clone)]
pub struct GuppyStage {
    /// The guppy library version.
    pub version: GuppyVersion,
    /// The path to the guppy file (`.gpy` or `.py`)
    pub path: PathBuf,
}

impl CompilationStage for GuppyStage {
    fn stage(&self) -> Stage {
        Stage::GuppyProgram
    }

    fn wrap(self) -> GenericStage {
        GenericStage::GuppyProgram(self)
    }

    fn compile(self, _args: &CliArgs) -> anyhow::Result<GenericStage> {
        // Execute the guppy compilation script using uv to set the guppylang version.
        // This will output the HUGR json file.

        const UV: &str = "uv";
        let args = self.uv_args()?;

        // Run the script, capturing the output.
        log::info!("Running uv with args: {}", args.iter().join(", "));
        let output = std::process::Command::new(UV).args(args).output();

        let output = match output {
            Ok(out) => out,
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to execute uv. {e}"));
            }
        };

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to execute uv. Exit code: {}.\n{}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8(output.stdout)?;

        Ok(HugrStage::from_json(stdout)?.wrap())
    }

    fn store(&self, _args: &crate::cli::CliArgs) -> anyhow::Result<()> {
        // Nothing to store.
        Ok(())
    }
}

impl GuppyStage {
    /// Returns a new GuppyStage with the given version and path.
    pub fn new(guppy_version: &GuppyVersion, path: impl AsRef<Path>) -> Self {
        Self {
            version: guppy_version.clone(),
            path: PathBuf::from(path.as_ref()),
        }
    }

    /// Returns a new GuppyStage using the default guppy version and the given path.
    pub fn with_default_guppy(path: impl AsRef<Path>) -> Self {
        Self {
            version: GuppyVersion::default(),
            path: PathBuf::from(path.as_ref()),
        }
    }

    /// Returns the `uv` command arguments to execute the guppy compilation script.
    pub fn uv_args(&self) -> anyhow::Result<Vec<String>> {
        let args = vec![
            "run".to_string(),
            "--with".to_string(),
            self.version.uv_version()?,
            "python".to_string(),
            "-I".to_string(),
            guppy_compiler_script().to_string_lossy().to_string(),
            self.path.to_string_lossy().to_string(),
        ];
        Ok(args)
    }
}

impl GuppyVersion {
    /// Returns the version string to use with `uv`.
    pub fn uv_version(&self) -> anyhow::Result<String> {
        let version_str = match (&self.guppy_git, &self.guppy_ref, &self.guppy_version) {
            (Some(git), None, None) => format!("@git+{}", git),
            (None, Some(r), None) => format!("@git+https://github.com/cqcl/guppylang@{}", r),
            (Some(git), Some(r), None) => format!("@git+{}@{}", git, r),
            (None, None, Some(v)) => format!("=={}", v),
            (None, None, None) => "".to_string(),
            _ => {
                anyhow::bail!("Invalid guppy version options.");
            }
        };

        Ok(format!("guppylang{version_str}"))
    }
}

/// Returns the path to the guppy compiler script.
///
/// Writes the embedded guppy compiler script to a temporary file and returns the path to it.
pub fn guppy_compiler_script() -> PathBuf {
    let path = std::env::temp_dir().join("compile_guppy.py");
    log::info!("Writing guppy compiler script to {}", path.display());
    std::fs::write(&path, GUPPY_COMPILER_SCRIPT).expect("Failed to write guppy compiler script");
    path
}
