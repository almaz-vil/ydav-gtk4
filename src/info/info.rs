use serde::{Deserialize, Serialize};
use crate::read_json_android::{ReadJsonAndroid};
use crate::send_command_android::CommandSend;

#[derive(Serialize, Deserialize, Default)]
///Информация о параметрах активной мобильной связи
pub struct Signal{
    pub signal_param: String,
    pub network_type: String,
    pub sim_county_iso: String,
    pub sim_operator: String,
    pub sim_operator_name: String
}
#[derive(Serialize, Deserialize, Default)]
///Информация о параметрах батареи
pub struct Battery{
    pub temperature: f64,
    pub level: f64,
    pub status: String,
    pub charge: String
}

#[derive(Serialize, Deserialize, Default)]
//Структура для сериализации полученного json плюс время с устройства
pub struct Phones {
    pub time: String,
    pub battery: Battery,
    pub signal: Signal,
    pub sms: usize,
    pub phone: usize
}
pub struct InfoLog{
    pub info: Phones,
    pub json: String
}

impl ReadJsonAndroid for Phones{}
impl InfoLog{
    pub async fn connect(address: String)->Result<InfoLog, String>{
        match Phones::connect(address, CommandSend::INFO, "").await{
            Ok((info,json))=> Ok(InfoLog{info, json }),
            Err(e)=> Err(e)
        }
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
