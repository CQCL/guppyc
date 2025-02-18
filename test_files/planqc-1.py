from guppylang import guppy
from guppylang.std.quantum import qubit, measure, h, rz


@guppy
def rx(q: qubit, a: float) -> None:
    # Implement Rx via Rz rotation
    h(rz(h(q), a))


@guppy
def main() -> bool:
    q = qubit()
    r = rx(q, 1.5)
    return measure(r)
