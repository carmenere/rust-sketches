#[cfg(feature = "single-thread")]
use tcp::single_thread::client;

fn main() -> Result<(), std::io::Error> {
    client::run()
}