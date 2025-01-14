use gdk4::glib;
mod imp;

// Optionally, define a wrapper type to make it more ergonomic to use from Rust
glib::wrapper! {
    pub struct PhoneObject(ObjectSubclass<imp::PhoneObject>);
}

impl PhoneObject {
    // Create an object instance of the new type.
    pub fn new() -> Self {
        glib::Object::new()
    }
}