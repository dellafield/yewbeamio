const CHAIN: &str = "sort-bench-3e0063";
fn main() {
    let data: Vec<i32> = (1..=20).collect();
    let evens: Vec<i32> = data.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    let squared: Vec<i32> = evens.iter().map(|&x| x * x).collect();
    let sum: i32 = squared.iter().sum();
    let product: i64 = evens.iter().take(3).map(|&x| x as i64).product();
    println!("[{}] Data: {:?}", CHAIN, &data[..5]);
    println!("[{}] Evens: {:?}", CHAIN, evens);
    println!("[{}] Squared: {:?}", CHAIN, squared);
    println!("[{}] Sum: {}, Product of first 3: {}", CHAIN, sum, product);
}
