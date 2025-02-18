//! LLVM IR stage.

use std::path::PathBuf;

use super::{CompilationStage, GenericStage, Stage};

/// An llvm IR object.
#[derive(Debug, Clone)]
pub struct LLVMStage {
    /// Path to the .ll file.
    pub ll: PathBuf,
}

impl CompilationStage for LLVMStage {
    fn stage(&self) -> Stage {
        Stage::LLVM
    }

    fn wrap(self) -> super::GenericStage {
        super::GenericStage::LLVM(self)
    }

    fn compile(self) -> anyhow::Result<GenericStage> {
        todo!()
    }

    fn store(&self, _args: &crate::cli::CliArgs) -> anyhow::Result<()> {
        todo!()
    }
}
