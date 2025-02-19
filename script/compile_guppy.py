"""Methods for compiling Guppy programs into HUGRs."""

import importlib.machinery
import types
from pathlib import Path
import sys

try:
    import guppylang
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
    ) -> str:
        """Load a Guppy file as a Python module, and return it."""
        if guppylang.__version__ < MINIMUM_GUPPY_VERSION:
            raise OldGuppyVersion(guppylang.__version__)

        module_name = input_path.stem.replace("-", "_").replace(".", "_")
        loader = importlib.machinery.SourceFileLoader(module_name, str(input_path))
        py_module = types.ModuleType(loader.name)
        try:
            loader.load_module(module_name)
        except FileNotFoundError as err:
            raise InvalidGuppyModulePathError(input_path) from err

        module: GuppyModule = self._get_module(py_module, input_path)
        pkg = module.compile().package
        return pkg.to_json()

    def _get_module(
        self,
        py_module: types.ModuleType,
        source_path: Path,
    ) -> GuppyModule:
        for module_id in guppy.registered_modules():
            if module_id.module == py_module or module_id.filename == source_path:
                module = guppy.get_module(module_id)
                break
        else:
            raise MissingModuleError()

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

    def __init__(self) -> None:
        """Initialize the error."""
        super().__init__("The Guppy program does not define a local module.")


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
