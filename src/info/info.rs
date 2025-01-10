use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::{self, BufRead, BufReader, Write};
use serde_json::{from_str};

#[derive(Serialize, Deserialize)]
///Информация о параметрах активной мобильной связи
pub struct Signal{
    pub rsrp: i64,
    pub rsrq: i64,
    pub rssi: i64,
    pub network_type: String,
    pub sim_county_iso: String,
    pub sim_operator: String,
    pub sim_operator_name: String
}
#[derive(Serialize, Deserialize)]
///Информация о параметрах батареи
pub struct Battery{
    pub temperature: f64,
    pub level: f64,
    pub status: String
}
#[derive(Serialize, Deserialize)]
//Информация о вызовах
pub struct Phone{
    pub time: String,
    pub phone: String,
    pub status: String
}
#[derive(Serialize, Deserialize)]
//Структура для сериализации полученного json плюс время с устройства
pub struct Info{
    pub time: String,
    pub battery: Battery,
    pub signal: Signal,
    pub phone: Vec<Phone>
}
pub struct InfoLog{
    pub info: Info,
    pub json: String
}

impl InfoLog{
    pub  fn connect(address: String)->Result<InfoLog, String>{
        let signal = Signal{
            rsrp:-1,
            rsrq:-1,
            rssi:-1,
            network_type:"".to_string(),
            sim_county_iso:"".to_string(),
            sim_operator:"".to_string(),
            sim_operator_name:"".to_string()
        };
        let battery = Battery{
            temperature:0.0,
            level:0.0,
            status:"".to_string()
        };
        let info = Info{
            time: "".to_string(),
            battery,
            signal,
            phone: vec![]
        };
        let mut info_log = InfoLog{
            info,
            json:"".to_string()
        };
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.write(b"Hello from client!\n").unwrap();
                let reader = BufReader::new(stream.try_clone().expect("error"));
                let str_json= match InfoLog::read_json(reader){
                    Ok(d)=>d,
                    Err(e)=> return Err(String::from( format!("Ошибка чтения: {}", e)))
                };
                let deserialized_info: Info= match from_str(&str_json){
                    Ok(info)=>info,
                    Err(e)=> {
                        println!("{}", str_json);
                        return Err(String::from( format!("Ошибка сериализации: {}", e)));}
                
                };
                info_log.info=deserialized_info;
                info_log.json=format!("{} \n", str_json, );
            }
            Err(e) => {
                return Err(String::from( format!("Ошибка соединения: {}", e)))
            }
        }
        Ok(info_log)
    }



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

#[derive(Clone)]
pub struct Level(pub f64);
impl Level {
    pub fn get_str(&mut self, tek: f64)->String{
        if (self.0==f64::default()) || (self.0==tek) {
            self.0=tek;
            return format!("{:.1}", tek);
        }
        if self.0 < tek {
            format!("▲{:.1}", tek)
        } else {
            format!("▼{:.1}", tek)
        }
    }
}
