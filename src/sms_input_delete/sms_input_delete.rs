use serde::{Deserialize, Serialize};
use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;

#[derive(Serialize, Deserialize, Default)]
//Структура для сериализации полученного json плюс время с устройства
pub struct SmsCount {
    pub time: String,
    pub sms: usize
}
pub struct SmsInputDelete{
    pub sms: SmsCount,
    pub json: String
}

impl ReadJsonAndroid for SmsCount {}


impl SmsInputDelete{
    pub fn connect(address: String, param: &str)->Result<SmsInputDelete, String>{
        match SmsCount::connect(address, CommandSend::DelSmsInput, param){
            Ok((sms,json))=>Ok(SmsInputDelete {sms, json}),
            Err(e)=> Err(e)
        }
    }
}