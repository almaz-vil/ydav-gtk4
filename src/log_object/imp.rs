use std::cell::RefCell;
use gtk4::glib as glib;
use glib::prelude::*;
use glib::subclass::prelude::*;

#[derive(Default)]
pub struct LogObject {
    log: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for LogObject {
    const NAME: &'static str = "log_object";
    type Type = super::LogObject;
    type ParentType = glib::Object;
    type Interfaces = ();
}

impl ObjectImpl for LogObject {
    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("log")
                    .build(),
            ]
        })
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "log" => {
                let log = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.log.replace(log);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "log" => self.log.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self) {
        self.parent_constructed();
    }
}