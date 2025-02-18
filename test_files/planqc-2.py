from guppylang.decorator import guppy
from guppylang.std.builtins import owned
from guppylang.std.quantum import qubit, measure, h, cx, z, x


@guppy
def teleport(src: qubit @ owned, tgt: qubit @ owned) -> qubit:
    # Entangle qubits with ancilla
    tmp, tgt = cx(h(qubit()), tgt)
    src, tmp = cx(src, tmp)
    # Apply classical corrections
    if measure(h(src)):
        tgt = z(tgt)
    if measure(tmp):
        tgt = x(tgt)
    return tgt


@guppy
def main() -> bool:
    q1, q2 = qubit(), qubit()  # TODO initialise into some interesting state
    return measure(q1, q2)
