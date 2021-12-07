#!/usr/bin/env python3

input_file = 'input.txt'

def count_increase_dirty(values):
    increases = 0
    for i in range(len(values)-1):
        increases += values[i] < values[i+1]
    return increases

def count_window_increase(values, window_size):
    increases = 0
    window = sum(values[0:3])
    for i in range(len(values)-window_size):
        new_window = window - values[i] + values[i+window_size]
        increases += window < new_window
    return increases

def part1():
    with open(input_file, 'r') as reader:
        lines = reader.read()
        values = [int(x) for x in lines.split()]
        print(count_increase_dirty(values))

def part2():
    with open(input_file, 'r') as reader:
        lines = reader.read()
        values = [int(x) for x in lines.split()]
        print(count_window_increase(values, 3))

part1()
part2()
