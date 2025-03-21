use std::collections::HashMap;
use gdk4::glib::home_dir;
use sqlite::{Connection, State, Error};
pub struct Config{
    pub param :HashMap<String, String>
}

impl Config{

    pub fn new()->Config
    {
        Config{ param : HashMap::with_capacity(2)}
    }

    pub fn load(&mut self)->Result<(),Error>
    {
        let connection = Self::sql_connection();
        let query = "SELECT name, value FROM config";
        let mut statement = connection.prepare(query)?;
        while let Ok(State::Row) = statement.next() {
            let name = statement.read::<String,_>("name")?;
            let value = statement.read::<String,_>("value")?;
            self.param.insert(name, value);
        }
        Ok(())
    }
    pub fn save(&self)->()
    {
        let mut sql ="BEGIN TRANSACTION;".to_string();
        for param in &self.param{
                sql=sql+format!("UPDATE config SET value='{}' WHERE name='{}';", param.1, param.0).as_str();
        }
        sql=sql+"COMMIT;";
        Self::sql_execute(sql);
    }

    pub fn sql_connection()->Connection{
        let home =  home_dir().join("ydav2024-data");
        sqlite::open(&home).expect(format!("не смог открыть базу данных {:?}", &home).as_str())
    }
    pub(crate) fn sql_execute(sql: String){
        let connection = Self::sql_connection();
        if let Err(e)=connection.execute(&sql){
            println!("Ошибка {} тут {}", e, sql);
        }
    }


}