from guppylang.decorator import guppy
from guppylang.std.quantum import qubit, measure, h, cx, z, x, tdg, t, discard


@guppy
def rus(q: qubit, tries: int) -> None:
    for _ in range(tries):
        # Prepare ancillary qubits
        a, b = h(qubit()), h(qubit())

        b, a = cx(b, tdg(a))
        if not measure(t(a)):
            # First part failed; try again
            discard(b)
            continue

        q, b = cx(z(t(q)), b)
        if measure(t(b)):
            # Success, we are done
            break

        # Otherwise, apply correction
        q = x(q)


@guppy
def main() -> bool:
    q = qubit()  # todo initialise into an interesting state
    return measure(rus(q, 100))
