use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let r: usize = input.next().unwrap().parse().unwrap();
        let a: usize = input.next().unwrap().parse().unwrap();
        let b: usize = input.next().unwrap().parse().unwrap();
        let mut result = String::new();
        result.push_str("Case #");
        result.push_str(&(r.to_string()));
        result.push_str(": ");
        result.push_str(&(solve(r, a, b).to_string()));
        println!("{}", result);
    }
}

// solve(r, a, b) returns the sum of areas of all the circles drawn
fn solve(r: usize, a: usize, b: usize) -> f64 {
    let mut result = 0.0;
    let mut r = r;
    while r > 0 {
        result += r as f64 * r as f64 * 3.14159265358979323846264338327950288;
        r = r * a / b;
    }
    result*10.0
}