use std::cell::RefCell;
use gtk4::glib as glib;
use glib::prelude::*;
use glib::subclass::prelude::*;

#[derive(Default)]
pub struct ContactObject {
    id_na_anrdoid: RefCell<Option<String>>,
    id: RefCell<i64>,
    name: RefCell<Option<String>>,
    phone: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ContactObject {
    const NAME: &'static str = "contact_object";
    type Type = super::ContactObject;
    type ParentType = glib::Object;
    type Interfaces = ();
}

impl ObjectImpl for ContactObject {
    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("idnaandroid")
                    .build(),
                glib::ParamSpecInt64::builder("id")
                    .build(),
                glib::ParamSpecString::builder("name")
                    .build(),
                glib::ParamSpecString::builder("phone")
                    .build(),
            ]
        })
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "idnaandroid" => {
                let id = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.id_na_anrdoid.replace(id);
            },
            "id" => {
                let id = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.id.replace(id);
            },
            "name" => {
                let name = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.name.replace(name);
            },
            "phone" => {
                let phone = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.phone.replace(phone);
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "idnaandroid" => self.id_na_anrdoid.borrow().to_value(),
            "id" => self.id.borrow().to_value(),
            "name" => self.name.borrow().to_value(),
            "phone" => self.phone.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self) {
        self.parent_constructed();
    }
}