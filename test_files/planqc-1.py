from guppylang import guppy
from guppylang.std.quantum import qubit, measure, h, rz
from guppylang.std.angles import angle


@guppy
def rx(q: qubit, a: float):
    # Implement Rx via Rz rotation
    h(q)
    rz(q, angle(a))
    h(q)


@guppy
def main() -> bool:
    q = qubit()
    rx(q, 1.5)
    return measure(q)
