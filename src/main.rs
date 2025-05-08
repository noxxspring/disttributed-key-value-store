mod node;
mod cli;
mod network;

use std::thread;

use cli::run_cli;
use network::start_server;
use node::KVStore;
use std::sync::{Arc, Mutex};

fn main() {
    println!("\n🚀 Starting DistKV Node...");

     // Create shared KVStore wrapped in Arc<Mutex<>> so it can be shared across threads
     let store = Arc::new(Mutex::new(KVStore::new()));

     // Clone store for the TCP server
    let store_for_network = Arc::clone(&store);


    // Start TCP server in a background thread
    thread::spawn(move || {
        let addr = "0.0.0.0:4000";
        if let Err(e) = start_server(addr, store_for_network) {
            eprintln!("❌ Failed to start network server: {}", e);
    }
    });

    run_cli(store);
}
