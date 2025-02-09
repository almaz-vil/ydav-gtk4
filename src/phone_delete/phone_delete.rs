use serde::{Deserialize, Serialize};
use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;

#[derive(Serialize, Deserialize, Default)]
//Структура для сериализации полученного json плюс время с устройства
pub struct PhoneCount {
    pub time: String,
    pub phone: usize
}
pub struct PhoneDelete {
    pub phone: PhoneCount,
    pub json: String
}

impl ReadJsonAndroid for PhoneCount {}


impl PhoneDelete {
    pub fn connect(address: String, param: &str)->Result<PhoneDelete, String>{
        match PhoneCount::connect(address, CommandSend::DelPhone, param){
            Ok((phone,json))=>Ok(PhoneDelete {phone, json}),
            Err(e)=> Err(e)
        }
    }
}