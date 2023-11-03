#[cfg(feature = "single-thread")]
use tcp::single_thread::server;

#[cfg(feature = "thread-pool")]
use tcp::thread_pool::server;

fn main() -> Result<(), std::io::Error> {
    server::run()
}