use std::io;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub trait ReadJsonAndroid{
    fn read_json(mut rad: BufReader<TcpStream>)-> io::Result<String>{
        let mut res_line = String::new();

        // Индикатор того, что хедеры были прочитаны
        loop {
            let mut buf_line = String::new();
            match rad.read_line(&mut buf_line) {
                Err(e) => panic!("Got an error: {}", e),
                Ok(0) => panic!("Got an error: "),
                Ok(_) => (),
            };

            res_line.push_str(&buf_line);
            if res_line.contains("}\n"){
                break;
            }
        }
        Ok(res_line)

    }
}