use std::net::{TcpListener, TcpStream};
use std::thread;
// std::net::TcpStream implements Read and Write traits
use std::io::{Read, Write};
use std::error::Error;

use socket_api::thread_pool::ThreadPool;
use crate::thread_pool::pool;

pub fn run() -> Result<(), Box<dyn Error>> {
    let lsocket = TcpListener::bind("127.0.0.1:5555")?;

    let pool = pool::ThreadPool::new(8)?;

    for con in lsocket.incoming() { // incoming() returns an iterator over the connections being received on this listener.
        let mut stream = con?;

        println!("Got new connection: src ip: {:?}, src port: {:?}", stream.peer_addr()?.ip(), stream.peer_addr()?.port());

        let _ = pool.execute(move || {
            let mut buf = [0u8; 1024];

            let r = stream.read(&mut buf)?;
            println!("read {r} bytes");
            let r = stream.write(&mut buf)?;
            println!("send {r} bytes");
            Ok(())
        })?;
    }

    Ok(())
}