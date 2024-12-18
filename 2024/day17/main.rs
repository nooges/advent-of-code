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

fn parse(input: &str) -> Computer {
    let (register_str, program_str) = input.split_once("\n\n").unwrap();
    let register_data = register_str
        .lines()
        .map(|l| aoc2024_utils::extract_u64s(l)[0])
        .collect_vec();
    let program = aoc2024_utils::extract_u64s(program_str);

    Computer {
        a: register_data[0],
        b: register_data[1],
        c: register_data[2],
        program,
        ptr: 0,
        output: Vec::new(),
    }
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

    match opcode {
        0 => c.a >>= combo_operand(c, operand),
        1 => c.b ^= operand,
        2 => c.b = combo_operand(c, operand) & 7,
        3 if c.a != 0 => {
            c.ptr = operand as usize;
            return;
        }
        4 => c.b ^= c.c,
        5 => c.output.push(combo_operand(c, operand) & 7),
        6 => c.b = c.a >> combo_operand(c, operand),
        7 => c.c = c.a >> combo_operand(c, operand),
        _ => (),
    }
    c.ptr += 2;
}

fn program_output(c: &Computer) -> Vec<u64> {
    let mut new = c.clone();
    while new.ptr < new.program.len() {
        process_instruction(&mut new);
    }
    new.output
}

fn part1(input: &str) -> String {
    let computer = parse(input);
    program_output(&computer)
        .into_iter()
        .map(|n| n.to_string())
        .join(",")
}

fn part2(input: &str) -> u64 {
    let computer = parse(input);
    let mut init_a = 1 << 45;

    // Figure out octal digits, from left to right
    let mut oct_digit = 15;
    let mut total_checks = 0;
    loop {
        loop {
            let mut test = computer.clone();
            total_checks += 1;
            test.a = init_a;
            let output = program_output(&test);
            //println!("{:#o}: {:?}", init_a, output);
            if output[oct_digit..16] == computer.program[oct_digit..16] {
                break;
            }
            init_a += 1 << (oct_digit * 3);
        }
        println!("Found output match: {:#o}", init_a);
        if oct_digit == 0 {
            break;
        }
        oct_digit -= 1;
    }
    dbg!(total_checks);
    init_a
}

fn main() -> std::io::Result<()> {
    let input = include_str!("input.txt");

    aoc2024_utils::timeit_str("Part 1", || part1(input));
    aoc2024_utils::timeit_u64("Part 2", || part2(input));
    Ok(())
}
