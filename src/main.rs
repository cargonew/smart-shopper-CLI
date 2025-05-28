use std::fs;
use std::collections::HashMap;
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
struct Args {
    items: Vec<String>,
}

type StoreData = HashMap<String, HashMap<String, f64>>;

fn load_all_store_data() -> StoreData {
    let mut all_data = HashMap::new();
    let path = Path::new("data");

    for entry in fs::read_dir(path).expect("Could not read data directory") {
        let entry = entry.expect("Invalid entry");
        let content = fs::read_to_string(entry.path()).expect("Could not read file");

        let store_json: StoreData = serde_json::from_str(&content).expect("Invalid JSON format");

        for (store, items) in store_json {
            all_data
                .entry(store)
                .and_modify(|m : &mut HashMap<String, f64>|   m.extend(items.clone()))
                .or_insert(items);
        }
    }

    all_data
}

fn main() {
    let args = Args::parse();
    let data = load_all_store_data();
    let mut total = 0.0;

    for query in &args.items {
        let mut cheapest: Option<(&String, &String, &f64)> = None;

        for (store, items) in &data {
            for (name, price) in items {
                if name.to_lowercase().contains(&query.to_lowercase()) {
                    if cheapest.is_none() || price < cheapest.unwrap().2 {
                        cheapest = Some((store, name, price));
                    }
                }
            }
        }

        match cheapest {
            Some((store, name, price)) => {
                println!("{} - R{:.2} from {}", name, price, store);
                total += price;
            }
            None => println!("{} - not found in any store", query),
        }
    }

    println!("\nTotal: R{:.2}", total);
}

