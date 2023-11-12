// Step 1: Define an Enum PaymentType
#[derive(PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
}


// Step 2: Define a Seller struct
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

// Step 3: Define a Buyer struct
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32,
}

// Step 4: Define a BuyerGroup struct
struct BuyerGroup {
    members: Vec<Buyer>,
}

impl BuyerGroup {
    // Step 5: Implement add_member method
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }

    // Step 6: Implement find_buyer method
    fn find_buyer(&self, payment_type: PaymentType) -> Option<usize> {
        for (index, buyer) in self.members.iter().enumerate() {
            if buyer.payment_type == payment_type {
                return Some(index);
            }
        }
        None
    }

    // Step 7: Implement buy method
    fn buy(&mut self, buyer_index: usize, seller: &mut Seller) {
        if buyer_index < self.members.len() {
            let buyer = &mut self.members[buyer_index];
            while buyer.balance >= seller.price {
                buyer.balance -= seller.price;
                seller.balance += seller.price;
            }
        }
    }
}

fn main() {
    // Step 8a: Create 2 buyers
    let buyer1 = Buyer {
        name: String::from("John"),
        payment_type: PaymentType::DigitalToken,
        balance: 100.0,
    };

    let buyer2 = Buyer {
        name: String::from("Sally"),
        payment_type: PaymentType::Cash,
        balance: 100.0,
    };

    // Step 8b: Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup { members: vec![] };

    // Step 8c: Add 2 buyers to the buyer group
    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    // Step 8d: Create a seller
    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };

    // Step 8e: Find the index of Sally
    if let Some(sally_index) = buyer_group.find_buyer(PaymentType::Cash) {
        // Step 8f: Call the buy method
        buyer_group.buy(sally_index, &mut seller);
    }

    // Print the updated balances
    for buyer in &buyer_group.members {
        println!("Buyer {} balance: {:.2}", buyer.name, buyer.balance);
    }
    println!("Seller balance: {:.2}", seller.balance);
}
