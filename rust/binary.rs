// Reverses a slice recursively
fn reverse_list<T: Copy>(values: &[T]) -> Vec<T> {
    match values {
        [] => Vec::new(),
        [first, rest @ ..] => {
            let mut result = reverse_list(rest);
            result.push(*first);
            result
        }
    }
}

// Converts 0 and 1 integers into Boolean values
fn to_bool(values: &[i32]) -> Vec<bool> {
    match values {
        [] => Vec::new(),

        [0, rest @ ..] => {
            let mut result = vec![false];
            result.extend(to_bool(rest));
            result
        }

        [1, rest @ ..] => {
            let mut result = vec![true];
            result.extend(to_bool(rest));
            result
        }

        _ => panic!("Invalid binary digit"),
    }
}

// Converts Boolean values back into 0 and 1
fn to_int(values: &[bool]) -> Vec<i32> {
    match values {
        [] => Vec::new(),

        [false, rest @ ..] => {
            let mut result = vec![0];
            result.extend(to_int(rest));
            result
        }

        [true, rest @ ..] => {
            let mut result = vec![1];
            result.extend(to_int(rest));
            result
        }
    }
}

// Finishes addition if one binary list ends first
fn finish_binary_add(values: &[bool], carry: bool) -> Vec<bool> {
    match (values, carry) {
        ([], false) => Vec::new(),
        ([], true) => vec![true],

        ([x, rest @ ..], carry) => {
            let (bit, next_carry) = match (*x, carry) {
                (false, false) => (false, false),
                (true, false) => (true, false),
                (false, true) => (true, false),
                (true, true) => (false, true),
            };

            let mut result = vec![bit];
            result.extend(finish_binary_add(rest, next_carry));
            result
        }
    }
}

// Adds two reversed Boolean binary lists
fn do_binary_addition(
    a: &[bool],
    b: &[bool],
    carry: bool,
) -> Vec<bool> {
    match (a, b) {
        ([], []) => {
            if carry {
                vec![true]
            } else {
                Vec::new()
            }
        }

        (values, []) => finish_binary_add(values, carry),

        ([], values) => finish_binary_add(values, carry),

        ([x, xs @ ..], [y, ys @ ..]) => {
            let (bit, next_carry) = match (*x, *y, carry) {
                (false, false, false) => (false, false),
                (false, false, true) => (true, false),
                (false, true, false) => (true, false),
                (true, false, false) => (true, false),
                (true, true, false) => (false, true),
                (true, false, true) => (false, true),
                (false, true, true) => (false, true),
                (true, true, true) => (true, true),
            };

            let mut result = vec![bit];
            result.extend(do_binary_addition(
                xs,
                ys,
                next_carry,
            ));
            result
        }
    }
}

// Main binary addition function
fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a_bits = to_bool(a);
    let b_bits = to_bool(b);

    let a_reversed = reverse_list(&a_bits);
    let b_reversed = reverse_list(&b_bits);

    let result =
        do_binary_addition(&a_reversed, &b_reversed, false);

    let normal_order = reverse_list(&result);

    to_int(&normal_order)
}

// Pads the shorter reversed binary list with false values
fn pad_binary(a: &[bool], b: &[bool]) -> (Vec<bool>, Vec<bool>) {
    match (a, b) {
        ([], []) => (Vec::new(), Vec::new()),

        ([x, xs @ ..], [y, ys @ ..]) => {
            let (mut left, mut right) =
                pad_binary(xs, ys);

            left.insert(0, *x);
            right.insert(0, *y);

            (left, right)
        }

        ([x, xs @ ..], []) => {
            let (mut left, mut right) =
                pad_binary(xs, &[]);

            left.insert(0, *x);
            right.insert(0, false);

            (left, right)
        }

        ([], [y, ys @ ..]) => {
            let (mut left, mut right) =
                pad_binary(&[], ys);

            left.insert(0, false);
            right.insert(0, *y);

            (left, right)
        }
    }
}

// Flips every bit recursively
fn invert_bits(values: &[bool]) -> Vec<bool> {
    match values {
        [] => Vec::new(),

        [first, rest @ ..] => {
            let mut result = vec![!*first];
            result.extend(invert_bits(rest));
            result
        }
    }
}

// Adds one to a reversed binary list while keeping its width
fn add_one_fixed(values: &[bool]) -> Vec<bool> {
    match values {
        [] => Vec::new(),

        [false, rest @ ..] => {
            let mut result = vec![true];
            result.extend_from_slice(rest);
            result
        }

        [true, rest @ ..] => {
            let mut result = vec![false];
            result.extend(add_one_fixed(rest));
            result
        }
    }
}

// Creates the two's complement of a binary list
fn twos_complement(values: &[bool]) -> Vec<bool> {
    let inverted = invert_bits(values);
    add_one_fixed(&inverted)
}

// Keeps the result at the original bit width
fn keep_width(result: &[bool], width: &[bool]) -> Vec<bool> {
    match width {
        [] => Vec::new(),

        [_w, rest_width @ ..] => {
            match result {
                [x, rest_result @ ..] => {
                    let mut output = vec![*x];
                    output.extend(keep_width(
                        rest_result,
                        rest_width,
                    ));
                    output
                }

                [] => {
                    let mut output = vec![false];
                    output.extend(keep_width(
                        &[],
                        rest_width,
                    ));
                    output
                }
            }
        }
    }
}

// Recursively copies a list
fn copy_list(values: &[i32]) -> Vec<i32> {
    match values {
        [] => Vec::new(),

        [first, rest @ ..] => {
            let mut result = vec![*first];
            result.extend(copy_list(rest));
            result
        }
    }
}

// Removes unnecessary leading zeros
fn remove_leading_zeros(values: &[i32]) -> Vec<i32> {
    match values {
        [] => vec![0],
        [0] => vec![0],

        [0, rest @ ..] =>
            remove_leading_zeros(rest),

        _ => copy_list(values),
    }
}

// Binary subtraction using two's complement
fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    let a_bits = to_bool(a);
    let b_bits = to_bool(b);

    let a_reversed = reverse_list(&a_bits);
    let b_reversed = reverse_list(&b_bits);

    let (a_padded, b_padded) =
        pad_binary(&a_reversed, &b_reversed);

    let negative_b =
        twos_complement(&b_padded);

    let full_result =
        do_binary_addition(
            &a_padded,
            &negative_b,
            false,
        );

    let result =
        keep_width(&full_result, &a_padded);

    let normal_order =
        reverse_list(&result);

    let integers =
        to_int(&normal_order);

    remove_leading_zeros(&integers)
}

fn main() {
    // Addition tests
    println!(
        "{:?}",
        binary_addition(
            &[1, 1, 1, 1, 0],
            &[1, 0, 1, 1]
        )
    );

    println!(
        "{:?}",
        binary_addition(
            &[1, 0, 0, 1, 1, 0, 1],
            &[0, 0, 1, 0, 0, 1, 0]
        )
    );

    println!(
        "{:?}",
        binary_addition(
            &[1, 0, 0, 1, 0, 0, 1],
            &[0, 0, 1, 1, 0, 0, 1]
        )
    );

    // Subtraction tests
    println!(
        "{:?}",
        binary_subtraction(
            &[1, 0, 1, 0, 0, 1],
            &[1, 0, 1, 1]
        )
    );

    println!(
        "{:?}",
        binary_subtraction(
            &[1, 0, 1, 1, 1, 1, 1],
            &[0, 0, 1, 0, 0, 1, 0]
        )
    );
}