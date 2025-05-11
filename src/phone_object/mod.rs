use gdk4::glib;
use gdk4::glib::Object;
use gdk4::prelude::{Cast, CastNone, ObjectExt};
use gtk4::{Justification, ListItem};
use gtk4::prelude::ListItemExt;

mod imp;

// Optionally, define a wrapper type to make it more ergonomic to use from Rust
glib::wrapper! {
    pub struct PhoneObject(ObjectSubclass<imp::PhoneObject>);
}

impl PhoneObject {
    // Create an object instance of the new type.
    pub fn new() -> Self {
        Object::new()
    }
    pub fn factorion(self, list_item: &Object, property_name: &str)
    {
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk4::Label>()
            .expect("The child has to be a `Label`.");
        label.set_justify(Justification::Left);
        label.set_label(self.property::<String>(property_name).as_str());
    }
}
