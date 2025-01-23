use serde::Serialize;

pub enum CommandSend{
    INFO,
    PHONE,
    CONTACT,
    SMS_INPUT,
}

#[derive(Serialize)]
struct Command{
    command: String
}


impl CommandSend {
    pub fn str_b(self)->String{
        let mut c = match self {
            CommandSend::INFO => {
                Command{ command: "INFO".to_string()}.json()}
            CommandSend::PHONE => {
                Command{ command: "PHONE".to_string()}.json()}
            CommandSend::CONTACT => {
                Command{ command: "CONTACT".to_string()}.json()}
            CommandSend::SMS_INPUT => {
                Command{ command: "SMS_INPUT".to_string()}.json()}
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