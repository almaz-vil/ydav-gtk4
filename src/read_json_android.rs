use crate::send_command_android::CommandSend;
use async_std::net::TcpStream;
use async_std::prelude::*;
use serde::Deserialize;
use serde_json::from_str;

pub trait ReadJsonAndroid{
      async fn connect<T: Default+ReadJsonAndroid+for<'a>Deserialize<'a>>(address: String, com: CommandSend, param: &str)->Result<(T, String), String>{
          match TcpStream::connect(address).await {
            Ok(mut stream) => {
                if let Err(e)=stream.write_all(com.str_b(param).as_bytes()).await{
                    return Err(String::from( format!("Ошибка отправки: {}", e)))
                };
                let mut str_json = String::new();
                if let Err(e)=stream.read_to_string(&mut str_json).await{
                    return Err(String::from( format!("Ошибка чтения: {}", e)))
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
}