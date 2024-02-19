import math

for y in range(14, -15, -1):
    for x in range(-31, 32):
        print("*" if y == round(10 * math.sin(x / 10)) else " ", end="")
    print()
