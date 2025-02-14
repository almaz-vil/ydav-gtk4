use gdk4::glib;
use gdk4::glib::Object;
use gdk4::prelude::{Cast, CastNone, ObjectExt};
use gtk4::{Justification, ListItem};
use gtk4::prelude::{ListItemExt, WidgetExt};
mod imp;

// Optionally, define a wrapper type to make it more ergonomic to use from Rust
glib::wrapper! {
    pub struct SmsOutputObject(ObjectSubclass<imp::SmsOutputObject>);
}

impl SmsOutputObject {
    // Create an object instance of the new type.
    pub fn new() -> Self {
        glib::Object::new()
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
    pub fn factorion_dy_panel(self, list_item: &Object, property_name_1: &str, property_name_2: &str)
    {
        let panel = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk4::Box>()
            .expect("The child has to be a `Label`.");
        let binding =  panel
            .first_child()
            .expect("error");
        let label_1 = binding
            .downcast_ref::<gtk4::Label>()
            .expect("error label");
        label_1.set_justify(Justification::Left);
        label_1.set_label(self.property::<String>(property_name_1).as_str());
        let binding_2 = panel
            .last_child()
            .expect("error");
        let label_2 = binding_2
            .downcast_ref::<gtk4::Label>()
            .expect("error label");
        label_2.set_justify(Justification::Left);
        label_2.set_label(self.property::<String>(property_name_2).as_str());

    }

}
