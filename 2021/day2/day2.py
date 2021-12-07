#!/usr/bin/env python3

input_file = 'input.txt'


def parse_command(command):
    direction = command[0]
    distance = int(command[1])
    if direction == 'forward':
        return (distance, 0)
    if direction == 'down':
        return (0, distance)
    if direction == 'up':
        return (0, -distance)


def part1():
    with open(input_file, 'r') as reader:
        lines = reader.read()
        commands = [x.split() for x in lines.split('\n')]
        print(commands)
        position_changes = [parse_command(command) for command in commands]
        print(position_changes)
        horizontal_change = sum([x[0] for x in position_changes])
        print(horizontal_change)
        depth_change = sum([x[1] for x in position_changes])
        print(depth_change)
        print(horizontal_change * depth_change)


part1()
