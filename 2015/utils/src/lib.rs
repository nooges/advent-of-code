pub fn timeit<F: Fn() -> u32>(name: &str, f: F) {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 1 {
        println!("{}: {} \tTime: {}ms", name, result, elapsed.as_millis());
    } else {
        println!("{}: {} \tTime: {}μs", name, result, elapsed.as_micros());
    }
}

pub fn extract_u32s(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}
