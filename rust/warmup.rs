// Recursively searches the slice for the number 42
fn hitchhiker(numbers: &[i32]) -> bool {
    match numbers {
        [] => false,
        [42, ..] => true,
        [_, rest @ ..] => hitchhiker(rest),
    }
}

// Returns every second element using pattern matching and recursion
fn everyother(numbers: &[i32]) -> Vec<i32> {
    match numbers {
        [] | [_] => Vec::new(),
        [_, second, rest @ ..] => {
            let mut result = everyother(rest);
            result.insert(0, *second);
            result
        }
    }
}

// Checks recursively whether n has a divisor
fn has_divisor(n: i32, d: i32) -> bool {
    if d * d > n {
        false
    } else if n % d == 0 {
        true
    } else {
        has_divisor(n, d + 1)
    }
}

// Returns true if n is prime
fn is_prime(n: i32) -> bool {
    n >= 2 && !has_divisor(n, 2)
}

// Searches for two prime numbers whose sum is n
fn find_goldbach(n: i32, x: i32) -> Option<(i32, i32)> {
    if x >= n {
        None
    } else if is_prime(x) && is_prime(n - x) {
        Some((x, n - x))
    } else {
        find_goldbach(n, x + 1)
    }
}

// Validates the input and prints the Goldbach result
fn goldbach(n: i32) {
    if n <= 2 || n % 2 != 0 {
        println!("Error: number must be even and greater than 2");
        return;
    }

    match find_goldbach(n, 2) {
        Some((a, b)) => println!("{} + {} = {}", a, b, n),
        None => println!("No solution found"),
    }
}

fn main() {
    println!("{}", hitchhiker(&[1, 5, 42, 10]));
    println!("{}", hitchhiker(&[1, 5, 10]));

    println!(
        "{:?}",
        everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43])
    );

    goldbach(28);
    goldbach(27);
}