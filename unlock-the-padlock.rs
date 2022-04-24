// Code by Jeremy Nguyen
// Language: rust, Be Afraid

// actual code
use std::io;

fn main() {
    // accept input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        // just figure out where the test cases are lol
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let n: usize = input.next().unwrap().parse().unwrap();
        let d: usize = input.next().unwrap().parse().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let mut arr: Vec<usize> = Vec::new();
        for _ in 0..n {
            arr.push(input.next().unwrap().parse().unwrap());
        }
        // since rust is such a powerful language and I'm not sure how to do this in rust, I'm just going to do it like this stupidly because its such a high level language
        let mut result = String::new();
        result.push_str("Case #");
        result.push_str(&(d.to_string()));
        result.push_str(": ");
        result.push_str(&((make_zero(n, d, arr)).to_string()));
        println!("{}", result);
    }
}  
      

fn make_zero(n: usize, d: usize, arr: Vec<usize>) -> usize {
    let mut result = 0;
    let mut arr = arr;
    let mut i = 0;
    // loop through the dials
    while i < arr.len() {
      if arr[i] == 0 {
        i += 1;
        continue;
      }
      if arr[i] > 0 {
        // if the dial is positive, we need to subtract it from the dials that are greater than it
        for j in i + 1..arr.len() {
          if arr[j] > arr[i] {
            arr[j] -= arr[i];
          }
        }
        result += 1;
      } else {
        // if the dial is negative, we need to add it to the dials that are less than it
        for j in i + 1..arr.len() {
          if arr[j] < arr[i] {
            arr[j] += arr[i];
          }
        }
        result += 1;
      }
      i += 1;
    }
    result
}