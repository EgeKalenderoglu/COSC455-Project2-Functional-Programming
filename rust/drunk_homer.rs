// Generates every possible ordering of the values in the vector.
fn make_permutations(values: Vec<usize>) -> Vec<Vec<usize>> {
    // Recursive helper that swaps values to create each possible permutation.
    fn build(
        start: usize,
        values: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        // When all positions are filled, save the permutation.
        if start == values.len() {
            result.push(values.clone());
            return;
        }

        // Try every remaining value in the current position.
        for i in start..values.len() {
            values.swap(start, i);

            build(start + 1, values, result);

            // Swap back so the next possibility can be tested.
            values.swap(start, i);
        }
    }

    let mut values = values;
    let mut result = Vec::new();

    build(0, &mut values, &mut result);

    result
}

fn main() {
    // Store names. Their array indexes are used throughout the program.
    let stores = [
        "The Leftorium",
        "Sprawl-Mart",
        "Try-N-Save",
        "King Toots",
    ];

    // Gift names. Their indexes represent each gift.
    let gifts = [
        "Green Dress",
        "Saxophone Book",
        "Slingshot",
        "Pacifier",
    ];

    // Generate every possible order in which Homer could visit the stores.
    let store_orders =
        make_permutations(vec![0, 1, 2, 3]);

    // Generate every possible way the gifts could be assigned to stores.
    let gift_assignments =
        make_permutations(vec![0, 1, 2, 3]);

    // Test every possible store order.
    for order in &store_orders {
        // Clue: The Leftorium was the second stop.
        if order[1] != 0 {
            continue;
        }

        // Test every possible gift assignment.
        for assignment in &gift_assignments {
            // Clue: The saxophone book was bought at King Toots.
            if assignment[3] != 1 {
                continue;
            }

            // Find which store sold the slingshot.
            let slingshot_store = assignment
                .iter()
                .position(|&gift| gift == 2)
                .unwrap();

            // Find when Homer visited that store.
            let slingshot_stop = order
                .iter()
                .position(|&store| store == slingshot_store)
                .unwrap();

            // Clue: The store immediately after the slingshot
            // was not Sprawl-Mart.
            if slingshot_stop == 3
                || order[slingshot_stop + 1] == 1
            {
                continue;
            }

            // Find when Homer visited Try-N-Save.
            let try_n_save_stop = order
                .iter()
                .position(|&store| store == 2)
                .unwrap();

            // Try-N-Save must have a stop two positions after it.
            if try_n_save_stop + 2 >= 4 {
                continue;
            }

            // Clue: Two stops after Try-N-Save, Homer bought the pacifier.
            let pacifier_store =
                order[try_n_save_stop + 2];

            if assignment[pacifier_store] != 3 {
                continue;
            }

            // If all clues passed, print the solution.
            println!("Solution:");

            for (stop, &store) in order.iter().enumerate() {
                println!(
                    "Stop {}: {} - {}",
                    stop + 1,
                    stores[store],
                    gifts[assignment[store]]
                );
            }
        }
    }
}