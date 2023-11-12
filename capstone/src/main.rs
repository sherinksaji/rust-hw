mod location; 
mod transaction;

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use location::{Country, Continent};
use transaction::Transaction;
use std::str::FromStr;
use std::collections::HashMap;


// Function to filter transactions by continent and print only those involving European companies
fn print_transactions_by_continent(transactions: &[Transaction], continent_filter: &Continent) {
    println!("Transactions with European companies:");

    transactions
        .iter()
        .filter(|&transaction| transaction.continent == *continent_filter)
        .for_each(|transaction| println!("{:?}", transaction));
}

fn main() {
    // Step a: Create a file variable
    let file = File::open("./transactions.csv").unwrap();

    // Step b: Create a reader variable
    let reader = io::BufReader::new(file);

    // Step c: Create a mutable transactions variable
    let mut transactions: Vec<Transaction> = Vec::new();

    // Step d: Create a mutable skipped_lines variable
    let mut skipped_lines = Vec::new();




    // Step e: Run a for loop with destructuring
    for (idx, line) in reader.lines().enumerate() {
        // If it's the header line, skip it
        if idx == 0 {
            continue;
        }

        // Create line_str variable
        let line_str = line.unwrap();

        // Create parsed_transaction variable
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        // Match on parsed_transaction
        match parsed_transaction {
            Ok(transaction) => {
                // Push value within Ok into transactions
                transactions.push(transaction);
            }
            Err(err) => {
                // Push the tuple of (idx, value within Err, line_str) into skipped_lines
                skipped_lines.push((idx, err, line_str));
            }
        }
    }

    // Utilize HashMap to keep track of total invested amount per continent
    let mut total_invested: HashMap<String, f64> = HashMap::new();

    // Step f: Run a for loop to print transactions
    for transaction in &transactions {
        println!("{:?}", transaction);

        // Update total invested amount per continent
        let continent_key = format!("{:?}", transaction.continent);
        let total = total_invested.entry(continent_key.clone()).or_insert(0.0);
        *total += transaction.amount;
    }

    // Print total invested amount per continent
    for (continent, total_amount) in total_invested.iter() {
        println!("Total invested amount in {} is: {}", continent, total_amount);
    }

     // Call function to filter transactions by continent and print European transactions
    print_transactions_by_continent(&transactions, &Continent::Europe);


    // Step g: Run a for loop to print skipped_lines
    for (idx, err, line_str) in &skipped_lines {
        println!("Error at line {}: {}, Line content: {}", idx, err, line_str);
    }

}