use std::cell::RefCell;
use gtk4::glib as glib;
use glib::prelude::*;
use glib::subclass::prelude::*;

#[derive(Default)]
pub struct SmsInputObject {
    id: RefCell<Option<String>>,
    time: RefCell<Option<String>>,
    phone: RefCell<Option<String>>,
    body: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SmsInputObject {
    const NAME: &'static str = "sms_input_object";
    type Type = super::SmsInputObject;
    type ParentType = glib::Object;
    type Interfaces = ();
}

impl ObjectImpl for SmsInputObject {
    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("id")
                    .build(),
                glib::ParamSpecString::builder("time")
                    .build(),
                glib::ParamSpecString::builder("phone")
                    .build(),
                glib::ParamSpecString::builder("body")
                    .build(),
            ]
        })
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "id" => {
                let id = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.id.replace(id);
            },
            "phone" => {
                let phone = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.phone.replace(phone);
            },"time" => {
                let time = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.time.replace(time);
            },
            "body" => {
                let body = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.body.replace(body);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "id" => self.id.borrow().to_value(),
            "phone" => self.phone.borrow().to_value(),
            "time" => self.time.borrow().to_value(),
            "body" => self.body.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self) {
        self.parent_constructed();
    }
}