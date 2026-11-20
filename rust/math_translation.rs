// Translates an English number word or Chinese pinyin into an integer
fn translate(word: &str) -> Option<i32> {
    match word {
        "zero" | "ling" => Some(0),
        "one" | "yi" => Some(1),
        "two" | "er" => Some(2),
        "three" | "san" => Some(3),
        "four" | "si" => Some(4),
        "five" | "wu" => Some(5),
        "six" | "liu" => Some(6),
        "seven" | "qi" => Some(7),
        "eight" | "ba" => Some(8),
        "nine" | "jiu" => Some(9),
        "ten" | "shi" => Some(10),
        _ => None,
    }
}

// Translates valid words, ignores unknown words, and prints the results
fn go(input: &[&str]) {
    let numbers: Vec<i32> = input
        .iter()
        .filter_map(|word| translate(word))
        .collect();

    let translation = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let addition_expression = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" + ");

    let multiplication_expression = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" * ");

    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();

    println!("Translation: {}", translation);
    println!("Addition: {} = {}", addition_expression, sum);
    println!(
        "Multiplication: {} = {}",
        multiplication_expression,
        product
    );
}

fn main() {
    let input1 = vec!["yi", "nine", "six", "ba"];
    go(&input1);

    println!();

    let input2 = vec!["yi", "josh", "three", "si"];
    go(&input2);
}