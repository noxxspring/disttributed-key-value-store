use std::{io::{self, Write}, sync::{Arc, Mutex}};

use crate::node::KVStore;



pub fn run_cli(store: Arc<Mutex<KVStore>>) {

    println!("🔑 DistKV CLI - Distributed Key-Value Store");
    println!("Type 'help' to see available commands.");
    println!();

    loop {
        print!("distkv> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();
        let command  = parts[0].to_uppercase();

        match command.as_str() {
            "SET" => {
                if parts.len() < 3 {
                    println!("⚠️ Usage: SET <key> <value>");
                    continue;
                }
                let key = parts[1];
                let value = parts[2];
                let mut store = store.lock().unwrap();
                store.set(key.to_string(), value.to_string());
                println!("✅ Set key '{}' = '{}'", key, value);
            }
            "GET" => {
                if parts.len() != 2 {
                    println!("⚠️ Usage: GET <key>");
                    continue;
                }
                let key = parts[1];
                let store = store.lock().unwrap();
                match store.get(key) {
                    Some(value) => println!("🔍 '{}' = '{}'", key, value),
                    None => println!("❌ Key '{}' not found", key),
                }
            }
            "DELETE" => {
                if parts.len() != 2 {
                    println!("⚠️ Usage: DELETE <key>");
                    continue;
                }
                let key = parts[1];
                let mut store = store.lock().unwrap();
                if store.delete(key) {
                    println!("🗑️ Deleted key '{}'", key);
                } else {
                    println!("❌ Key '{}' not found", key);
                }
            }
            "HELP" => {
                println!(
                    "\nAvailable Commands:
  SET <key> <value>    - Set a key-value pair
  GET <key>            - Retrieve the value of a key
  DELETE <key>         - Remove a key
  HELP                 - Show this message
  EXIT                 - Quit the CLI\n"
                );
            }
            "EXIT" => {
                println!("👋 Exiting DistKV CLI.");
                break;
            }
            _ => {
                println!("❓ Unknown command '{}'. Type 'HELP' for help.", command);
            }
        }
    }
}