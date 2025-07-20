use serde::Serialize;

pub enum CommandSend{
    INFO,
    PHONE,
    CONTACT,
    DelContact,
    SmsInput,
    DelSmsInput,
    DelPhone,
    SmsOutput,
    SmsOutputStatus,
    UssdSend,
    UssdRespond,
}

#[derive(Serialize)]
struct Command{
    command: String,
    param: String
}


impl CommandSend {
    pub fn str_b(self, param: &str)->String{
        let mut c = match self {
            CommandSend::INFO => {
                Command{ command: "INFO".to_string(), param: param.to_string()}.json()}
            CommandSend::PHONE => {
                Command{ command: "PHONE".to_string(), param: param.to_string()}.json()}
            CommandSend::CONTACT => {
                Command{ command: "CONTACT".to_string(), param: param.to_string()}.json()}
            CommandSend::DelContact => {
                Command{ command: "DELETE_CONTACT".to_string(), param: param.to_string()}.json()}
            CommandSend::SmsInput => {
                Command{ command: "SMS_INPUT".to_string(), param: param.to_string()}.json()}
            CommandSend::DelSmsInput => {
                Command{ command: "DELETE_SMS_INPUT".to_string(), param: param.to_string()}.json()}
            CommandSend::DelPhone => {
                Command{ command: "DELETE_PHONE".to_string(), param: param.to_string()}.json()}
            CommandSend::SmsOutput => {
                Command{ command: "SMS_OUTPUT".to_string(), param: param.to_string()}.json()}
            CommandSend::SmsOutputStatus => {
                Command{ command: "SMS_OUTPUT_STATUS".to_string(), param: param.to_string()}.json()}
            CommandSend::UssdSend => {
                Command{ command: "USSD_SEND".to_string(), param: param.to_string()}.json()}
            CommandSend::UssdRespond => {
                Command{ command: "USSD_RESPOND".to_string(), param: param.to_string()}.json()}
        };
       c.push_str("\n");
        c
    }
}

impl Command {
    fn json(self)->String{
        serde_json::to_string(&self).unwrap()
    }
}