use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default)]
pub struct UssdR {
    pub failure: String,
    pub response: String
}
#[derive(Serialize, Deserialize, Default)]
pub struct Ussd {
    pub time: String,
    pub ussd: UssdR
}
#[derive(Default)]
pub struct UssdLog {
    pub ussd: Ussd,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for Ussd {}

impl UssdLog {
    pub async fn send(address: String, ussd_text: String) ->Result<UssdLog, String>{
        match Ussd::connect(address, CommandSend::UssdSend, &ussd_text).await{
            Ok((ussd,json))=>Ok(UssdLog { ussd, json}),
            Err(e)=> Err(e)
        }
    }
    pub async fn response(address: String) ->Result<UssdLog, String>{
        match Ussd::connect(address, CommandSend::UssdRespond, "").await{
            Ok((ussd,json))=>Ok(UssdLog { ussd, json}),
            Err(e)=> Err(e)
        }
    }
}