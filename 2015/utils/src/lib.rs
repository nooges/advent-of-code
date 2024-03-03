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
