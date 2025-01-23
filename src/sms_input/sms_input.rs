use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
//Информация о входящие СМС
pub struct Sms{
    pub id: String,
    pub time: String,
    pub phone: String,
    pub body: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct SmsInput {
    pub time: String,
    pub sms: Vec<Sms>
}
#[derive(Default)]
pub struct SmsInputLog {
    pub sms_input: SmsInput,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for SmsInput{}
impl SmsInputLog {
    pub fn connect(address: String)->Result<SmsInputLog, String>{
        match SmsInput::connect(address, CommandSend::SMS_INPUT){
            Ok((sms_input,json))=>Ok(SmsInputLog {sms_input, json}),
            Err(e)=> Err(e)
        }
    }
}