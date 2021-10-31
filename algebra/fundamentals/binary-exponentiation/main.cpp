#include <cstdint>
#include <cassert>

uint64_t recursive_binary_pow(uint64_t base, uint64_t exponent)
{
	if (!exponent)
		return 1;

	auto result = recursive_binary_pow(base, exponent >> 1);
	return result * result * (exponent & 1 ? base : 1);
}

uint64_t binary_pow(uint64_t base, uint64_t exponent)
{
	uint64_t result = 1;
	while (exponent)
	{
		if (exponent & 1)
			result *= base;
		base *= base;
		exponent >>= 1;
	}
	return result;
}

uint64_t modular_binary_pow(uint64_t base, uint64_t exponent, uint64_t modulo)
{
	base %= modulo;
	uint64_t result = 1;

	while (exponent)
	{
		if (exponent & 1)
			result *= base % modulo;
		base *= base % modulo;
		exponent >>= 1;
	}

	return result % modulo;
}

int main()
{
	assert(recursive_binary_pow(3, 13) == 1594323);
	assert(binary_pow(3, 13) == 1594323);
	assert(modular_binary_pow(3, 13, 100) == 23);
}