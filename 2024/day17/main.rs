use itertools::Itertools;

#[derive(Debug, Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    ptr: usize,
    output: Vec<u64>,
}

fn parse(input: &str) -> (Computer, &str) {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let register_data = register_str
        .lines()
        .map(|l| aoc2024_utils::extract_u64s(l)[0])
        .collect_vec();
    let program = aoc2024_utils::extract_u64s(program_str);

    (
        Computer {
            a: register_data[0],
            b: register_data[1],
            c: register_data[2],
            program,
            ptr: 0,
            output: Vec::new(),
        },
        program_str.split_once(" ").unwrap().1,
    )
}

fn dv(numerator: u64, operand: u64) -> u64 {
    let denominator = 2u64.pow(operand as u32);
    numerator / denominator
}

fn combo_operand(c: &Computer, operand: u64) -> u64 {
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

fn program_output(c: &Computer) -> String {
    let mut new = c.clone();
    while new.ptr < new.program.len() {
        process_instruction(&mut new);
    }
    new.output.iter().map(|n| n.to_string()).join(",")
}

fn part1(input: &str) -> u32 {
    let (computer, _) = parse(input);
    dbg!(program_output(&computer));
    0
}

fn part2(input: &str) -> u64 {
    let (computer, program_str) = parse(input);
    let init = 0o5325644676236000;
    let mut init_a = init;
    let mut i = 0;
    loop {
        let mut test = computer.clone();
        test.a = init_a;
        let output = program_output(&test);
        println!("{:#o}: {}", init_a, output);
        init_a += 1 << (0 * 3);
        i += 1;
        if i == 1 << 12 {
            return 0;
        }
        if output == program_str {
            return init_a;
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
