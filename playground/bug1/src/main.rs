use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

fn process_payment(transaction: &str) {
    println!("Processing transaction: {}", transaction);
    // Simulate processing time
    sleep(Duration::from_secs(1));
}

fn main() {
    let mut transactions: VecDeque<String> = VecDeque::from(vec![
        "Payment 1".to_string(),
        "Payment 2".to_string(),
        "Payment 3".to_string(),
    ]);

    println!("Starting payment processing...");

    while transactions.len() > 0 {
        let transaction = transactions.front();

        if let Some(tx) = transaction {
            process_payment(&tx);
        }
    }

    println!("All transactions processed!");
}
