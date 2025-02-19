from guppylang.decorator import guppy
from guppylang.std.builtins import owned
from guppylang.std.quantum import qubit, measure, h, cx, z, x


@guppy
def teleport(src: qubit @ owned, tgt: qubit @ owned) -> qubit:
    # Entangle qubits with ancilla
    q = qubit()
    h(q)
    cx(q, tgt)
    cx(src, q)
    # Apply classical corrections
    h(src)
    if measure(q):
        z(tgt)
    if measure(src):
        x(tgt)
    return tgt


@guppy
def main() -> bool:
    q1, q2 = qubit(), qubit()  # TODO initialise into some interesting state
    return measure(teleport(q1, q2))
