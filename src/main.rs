mod server;

use server::{serve};


fn main() {
    match serve("127.0.0.1", 8080) {
        Err(error) => println!("ERROR: Failed to start server ({})", error),
        Ok(()) => {}
    }
}
