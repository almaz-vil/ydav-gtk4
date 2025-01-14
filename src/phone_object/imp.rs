use std::cell::RefCell;
use gtk4::glib as glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Variant;

// This is the struct containing all state carried with
// the new type. Generally this has to make use of
// interior mutability.
// If it implements the `Default` trait, then `Self::default()`
// will be called every time a new instance is created.
#[derive(Default)]
pub struct PhoneObject {
    time: RefCell<Option<String>>,
    phone: RefCell<Option<String>>,
    variant: RefCell<Option<Variant>>,
}

// ObjectSubclass is the trait that defines the new type and
// contains all information needed by the GObject type system,
// including the new type's name, parent type, etc.
// If you do not want to implement `Default`, you can provide
// a `new()` method.
#[glib::object_subclass]
impl ObjectSubclass for PhoneObject {
    // This type name must be unique per process.
    const NAME: &'static str = "phone_object";

    type Type = super::PhoneObject;

    // The parent type this one is inheriting from.
    // Optional, if not specified it defaults to `glib::Object`
    type ParentType = glib::Object;

    // Interfaces this type implements.
    // Optional, if not specified it defaults to `()`
    type Interfaces = ();
}

// Trait that is used to override virtual methods of glib::Object.
impl ObjectImpl for PhoneObject {
    // Called once in the very beginning to list all properties of this class.
    fn properties() -> &'static [glib::ParamSpec] {
        use std::sync::OnceLock;
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecString::builder("time")
                    .build(),
                glib::ParamSpecString::builder("phone")
                    .build(),
                glib::ParamSpecVariant::builder("variant", glib::VariantTy::ANY)
                    .build(),
            ]
        })
    }

    // Called whenever a property is set on this instance. The id
    // is the same as the index of the property in the PROPERTIES array.
    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "time" => {
                let time = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.time.replace(time);
            },
            "phone" => {
                let phone = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.phone.replace(phone);
            },
            "variant" => {
                let variant = value
                    .get()
                    .expect("type conformity checked by `Object::set_property`");
                self.variant.replace(variant);
            },
            _ => unimplemented!(),
        }
    }

    // Called whenever a property is retrieved from this instance. The id
    // is the same as the index of the property in the PROPERTIES array.
    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "time" => self.time.borrow().to_value(),
            "phone" => self.phone.borrow().to_value(),
            "variant" => self.variant.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    // Called right after construction of the instance.
    fn constructed(&self) {
        // Chain up to the parent type's implementation of this virtual
        // method.
        self.parent_constructed();

        // And here we could do our own initialization.
    }
}