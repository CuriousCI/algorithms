def recursive_binary_pow(base, exponent):
    if exponent == 0:
        return 1

    result = recursive_binary_pow(base, exponent >> 1)
    return result * result * (base if exponent & 1 else 1)


def binary_pow(base, exponent):
    result = 1

    while exponent > 0:
        if exponent & 1:
            result *= base
        base *= base
        exponent >>= 1

    return result


def test():
    assert recursive_binary_pow(3, 13) == 1594323
    assert binary_pow(3, 13) == 1594323


if __name__ == '__main__':
    test()
