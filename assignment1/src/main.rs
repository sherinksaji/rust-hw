struct User {
    name: String,
    balance: (f64, String),
}

impl User {
    fn print_user_detail(&self) {
        println!("Name: {}", self.name);
        println!("Balance: {} {}", self.balance.0, self.balance.1);
    }
}

fn accrue_interest(user: &mut User, interest_percentage: f64) {
    let interest_amount = user.balance.0 * interest_percentage / 100.0;
    user.balance.0 += interest_amount;
    println!("Accrued interest: {} {}", interest_amount, user.balance.1);
}

fn main() {
    let mut user = User {
        name: "John".to_string(),
        balance: (100.00, "SGD".to_string()),
    };

    user.print_user_detail();

    for _ in 0..5 {
        accrue_interest(&mut user, 5.0); // Simulating 5% interest
        user.print_user_detail();
    }
}
