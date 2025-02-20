//! LLVM IR stage.

use std::fs;

use hugr::llvm::CodegenExtsBuilder;
use hugr::llvm::custom::CodegenExtsMap;
use hugr::llvm::inkwell::context::Context;
use hugr::llvm::inkwell::memory_buffer::MemoryBuffer;
use hugr::llvm::inkwell::module::Module;
use hugr::llvm::inkwell::passes::PassManager;
use hugr::llvm::utils::fat::FatExt;
use hugr::{Hugr, Node};

use crate::cli::{CliArgs, OptimisationLevel};

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
    pub entrypoint: Option<String>,
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
        let out = &args.output;

        if let Some(llvm) = &out.llvm {
            fs::write(llvm, self.module_text.as_ref().unwrap())?;
        }

        if let Some(bitcode) = &out.bitcode {
            fs::write(bitcode, self.module_bitcode.as_slice())?;
        }

        Ok(())
    }
}

impl LLVMStage {
    /// Lower a HUGR into LLVM.
    ///
    /// Assumes any hugr-side rewrites have already been done.
    pub fn from_hugr(hugr: Hugr, entrypoint: Option<Node>, args: &CliArgs) -> anyhow::Result<Self> {
        let namer = hugr::llvm::emit::Namer::default();
        let mangled_name = entrypoint
            .map(|entrypoint| namer.name_func(args.entrypoint.as_ref().unwrap(), entrypoint));

        let context = Context::create();
        let module = compile_module(&hugr, &context, namer)?;
        optimise_module(&module, args)?;

        let module_bitcode = module.write_bitcode_to_memory();
        let module_text = match args.output.llvm {
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

// Run some standard optimisations on the module.
fn optimise_module<'a>(module: &Module<'a>, args: &CliArgs) -> anyhow::Result<()> {
    if args.opt == OptimisationLevel::O0 {
        return Ok(());
    }

    let pb = PassManager::create(());
    pb.add_promote_memory_to_register_pass();
    pb.add_scalar_repl_aggregates_pass();
    pb.add_cfg_simplification_pass();
    pb.add_aggressive_inst_combiner_pass();
    pb.add_aggressive_dce_pass();
    pb.run_on(module);

    Ok(())
}

fn codegen_extensions() -> CodegenExtsMap<'static, Hugr> {
    CodegenExtsBuilder::default()
        .add_default_prelude_extensions()
        .add_default_list_extensions()
        .add_default_array_extensions()
        .add_int_extensions()
        .add_float_extensions()
        .add_conversion_extensions()
        .add_logic_extensions()
        .finish()
}
