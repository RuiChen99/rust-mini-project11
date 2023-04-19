fn main() {
    // Get the limit from the user
    let limit = get_limit();

    // Generate the list of prime numbers up to the limit
    let primes = generate_primes(limit);

    // Print the list of prime numbers
    println!("Prime numbers up to {}: {:?}", limit, primes);
}

// Function to get the limit from the user
fn get_limit() -> u32 {
    use std::io::{stdin, stdout, Write};
    
    let mut input = String::new();
    
    print!("Enter the limit: ");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().parse().expect("Invalid input")
}

// Function to generate the list of prime numbers up to the limit
fn generate_primes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();

    for num in 2..=limit {
        if is_prime(num) {
            primes.push(num);
        }
    }

    primes
}

// Function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }

    true
}
