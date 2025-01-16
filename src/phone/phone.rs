use crate::read_json_android::{CommandSend, ReadJsonAndroid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
//Информация о вызовах
pub struct Phone{
    pub time: String,
    pub phone: String,
    pub status: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct Phones{
    pub time: String,
    pub phone: Vec<Phone>
}
#[derive(Default)]
pub struct PhoneLog{
    pub phones: Phones,
    pub json: String
}

impl ReadJsonAndroid for Phones {}
impl PhoneLog{
    pub  fn connect(address: String)->Result<PhoneLog, String>{
        match Phones::connect(address, CommandSend::PHONE){
            Ok((phones,json))=> Ok(PhoneLog{phones, json}),
            Err(e)=> Err(e)
        }
    }
}