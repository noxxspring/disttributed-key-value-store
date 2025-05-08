use std::{ io::{self, Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}};
use crate::node::KVStore;

/// Starts a TCP server that listens for RPC commands (SET, GET, DELETE).
/// Each command must be in the format: `SET key value`, `GET key`, or `DELETE key`.
pub fn start_server(addr: &str, store: Arc<Mutex<KVStore>>) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("🌐 Server started at {}", addr);

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store_clone = Arc::clone(&store);
                std::thread::spawn(move || {
                    // Handle client communication in a separate thread
                    if let Err(e) = handle_client(stream, store_clone) {
                        eprintln!("❌ Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("❌ Error accepting client: {}", e);
            }
        }
    }

    Ok(())
}

/// Handles an individual TCP client connection and parses basic RPC commands.
fn handle_client(mut stream: TcpStream, store: Arc<Mutex<KVStore>>) -> io::Result<()> {
    let mut buffer = vec![0; 1024]; // buffer to read incoming data

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break; // Client disconnected, break the loop
            }
            Ok(bytes_read) => {
               // Only process the data up to bytes_read
               let  request = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
               // Print the raw request to debug what the client is sending
               println!("Raw request: {}", request);


                let parts: Vec<&str> = request.trim().splitn(3, ' ').collect();
                let command = parts[0].to_uppercase();

                match command.as_str() {
                    "SET" => {
                        if parts.len() < 3 {
                            let _ = stream.write("⚠️ Usage: SET <key> <value>\n".as_bytes());
                        } else {
                            let key = parts[1].to_string();
                            let value = parts[2].to_string();
                            let mut store = store.lock().unwrap();
                            store.set(key, value);
                            let _ = stream.write("✅ Key set successfully.\n".as_bytes());
                        }
                    }
                
                    "GET" => {
                        if parts.len() != 2 {
                            let _ = stream.write("⚠️ Usage: GET <key>\n".as_bytes());
                        } else {
                            let key = parts[1];
                            let store = store.lock().unwrap();
                            match store.get(key) {
                                Some(value) => {
                                    let _ = stream.write(format!("🔍 {} = {}\n", key, value).as_bytes());
                                }
                                None => {
                                    let _ = stream.write(format!("❌ Key '{}' not found.\n", key).as_bytes());
                                }
                            }
                        }
                    }
                    "DELETE" => {
                        if parts.len() != 2 {
                            let _ = stream.write("⚠️ Usage: DELETE <key>\n".as_bytes());
                        } else {
                            let key = parts[1];
                            let mut store = store.lock().unwrap();
                            if store.delete(key) {
                                let _ = stream.write("🗑️ Key deleted.\n".as_bytes());
                            } else {
                                let _ = stream.write("❌ Key not found.\n".as_bytes());
                            }
                        }
                    }

                    "UPDATE" => {
                        if parts.len() != 3 {
                            let _ = stream.write("⚠️ Usage: UPDATE <key> <new_value>\n".as_bytes());
                        } else {
                            let key = parts[1].to_string();
                            let new_value = parts[2].to_string();
                            let mut store = store.lock().unwrap();
                            match store.update(key, new_value) {
                                Some(old_value) => {
                                    let _ = stream.write(format!("✅ Key updated. Old value: {}\n", old_value).as_bytes());
                                }
                                None => {
                                    let _ = stream.write("❌ Key not found for update.\n".as_bytes());
                                }
                            }
                        }
                    }

                   "LIST" => {
                        let store = store.lock().unwrap();
                        let keys = store.keys();
                        if keys.is_empty() {
                            let _ = stream.write("⚠️ No keys stored.\n".as_bytes());
                        } else {
                            let keys_list = keys.join(", ");
                            let _ = stream.write(format!("🗂️ Stored keys: {}\n", keys_list).as_bytes());
                        }
                    }
                    "CLEAR" => {
                        let mut store = store.lock().unwrap();
                        store.clear();
                        let _ = stream.write("🧹 Store cleared.\n".as_bytes());
                    }

                    "HELP" => {
                        let _ = stream.write("🆘 Available commands:\nSET <key> <value>\nGET <key>\nDELETE <key>\n".as_bytes());
                    }

                
                "EXIT" => {
                    println!("👋 Exiting DistKV CLI.");
                    break;
                }

                    _ => {
                        let _ = stream.write("❓ Unknown command. Type 'HELP' for available commands.\n".as_bytes());
                    }
                }
            }
            Err(_) => {
                eprintln!("❌ Error reading from client.");
                break; // Disconnect client on error
            }
        }
    }

    Ok(()) // Return Ok to indicate the function completed successfully
}