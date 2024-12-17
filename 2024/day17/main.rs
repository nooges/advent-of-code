use itertools::Itertools;

#[derive(Debug, Clone)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<u32>,
    ptr: usize,
    output: Vec<u32>,
}

fn parse(input: &str) -> Computer {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let register_data = register_str
        .lines()
        .map(|l| aoc2024_utils::extract_u32s(l)[0])
        .collect_vec();
    let program = aoc2024_utils::extract_u32s(program_str);

    Computer {
        a: register_data[0],
        b: register_data[1],
        c: register_data[2],
        program,
        ptr: 0,
        output: Vec::new(),
    }
}

fn dv(numerator: u32, operand: u32) -> u32 {
    let denominator = 2u32.pow(operand);
    numerator / denominator
}

fn combo_operand(c: &Computer, operand: u32) -> u32 {
    match operand {
        4 => c.a,
        5 => c.b,
        6 => c.c,
        7 => panic!("Reserved"),
        _ => operand,
    }
}
fn process_instruction(c: &mut Computer) {
    let opcode = c.program[c.ptr];
    let operand = c.program[c.ptr + 1];
    let mut jump = false;

    match opcode {
        0 => {
            c.a = dv(c.a, combo_operand(c, operand));
        }
        1 => {
            c.b ^= operand;
        }
        2 => {
            c.b = combo_operand(c, operand) & 7;
        }
        3 => {
            if c.a != 0 {
                c.ptr = operand as usize;
                jump = true;
            }
        }
        4 => {
            c.b ^= c.c;
        }
        5 => {
            c.output.push(combo_operand(c, operand) & 7);
        }
        6 => {
            c.b = dv(c.a, combo_operand(c, operand));
        }
        7 => {
            c.c = dv(c.a, combo_operand(c, operand));
        }
        _ => (),
    }

    if !jump {
        c.ptr += 2;
    }
}

fn part1(input: &str) -> u32 {
    let mut computer = parse(input);
    dbg!(&computer);
    while computer.ptr < computer.program.len() {
        process_instruction(&mut computer);
        dbg!(&computer);
    }
    dbg!(computer.output.into_iter().map(|n| n.to_string()).join(","));
    0
}

fn part2(input: &str) -> u32 {
    let mut computer = parse(input);
    dbg!(&computer);
    while computer.ptr < computer.program.len() {
        process_instruction(&mut computer);
        dbg!(&computer);
    }
    dbg!(computer.output.into_iter().map(|n| n.to_string()).join(","));
    0
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    //aoc2024_utils::timeit("Part 2", || part2(input));
    Ok(())
}
