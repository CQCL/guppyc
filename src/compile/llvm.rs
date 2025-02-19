//! LLVM IR stage.

use std::{any, fs};

use hugr::llvm::CodegenExtsBuilder;
use hugr::llvm::custom::CodegenExtsMap;
use hugr::llvm::inkwell::context::Context;
use hugr::llvm::inkwell::memory_buffer::MemoryBuffer;
use hugr::llvm::inkwell::module::Module;
use hugr::llvm::utils::fat::FatExt;
use hugr::{Hugr, Node};

use crate::cli::CliArgs;

use super::{CompilationStage, GenericStage, Stage};

/// An llvm IR object.
#[derive(Debug)]
pub struct LLVMStage {
    /// `inkwell` Module containing the llvm IR.
    pub module_bitcode: MemoryBuffer,
    /// Text representation of the llvm IR.
    ///
    /// We have to pre-compute this because `inkwell` is quite restrictive with
    /// `Module` lifetimes and buffer ownership -.-
    pub module_text: Option<String>,
    /// Mangled name of the entrypoint function.
    pub entrypoint: String,
}

impl CompilationStage for LLVMStage {
    fn stage(&self) -> Stage {
        Stage::LLVM
    }

    fn wrap(self) -> super::GenericStage {
        super::GenericStage::LLVM(self)
    }

    fn compile(self, _args: &CliArgs) -> anyhow::Result<GenericStage> {
        anyhow::bail!("LLVM stage cannot be compiled further")
    }

    fn store(&self, args: &CliArgs) -> anyhow::Result<()> {
        if let Some(llvm) = &args.llvm {
            fs::write(llvm, self.module_text.as_ref().unwrap())?;
        }

        if let Some(bitcode) = &args.bitcode {
            fs::write(bitcode, self.module_bitcode.as_slice())?;
        }

        Ok(())
    }
}

impl LLVMStage {
    /// Lower a HUGR into LLVM.
    ///
    /// Assumes any hugr-side rewrites have already been done.
    pub fn from_hugr(hugr: Hugr, entrypoint: Node, args: &CliArgs) -> anyhow::Result<Self> {
        let namer = hugr::llvm::emit::Namer::default();
        let mangled_name = namer.name_func(&args.entrypoint, entrypoint);

        let context = Context::create();
        let module = compile_module(&hugr, &context, namer)?;
        let module_bitcode = module.write_bitcode_to_memory();
        let module_text = match args.llvm {
            Some(_) => Some(module.to_string()),
            None => None,
        };

        Ok(Self {
            module_bitcode,
            module_text,
            entrypoint: mangled_name,
        })
    }
}

fn compile_module<'a>(
    hugr: &Hugr,
    ctx: &'a Context,
    namer: hugr::llvm::emit::Namer,
) -> anyhow::Result<Module<'a>> {
    let llvm_module = ctx.create_module("guppy_llvm");
    // TODO: Handle tket2 codegen extension
    let extensions = codegen_extensions();

    let emitter =
        hugr::llvm::emit::EmitHugr::new(ctx, llvm_module, namer.into(), extensions.into());
    let hugr_module = hugr.fat_root().unwrap();
    let emitter = emitter.emit_module(hugr_module)?;

    Ok(emitter.finish())
}

fn codegen_extensions() -> CodegenExtsMap<'static, Hugr> {
    CodegenExtsBuilder::default()
        .add_default_prelude_extensions()
        .add_int_extensions()
        .add_float_extensions()
        .add_conversion_extensions()
        .add_logic_extensions()
        .add_default_array_extensions()
        .finish()
}
