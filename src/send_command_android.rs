use serde::Serialize;

pub enum CommandSend{
    INFO,
    PHONE,
    CONTACT,
    SmsInput,
    DelSmsInput,
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
            CommandSend::SmsInput => {
                Command{ command: "SMS_INPUT".to_string(), param: param.to_string()}.json()}
            CommandSend::DelSmsInput => {
                Command{ command: "DELETE_SMS_INPUT".to_string(), param: param.to_string()}.json()}
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