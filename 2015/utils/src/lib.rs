pub fn timeit<F: Fn() -> T, T>(name: &str, f: F) -> T {
    let start = std::time::Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    if elapsed.as_millis() > 1 {
        println!("{} Time: {}ms", name, elapsed.as_millis());
    } else {
        println!("{} Time: {}μs", name, elapsed.as_micros());
    }
    result
}
