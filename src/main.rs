use std::{io::{Read, Write},net::{TcpListener,TcpStream}};
use std::thread;
use std::str;

fn main() {
    println!("Start server! ");

    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).unwrap();
    for stream in listener.incoming(){
        println!(" new connection ------");

        match stream {
            Ok(stream) => {
                thread::spawn(move ||{
                    handle_client(stream);
                });
            }
            Err(e)=>{
                panic!(" connection err {:?}!", e)
            }
        }

    }
    drop(listener);

}


fn handle_client(mut stream:TcpStream){
    println!(" DEBUG:: received client request ");
    let mut buf = [0;512];

    loop {
        let bytes_read = stream.read(&mut buf).expect(" read req failed. break");

        println!(" byte size :{}",bytes_read);
        println!(" content read:{} ", str::from_utf8(&buf).unwrap());
        if bytes_read == 0 {
            println!( " read 0 bytes and quit");
            break;
        }


        let s = match str::from_utf8(&buf[..bytes_read]){
            Ok(v)=>v,
            Err(e)=>{
                stream.write(b"Need utf-8 sequence").unwrap();

                continue;
            },
        };

        if s.len() >= 3 && s[0..3] == "bye".to_string(){
            stream.write(b"By by. \n").unwrap();
            break;

        }

        stream.write(&buf[..bytes_read]).unwrap();
    }
}
