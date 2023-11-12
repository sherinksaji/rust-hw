// Define the UserAccount struct
struct UserAccount {
    name: String,
    age: Option<u32>,
}

// Define the Balance trait
trait Balance {
    fn get_balance(&self) -> u32;
}

// Implement the Balance trait for UserAccount
impl Balance for UserAccount {
    fn get_balance(&self) -> u32 {
        10
    }
}

// Create the increase_balance function
fn increase_balance<T: Balance>(account: &T, amount: u32) -> Result<u32, String> {
    if amount <= 10 {
        Ok(account.get_balance() + amount)
    } else {
        Err("Increase must be less than 10!".to_string())
    }
}

fn main() {
    // Create a UserAccount and set the age as Option::None
    let user_account = UserAccount {
        name: "John".to_string(),
        age: None,
    };

    // Increase the user_account's balance by 11
    let increase_amount = 11;
    let result = increase_balance(&user_account, increase_amount);

    // Use a match to handle the result
    match result {
        Ok(new_balance) => {
            println!("UserAccount balance increased to {}", new_balance);
        }
        Err(error_message) => {
            println!("{}", error_message);
        }
    }

    // Use an if...let...else statement to print the UserAccount age if it is Some
    if let Some(age) = user_account.age {
        println!("UserAccount age is: {}", age);
    } else {
        println!("UserAccount age is not specified.");
    }
}

