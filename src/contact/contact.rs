use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
//Информация о вызовах
pub struct Contact{
    pub name: String,
    pub phone: Vec<String>
}

#[derive(Serialize, Deserialize, Default)]
pub struct Contacts {
    pub time: String,
    pub contact: Vec<Contact>
}
#[derive(Default)]
pub struct ContactLog {
    pub contacts: Contacts,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for Contacts {}
impl ContactLog {
    pub  fn connect(address: String)->Result<ContactLog, String>{
        match Contacts::connect(address, CommandSend::CONTACT, ""){
            Ok((contacts,json))=> Ok(ContactLog { contacts, json}),
            Err(e)=> Err(e)
        }
    }
}