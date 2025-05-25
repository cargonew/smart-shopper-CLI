use std::collections::HashMap;
use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn main() {
    println!("Enter your budget in Rands:");
    let budget: i32 = read_line().parse().unwrap_or(0);

    println!("Enter your preferred distance (km):");
    let _radius: i32 = read_line().parse().unwrap_or(0);

    println!("How many shops do you want to shop at?");
    let num_shops: usize = read_line().parse().unwrap_or(0);

    println!("Enter names of your preferred shops:");
    let mut shops = Vec::new();
    for _ in 0..num_shops {
        let shop = read_line();
        shops.push(shop);
    }

    println!("Enter your grocery list (type 'done' when finished):");
    let mut grocery_list = Vec::new();
    loop {
        let item = read_line();
        if item.to_lowercase() == "done" {
            break;
        }
        grocery_list.push(item);
    }

    // Dummy price table
    let mut price_table: HashMap<String, HashMap<String, i32>> = HashMap::new();

    price_table.insert("Shoprite".into(), {
        let mut p = HashMap::new();
        p.insert("milk".into(), 20);
        p.insert("bread".into(), 15);
        p.insert("eggs".into(), 30);
        p
    });

    price_table.insert("PicknPay".into(), {
        let mut p = HashMap::new();
        p.insert("milk".into(), 22);
        p.insert("bread".into(), 14);
        p.insert("eggs".into(), 28);
        p
    });

    println!("\nSmart Shopping Plan:\n");

    let mut total_cost = 0;

    for item in &grocery_list {
        let mut min_price = i32::MAX;
        let mut best_shop = "N/A";

        for shop in &shops {
            if let Some(shop_prices) = price_table.get(shop) {
                if let Some(&price) = shop_prices.get(item) {
                    if price < min_price {
                        min_price = price;
                        best_shop = shop;
                    }
                }
            }
        }

        if min_price == i32::MAX {
            println!("{:<10} not available in any preferred shop.", item);
        } else {
            println!("{:<10} - R{:<3} at {}", item, min_price, best_shop);
            total_cost += min_price;
        }
    }

    println!("\nTotal cost: R{}", total_cost);
    if total_cost <= budget {
        println!("You're within budget!");
    } else {
        println!("Over budget by R{}", total_cost - budget);
    }
}

