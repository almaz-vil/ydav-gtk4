use std::io::{BufReader, Write};
use std::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::read_json_android::ReadJsonAndroid;

#[derive(Serialize, Deserialize)]
//Информация о вызовах
pub struct Phone{
    pub time: String,
    pub phone: String,
    pub status: String
}

#[derive(Serialize, Deserialize)]
pub struct Phones{
    pub time: String,
    pub phone: Vec<Phone>
}
pub struct PhoneLog{
    pub phones: Phones,
    pub json: String
}

impl ReadJsonAndroid for PhoneLog {}
impl PhoneLog{
    pub  fn connect(address: String)->Result<PhoneLog, String>{
        let phones = Phones{
            time:"".to_string(),
            phone: vec![]
        };
        let mut phone_log = PhoneLog{
            phones,
            json:"".to_string()
        };
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.write(b"PHONE\n").unwrap();
                let reader = BufReader::new(stream.try_clone().expect("error"));
                let str_json= match PhoneLog::read_json(reader){
                    Ok(d)=>d,
                    Err(e)=> return Err(String::from( format!("Ошибка чтения: {}", e)))
                };
                let deserialized_phones: Phones = match from_str(&str_json){
                    Ok(info)=>info,
                    Err(e)=> {
                        return Err(String::from( format!("Ошибка сериализации: {}", e)));}

                };
                phone_log.phones= deserialized_phones;
                phone_log.json=format!("{} \n", str_json, );
            }
            Err(e) => {
                return Err(String::from( format!("Ошибка соединения: {}", e)))
            }
        }
        Ok(phone_log)
    }
}