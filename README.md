#  Grocery Picker CLI (Rust)

**Grocery Picker** is a command-line tool built in Rust that helps you find the **cheapest items** across multiple grocery stores using local flyer data (in JSON format)...well...thats the plan ...for now ...it works..to an extent.. Type what you need — it searches and returns the best deal per item...



Features

-  Search multiple stores with a single command
-  Uses real JSON data from weekly store flyers
-  Case-insensitive matching
-  Ignores items not found, gracefully
-  Calculates total shopping cost


 How It Works

1. Store flyers are manually converted to `JSON` and saved in the `data/` folder.
2. You run the app with the list of items you're shopping for.
3. The CLI checks every store for each item and returns the lowest price available.

 Example

```bash
cargo run -- potatoes cheese bread pears wors beef

