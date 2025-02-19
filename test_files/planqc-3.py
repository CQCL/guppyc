from guppylang.decorator import guppy
from guppylang.std.quantum import qubit, measure, h, cx, z, x, tdg, t, discard


@guppy
def rus(q: qubit, tries: int) -> None:
    for _ in range(tries):
        # Prepare ancillary qubits
        a, b = qubit(), qubit()
        h(a)
        h(b)

        tdg(a)
        cx(b, a)
        t(a)
        if not measure(a):
            # First part failed; try again
            discard(b)
            continue
        t(q)
        z(q)
        cx(q, b)
        t(b)
        if measure(b):
            # Success, we are done
            break

        # Otherwise, apply correction
        x(q)


@guppy
def main() -> bool:
    q = qubit()  # todo initialise into an interesting state
    rus(q, 100)
    return measure(q)
