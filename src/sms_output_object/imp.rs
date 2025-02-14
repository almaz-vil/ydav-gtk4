use std::cell::RefCell;
use gtk4::glib as glib;
use glib::prelude::*;
use glib::subclass::prelude::*;

#[derive(Default)]
pub struct SmsOutputObject {
    id: RefCell<Option<String>>,
    phone: RefCell<Option<String>>,
    text: RefCell<Option<String>>,
    sent: RefCell<Option<String>>,
    senttime: RefCell<Option<String>>,
    delivery: RefCell<Option<String>>,
    deliverytime: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for SmsOutputObject {
    const NAME: &'static str = "sms_output_object";
    type Type = super::SmsOutputObject;
    type ParentType = glib::Object;
    type Interfaces = ();
}

impl ObjectImpl for SmsOutputObject {
    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("id")
                    .build(),
                glib::ParamSpecString::builder("phone")
                    .build(),
                glib::ParamSpecString::builder("text")
                    .build(),
                glib::ParamSpecString::builder("sent")
                    .build(),
                glib::ParamSpecString::builder("senttime")
                    .build(),
                glib::ParamSpecString::builder("delivery")
                    .build(),
                glib::ParamSpecString::builder("deliverytime")
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
            },"text" => {
                let text = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.text.replace(text);
            },
            "sent" => {
                let sent = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.sent.replace(sent);
            },
            "senttime" => {
                let sent_time = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.senttime.replace(sent_time);
            },
            "delivery" => {
                let delivery = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.delivery.replace(delivery);
            },
            "deliverytime" => {
                let delivery_time = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.deliverytime.replace(delivery_time);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "id" => self.id.borrow().to_value(),
            "phone" => self.phone.borrow().to_value(),
            "text" => self.text.borrow().to_value(),
            "sent" => self.sent.borrow().to_value(),
            "senttime" => self.senttime.borrow().to_value(),
            "delivery" => self.delivery.borrow().to_value(),
            "deliverytime" => self.deliverytime.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self) {
        self.parent_constructed();
    }
}