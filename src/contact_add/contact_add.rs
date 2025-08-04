use serde::{Deserialize, Serialize};
use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;

#[derive(Serialize, Deserialize, Default)]
pub struct ContactsAdd {
    pub time: String,
    pub id: String
}
#[derive(Default)]
pub struct ContactAddLog {
    pub contacts: ContactsAdd,
    pub param: ContactNewParam,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for ContactsAdd {}
impl ContactAddLog {
    pub  async fn connect(address: String, param: ContactNewParam)->Result<ContactAddLog, String>{
        let param_str=param.json();
        match ContactsAdd::connect(address, CommandSend::AddContact, &param_str).await{
            Ok((contacts,json))=> Ok(ContactAddLog { contacts, param, json}),
            Err(e)=> Err(e)
        }
    }
}

#[derive(Serialize, Default)]
pub struct ContactNewParam{
   pub name: String,
   pub phone: String,
}

impl ContactNewParam {
    pub fn json(&self)->String{
        serde_json::to_string(&self).unwrap()
    }
}
