#!/usr/bin/env python3
import functools

input_file = 'data/day3-sample.txt'
input_file = 'data/day3-input.txt'


def part1():
    with open(input_file, 'r') as reader:
        lines = reader.read()
        binary_values = lines.split()
        num_bits = len(binary_values[0])
        values = [int(x, 2) for x in binary_values]

        bit_counts = [0] * num_bits
        for value in values:
            for bit in range(num_bits):
                bit_counts[bit] += (value >> (num_bits - bit - 1)) & 1

        print(bit_counts)

        most_common_bits = [count > len(values)/2 for count in bit_counts]
        print(most_common_bits)

        gamma_rate = functools.reduce(
            lambda x, y: x << 1 | y, most_common_bits)
        print(gamma_rate)
        epsilon_rate = ((1 << num_bits) - 1) ^ gamma_rate
        print(epsilon_rate)
        print(gamma_rate * epsilon_rate)


def part2():
    return


part1()
part2()
