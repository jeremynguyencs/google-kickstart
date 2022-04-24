use std::io;

// return the number of factors of A which are palindromes
fn main() {
    // accept input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    // loop through the test cases
    for _ in 0..t {
      let mut count = 0;
      let mut input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let n: usize = input.trim().parse().unwrap();
      // find the number of factors of A which are palindromes
      let mut num_result = 0;
      let mut factors = find_factors(n);
      for i in 0..factors.len() {
        // if the factor is a palindrome, increment the count
        if is_palindrome(factors[i]) {
          num_result += 1;
        }
      }
      count+=1;
      // output the number of factors of A which are palindromes
      let mut result = String::new();
      result.push_str("Case #");
      result.push_str(&(count.to_string()));
      result.push_str(": ");
      result.push_str(&(num_result.to_string()));
      println!("{}", result);
    }
}

fn find_factors(n: usize) -> Vec<usize> {
  let mut result = Vec::new();
  let mut i = 1;
  while i <= n {
    if n % i == 0 {
      result.push(i);
    }
    i += 1;
  }
  result
}

// return true if num is a palindrome
fn is_palindrome(num: usize) -> bool {
  let num_string = num.to_string();
  num_string.chars().zip(num_string.chars().rev()).take(num_string.chars().count() / 2).all(|(a, b)| a == b)
}