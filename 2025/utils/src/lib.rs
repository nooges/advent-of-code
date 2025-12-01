use regex::Regex;

pub fn timeit<F: Fn() -> u32>(name: &str, f: F) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 10 {
        println!("{}: {} \tTime: {}ms", name, result, elapsed.as_millis());
    } else {
        println!("{}: {} \tTime: {}μs", name, result, elapsed.as_micros());
    }
}

pub fn timeit_u64<F: Fn() -> u64>(name: &str, f: F) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 10 {
        println!("{}: {} \tTime: {}ms", name, result, elapsed.as_millis());
    } else {
        println!("{}: {} \tTime: {}μs", name, result, elapsed.as_micros());
    }
}

pub fn timeit_str<F: Fn() -> String>(name: &str, f: F) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 10 {
        println!("{}: {} \tTime: {}ms", name, result, elapsed.as_millis());
    } else {
        println!("{}: {} \tTime: {}μs", name, result, elapsed.as_micros());
    }
}

pub fn extract_u32s(input: &str) -> Vec<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

pub fn extract_i32s(input: &str) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

pub fn extract_u64s(input: &str) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}
