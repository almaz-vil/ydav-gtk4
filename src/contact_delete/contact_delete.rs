use crate::read_json_android::ReadJsonAndroid;
use crate::send_command_android::CommandSend;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct ContactsDelete {
    pub time: String,
    pub count: usize
}
#[derive(Default)]
pub struct ContactDeleteLog {
    pub contacts: ContactsDelete,
    #[allow(dead_code)]
    pub json: String
}

impl ReadJsonAndroid for ContactsDelete {}
impl ContactDeleteLog {
    pub  async fn connect(address: String, param: &str)->Result<ContactDeleteLog, String>{
        match ContactsDelete::connect(address, CommandSend::DelContact, param).await{
            Ok((contacts,json))=> Ok(ContactDeleteLog { contacts, json}),
            Err(e)=> Err(e)
        }
    }
}