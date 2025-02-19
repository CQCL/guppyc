"""Methods for compiling Guppy programs into HUGRs."""

import importlib.machinery
import types
from pathlib import Path
import sys

try:
    from guppylang.module import GuppyModule
    from guppylang import guppy
except ImportError as e:
    raise RuntimeError("The `guppylang` python dependency is not installed.") from e

MINIMUM_GUPPY_VERSION: str = "0.14.0"


class GuppyCompiler:
    """A processor for compiling Guppy programs into Hugrs."""

    def compile_guppy(  # noqa: PLR0913
        self,
        *,
        input_path: Path,
        temp_file: bool = False,
        module_name: str | None = None,
    ) -> str:
        """Load a Guppy file as a Python module, and return it."""
        loader = importlib.machinery.SourceFileLoader("module", str(input_path))
        py_module = types.ModuleType(loader.name)
        try:
            loader.exec_module(py_module)
        except FileNotFoundError as err:
            raise InvalidGuppyModulePathError(input_path) from err

        module = self._get_module(
            py_module,
            input_path if not temp_file else None,
            module_name=module_name,
        )
        pkg = module.compile().package
        return pkg.to_json()

    def _get_module(
        self,
        py_module: types.ModuleType,
        source_path: Path | None,
        module_name: str | None = None,
    ) -> GuppyModule:
        if module_name is not None:
            if module_name not in py_module.__dir__():
                raise MissingModuleError(module_name, source_path)
            module = getattr(py_module, module_name)
        else:
            for module_id in guppy.registered_modules():
                if module_id.module == py_module or module_id.filename == source_path:
                    module = guppy.get_module(module_id)
                    break
            else:
                raise MissingModuleError(None, source_path)

        if not isinstance(module, GuppyModule):
            assert module_name is not None
            raise NotAGuppyError(source_path)

        return module


class GuppyCompilerError(Exception):
    """Base class for Guppy compiler errors."""


class OldGuppyVersion(GuppyCompilerError):
    """The guppy dependency is not installed."""

    def __init__(self, version: str) -> None:
        """Initialize the error."""
        super().__init__(
            f"`guppylang@{version}` is not supported. Please upgrade to `guppylang@{MINIMUM_GUPPY_VERSION}` or later."
        )


class InvalidGuppyModulePathError(GuppyCompilerError):
    """Raised when a Guppy program path is invalid."""

    def __init__(self, guppy: Path) -> None:
        """Initialize the error."""
        super().__init__(f"Invalid Guppy module path '{guppy}'.")


class MissingModuleError(GuppyCompilerError):
    """Raised when a Guppy program cannot be loaded."""

    def __init__(self, module: str | None, guppy: Path | None) -> None:
        """Initialize the error."""
        if module is None:
            super().__init__("The Guppy program does not define a local module.")
        elif guppy is None:
            super().__init__(f"The Guppy program does not define a `{module}` module.")
        else:
            super().__init__(
                f"The Guppy program {guppy} does not define a `{module}` module.",
            )


class NotAGuppyError(GuppyCompilerError):
    """Raised when a the program is not a GuppyModule."""

    def __init__(self, guppy: Path | None) -> None:
        """Initialize the error."""
        if guppy is None:
            super().__init__("`main` must be a GuppyModule.")
        else:
            super().__init__(f"`main` in program {guppy} must be a GuppyModule.")


if __name__ == "__main__":
    args = sys.argv[1:]
    if len(args) != 1 or args[0] in {"-h", "--help"}:
        print(
            "Compiles a Guppy file into a HUGR package, and prints the resulting JSON.\n"
            + "\n"
            + "Usage: compile_guppy.py <path_to_guppy_file>"
        )
        sys.exit(1)

    guppy_file = Path(args[0])

    compiler = GuppyCompiler()
    module = compiler.compile_guppy(input_path=guppy_file)
    print(module)
