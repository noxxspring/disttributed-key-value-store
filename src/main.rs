mod node;
mod cli;

use cli::run_cli;

fn main() {
    println!("\n🚀 Starting DistKV Node...");
    run_cli();
}
