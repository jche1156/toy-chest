import math

print(
    "\n".join(
        "".join(
            "*" if round(10 * math.sin(x / 10)) == y else " " for x in range(-31, 32)
        )
        for y in range(14, -15, -1)
    )
)
