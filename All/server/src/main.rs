use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    // Handle multiple access stream
    let mut buf = [0;512];
    for _ in 0..1000{
        // let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buf)?;
        // sender stream in a mutable variable
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        println!("Messages recieved:{}",String::from_utf8_lossy(&buf));
        thread::sleep(time::Duration::from_secs(1));  
    }
    Ok(())
}

fn main() -> io::Result<()>{
    let receiver_listener = TcpListener::bind("127.0.0.1:6000").expect("Failed and bind with the transmitter");
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // listen to incoming connections messages and bind them to a sever socket address.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // let the receiver connect with the sender
        let handle = thread::spawn(move || {
            //receiver failed to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });
        
        // Push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // return each single value Output contained in the heap
        handle.join().unwrap();
    }
    Ok(())
}
