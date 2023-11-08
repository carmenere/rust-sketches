use std::error::Error;

#[cfg(feature = "single-thread")]
mod single_thread;

#[cfg(feature = "thread-pool")]
mod thread_pool;

fn main() -> Result<(), Box<dyn Error>> {
    println!("tcp-server.rs");

    #[cfg(feature = "thread-pool")]
    let _ = thread_pool::server::run()?;

    #[cfg(feature = "single-thread")]
    let _ = single_thread::server::run()?;

    Ok(())
}