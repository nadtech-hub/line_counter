pub mod file_processor;
pub mod server;
pub mod test;

use server::start_server;

fn main() {
    start_server("127.0.0.1:1080");
}
