use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ResultStatus {
    pub result: String,
    pub time: String
}
#[derive(Serialize, Deserialize, Default)]
pub struct Status {
    pub sent: ResultStatus,
    pub delivery: ResultStatus
}
#[derive(Serialize, Deserialize, Default)]
pub struct StatusSMSOutput {
    pub time: String,
    pub status: Status
}
#[derive(Default)]
pub struct SmsOutputLog {
    pub status: StatusSMSOutput,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for StatusSMSOutput{}

impl SmsOutputLog {
    pub async fn send(address: String, sms_output_param: SmsOutputParam ) ->Result<SmsOutputLog, String>{
        match StatusSMSOutput::connect(address, CommandSend::SmsOutput, &sms_output_param.json()).await{
            Ok((status,json))=>Ok(SmsOutputLog {status, json}),
            Err(e)=> Err(e)
        }
    }
    pub async fn status(address: String, id: &str) ->Result<SmsOutputLog, String>{
        match StatusSMSOutput::connect(address, CommandSend::SmsOutputStatus, id).await{
            Ok((status,json))=>Ok(SmsOutputLog {status, json}),
            Err(e)=> Err(e)
        }
    }
}

//Для отправки
#[derive(Serialize)]
pub struct SmsOutputParam {
    pub id: String,
    pub phone: String,
    pub text: String
}

impl SmsOutputParam {
    pub fn json(self)->String{
        serde_json::to_string(&self).unwrap()
    }

}

