//! Hugr IR stage.

use std::fs;

use hugr::HugrView;
use hugr::package::Package;

use super::{CompilationStage, GenericStage, Stage};

/// A hugr IR object.
#[derive(Debug, Clone)]
pub struct HugrStage {
    pub pkg: Package,
}
impl HugrStage {
    /// Load a HugrStage from a JSON string.
    pub fn from_json(json: impl AsRef<str>) -> anyhow::Result<Self> {
        let pkg = Package::from_json(json, &Default::default())?;
        Ok(Self { pkg })
    }

    /// Load a HugrStage from a JSON file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let pkg = Package::from_json_file(path, &Default::default())?;
        Ok(Self { pkg })
    }
}

impl CompilationStage for HugrStage {
    fn stage(&self) -> Stage {
        Stage::Hugr
    }

    fn wrap(self) -> super::GenericStage {
        super::GenericStage::Hugr(self)
    }

    fn compile(self) -> anyhow::Result<GenericStage> {
        todo!()
    }

    fn store(&self, args: &crate::cli::CliArgs) -> anyhow::Result<()> {
        if let Some(mermaid_out) = &args.mermaid {
            let mermaid = self.pkg.modules[0].mermaid_string();
            fs::write(mermaid_out, mermaid)?;
        }

        if let Some(hugr_out) = &args.hugr {
            self.pkg.to_json_file(hugr_out)?;
        }

        Ok(())
    }
}
