//! Compilation stage definitions.

use strum::IntoEnumIterator;

use crate::cli::CliArgs;

pub mod guppy;
pub mod hugr;
pub mod llvm;

/// Stages of the guppy compilation artifacts.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumIter, strum::Display,
)]
pub enum Stage {
    /// Guppy program definition.
    GuppyProgram = 0,
    /// Hugr program.
    Hugr = 1,
    /// LLVM IR.
    LLVM = 2,
}

/// Data for a compilation stage.
#[derive(Debug)]
pub enum GenericStage {
    /// Guppy program.
    GuppyProgram(guppy::GuppyStage),
    /// Hugr program.
    Hugr(hugr::HugrStage),
    /// LLVM IR.
    LLVM(llvm::LLVMStage),
    // TODO: Object and Executable stages.
}

pub trait CompilationStage: Sized {
    /// The stage for this data.
    fn stage(&self) -> Stage;

    /// Wrap the data in a `StageData`.
    fn wrap(self) -> GenericStage;

    /// Compile the object into the next stage.
    fn compile(self, args: &CliArgs) -> anyhow::Result<GenericStage>;

    /// Store any data that needs to be stored, according to the program arguments.
    fn store(&self, args: &CliArgs) -> anyhow::Result<()>;
}

impl Stage {
    /// Returns `true` if the stage is required to produce the specified artifacts.
    pub fn required(&self, args: &CliArgs) -> bool {
        let out = &args.output;
        match self {
            Stage::GuppyProgram => true,
            Stage::Hugr => out.hugr.is_some() || out.mermaid.is_some() || out.sexpr.is_some(),
            Stage::LLVM => out.llvm.is_some() || out.bitcode.is_some(),
        }
    }

    /// Return the latest compilation stage required to produce the artifacts specified by the CLI arguments.
    pub fn last_required(args: &CliArgs) -> Stage {
        for stage in Stage::iter().rev() {
            if stage.required(args) {
                return stage;
            }
        }
        Stage::GuppyProgram
    }
}

impl CompilationStage for GenericStage {
    fn stage(&self) -> Stage {
        match self {
            GenericStage::GuppyProgram { .. } => Stage::GuppyProgram,
            GenericStage::Hugr { .. } => Stage::Hugr,
            GenericStage::LLVM { .. } => Stage::LLVM,
        }
    }

    fn wrap(self) -> GenericStage {
        self
    }

    fn compile(self, args: &CliArgs) -> anyhow::Result<GenericStage> {
        match self {
            GenericStage::GuppyProgram(guppy) => guppy.compile(args),
            GenericStage::Hugr(hugr) => hugr.compile(args),
            GenericStage::LLVM(llvm) => llvm.compile(args),
        }
    }

    fn store(&self, args: &CliArgs) -> anyhow::Result<()> {
        match self {
            GenericStage::GuppyProgram(guppy) => guppy.store(args),
            GenericStage::Hugr(hugr) => hugr.store(args),
            GenericStage::LLVM(llvm) => llvm.store(args),
        }
    }
}
