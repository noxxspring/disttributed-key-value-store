use std::{io::{self, Write}, thread};

use crate::node::Node;



pub fn run_cli() {
    let node = Node::new();

    println!();
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

        let parts: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

        if parts.is_empty() {
            continue;
        }

        let command  = parts[0].to_lowercase();

        match command.as_str() {
            "set" if parts.len() == 3 => {
                let key = parts[1].clone();
                let value = parts[2].clone();
                let node_clone = node.clone();

                //spawn a thread for each command 
                thread::spawn(move || {
                    node_clone.set(key, value);
                    println!("✅ Key set successfully.");
                }).join().unwrap();
            }

            "get" if parts.len() == 2 => {
                let key = parts[1].clone();
                let node_clone = node.clone();

                thread::spawn(move || {
                    match node_clone.get(key){
                        Some(val) => println!("📦 Value: {}", val),
                        None => println!("Npo key was found"),
                    }
                }).join().unwrap();
            }
            "delete" if parts.len() == 2 => {
                let key = parts[1].clone();
                let node_clone = node.clone();

                thread::spawn(move || {
                    if node_clone.delete(key){
                        println!("🗑️  Key deleted.");
                    }else {
                        println!("Key not found");
                    }
                }).join().unwrap();
            }
            "help" => {
                println!("Available commands:");
                println!("  set <key> <value>   - Set a key");
                println!("  get <key>           - Get a key");
                println!("  delete <key>        - Delete a key");
                println!("  exit                - Exit CLI");
            }
            "exit" => {
                println!("👋 Exiting DistKV");
                break;
            }
            _=> {
                println!("❓ Unknown command or wrong arguments. Try 'help'.")
            }
        }
    }
}