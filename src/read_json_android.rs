use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use serde::Deserialize;
use serde_json::from_str;
use crate::send_command_android::CommandSend;

pub trait ReadJsonAndroid{
      fn connect<T: Default+ReadJsonAndroid+for<'a>Deserialize<'a>>(address: String, com: CommandSend)->Result<(T, String), String>{
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.write(com.str_b().as_bytes()).unwrap();
                //stream.write(b"INFO\n").unwrap();
                let reader = BufReader::new(stream.try_clone().expect("error"));
                let str_json= match T::read_json(reader){
                    Ok(d)=>d,
                    Err(e)=> return Err(String::from( format!("Ошибка чтения: {}", e)))
                };
                match from_str(&str_json) {
                    Ok(info) => {
                       Ok((info, str_json))
                    }
                    Err(e) => {
                        Err(String::from(format!("Ошибка сериализации: {}", e)))
                    }
                }
            }
            Err(e) => {
               Err(String::from( format!("Ошибка соединения: {}", e)))
            }
        }
    }
    fn read_json(mut rad: BufReader<TcpStream>)-> Result<String, String>{
        let mut res_line = String::new();

        // Индикатор того, что хедеры были прочитаны
        loop {
            let mut buf_line = String::new();
            match rad.read_line(&mut buf_line) {
                Err(e) => return Err(String::from( format!("Ошибка чтения: {}", e))),
                Ok(0) =>  return Err(String::from( "Ошибка чтения: EOF")),
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