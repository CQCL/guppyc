//! Hugr IR stage.

use derive_more::{Display, Error};
use hugr::package::Package;
use hugr::{Hugr, HugrView, Node, ops};
use std::{fs, io, mem};

use crate::cli::CliArgs;

use super::llvm::LLVMStage;
use super::{CompilationStage, GenericStage, Stage};

/// A hugr IR object.
#[derive(Debug, Clone)]
pub struct HugrStage {
    pub pkg: Package,
}

impl CompilationStage for HugrStage {
    fn stage(&self) -> Stage {
        Stage::Hugr
    }

    fn wrap(self) -> super::GenericStage {
        super::GenericStage::Hugr(self)
    }

    fn compile(mut self, args: &CliArgs) -> anyhow::Result<GenericStage> {
        log::debug!("Compiling Hugr to LLVM IR");
        let entrypoint = match &args.entrypoint {
            Some(fn_name) => Some(self.find_funcdef_node(fn_name)?),
            None => None,
        };
        self.guppy_pass(entrypoint)?;
        let hugr = mem::take(&mut self.pkg.modules[0]);
        Ok(LLVMStage::from_hugr(hugr, entrypoint, args)?.wrap())
    }

    fn store(&self, args: &crate::cli::CliArgs) -> anyhow::Result<()> {
        let out = &args.output;

        if let Some(mermaid_out) = &out.mermaid {
            log::debug!("Storing mermaid output to {}", mermaid_out.display());
            let mermaid = self.pkg.modules[0].mermaid_string();
            fs::write(mermaid_out, mermaid)?;
        }

        if let Some(hugr_out) = &out.hugr {
            log::debug!("Storing Hugr output to {}", hugr_out.display());
            let file = fs::File::create(hugr_out)?;
            let writer = io::BufWriter::new(file);
            self.pkg.to_json_writer(writer)?;
        }

        if let Some(hugr_sexpr_out) = &out.sexpr {
            log::debug!(
                "Storing Hugr S-expression output to {}",
                hugr_sexpr_out.display()
            );
            let bump = bumpalo::Bump::new();
            let model = hugr_core::export::export_hugr(&self.pkg.modules[0], &bump);
            let sexpr = hugr_model::v0::text::print_to_string(&model, 120)?;
            fs::write(hugr_sexpr_out, sexpr)?;
        }

        Ok(())
    }
}

impl HugrStage {
    /// Load a HugrStage from a JSON string.
    pub fn from_json(json: impl AsRef<str>) -> anyhow::Result<Self> {
        let pkg = Package::from_json(json, &hugr::std_extensions::std_reg())?;
        Ok(Self { pkg })
    }

    /// Load a HugrStage from a JSON file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let pkg = Package::from_json_file(path, &hugr::std_extensions::std_reg())?;
        Ok(Self { pkg })
    }

    /// Reference to the [Hugr] in the package.
    pub fn hugr(&self) -> &Hugr {
        &self.pkg.modules[0]
    }

    /// Mutable reference to the [Hugr] in the package.
    pub fn hugr_mut(&mut self) -> &mut Hugr {
        &mut self.pkg.modules[0]
    }

    // Find the FuncDefn node for the function we're trying to execute in the package.
    fn find_funcdef_node(&self, fn_name: &str) -> Result<Node, HugrToLlvmError> {
        let root = self.hugr().root();
        let mut fn_nodes = Vec::new();

        // Return the function name of an operation, if it is a FuncDefn.
        fn get_fn_name(op: &ops::OpType) -> Option<&str> {
            match op {
                ops::OpType::FuncDefn(ops::FuncDefn { name, .. }) => Some(name),
                _ => None,
            }
        }

        for n in self.hugr().children(root) {
            let op = self.hugr().get_optype(n);
            if get_fn_name(op) == Some(fn_name) {
                fn_nodes.push(n);
            }
        }

        if fn_nodes.is_empty() {
            let available = self
                .hugr()
                .children(root)
                .filter_map(|n| get_fn_name(self.hugr().get_optype(n)))
                .map(ToString::to_string)
                .collect();
            return Err(HugrToLlvmError::MissingFunction {
                fn_name: fn_name.to_string(),
                available,
            });
        }
        if fn_nodes.len() > 1 {
            return Err(HugrToLlvmError::MultipleFunctions {
                fn_name: fn_name.to_string(),
            });
        }
        Ok(fn_nodes[0])
    }

    fn guppy_pass(&mut self, entrypoint: Option<Node>) -> anyhow::Result<()> {
        hugr::algorithms::MonomorphizePass::default().run(self.hugr_mut())?;
        if let Some(entrypoint) = entrypoint {
            hugr::algorithms::RemoveDeadFuncsPass::default()
                .with_module_entry_points([entrypoint])
                .run(self.hugr_mut())?
        }
        Ok(())
    }
}

/// Hugr to LLVM compilation error.
#[derive(Debug, Display, Error)]
pub enum HugrToLlvmError {
    /// The HUGR does not contain the function we're trying to compile.
    #[display(
        "Cannot find function {fn_name} in the Hugr package. Available functions: {available:?}"
    )]
    MissingFunction {
        /// The function name we were trying to compile.
        fn_name: String,
        /// The available function names.
        available: Vec<String>,
    },
    /// The HUGR contains multiple functions with the same name.
    #[display("Multiple functions with the name {fn_name} found in the Hugr package.")]
    MultipleFunctions { fn_name: String },
}
