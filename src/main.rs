mod info;
mod phone;
mod phone_object;
mod read_json_android;

use crate::info::{Info, Level};
use gdk4 as gdk;
use gdk4::glib::{clone, ControlFlow};
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk4 as gtk;
use gtk::{ColumnViewColumn, ListItem};
use std::path::Path;


use crate::phone::Phone;

fn main() {
    let app = Application::builder()
        .application_id("ru.Dimon.Ydav-gtk")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let class_info=["info"];
    let gtk_box_horizontal =gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Fill)
            .build();
    let label_battery_level = gtk::Label::new(Some("_"));
    label_battery_level.set_css_classes(&class_info);
    label_battery_level.set_tooltip_text(Some("Уровень заряда аккумулятора"));
    let label_battery_temperature = gtk::Label::new(Some("_"));
    label_battery_temperature.set_css_classes(&class_info);
    label_battery_temperature.set_tooltip_text(Some("Температура аккумулятора"));
    let label_battery_status = gtk::Label::new(Some("_"));
    label_battery_status.set_css_classes(&class_info);

    let label_network_type = gtk::Label::new(Some("_"));
    label_network_type.set_css_classes(&class_info);
    label_network_type.set_tooltip_text(Some("Тип связи"));
    let label_sim_county_iso = gtk::Label::new(Some("_"));
    label_sim_county_iso.set_css_classes(&class_info);
    label_sim_county_iso.set_tooltip_text(Some("Код страны СИМ"));
    let label_sim_operator = gtk::Label::new(Some("_"));
    label_sim_operator.set_css_classes(&class_info);
    label_sim_operator.set_tooltip_text(Some("Код оператора СИМ"));
    let label_sim_operator_name = gtk::Label::new(Some("_"));
    label_sim_operator_name.set_css_classes(&class_info);
    label_sim_operator_name.set_tooltip_text(Some("Оператор СИМ"));
    gtk_box_horizontal.append(&label_battery_level);
    gtk_box_horizontal.append(&label_battery_temperature);
    gtk_box_horizontal.append(&label_battery_status);
    let gtk_box_horizontal2 =gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Fill)
        .build();
    gtk_box_horizontal.set_css_classes(&["panel"]);
    gtk_box_horizontal2.set_css_classes(&["panel"]);
    gtk_box_horizontal2.append(&label_network_type);
    gtk_box_horizontal2.append(&label_sim_operator_name);
    gtk_box_horizontal2.append(&label_sim_operator);
    gtk_box_horizontal2.append(&label_sim_county_iso);
    let label_rsrp = gtk::Label::new(Some("_"));
    label_rsrp.set_css_classes(&class_info);
    gtk_box_horizontal2.append(&label_rsrp);
    let label_rsrq = gtk::Label::new(Some("_"));
    label_rsrq.set_css_classes(&class_info);
    gtk_box_horizontal2.append(&label_rsrq);
    let label_rssi = gtk::Label::new(Some("_"));
    label_rssi.set_css_classes(&class_info);
    gtk_box_horizontal2.append(&label_rssi);
    let edit_ip_address = gtk::Entry::new();
    edit_ip_address.set_text("192.168.1.91:38300");
    edit_ip_address.set_widget_name("edit_ip");
    
    let gtk_box_g=gtk::Box::builder()
       .orientation(gtk::Orientation::Vertical)
      //  .valign(gtk::Align::Fill)
       .build();
    let stack = gtk::Stack::new();

    let times = gtk::Label::new(Some(""));
    let button_stop_info = gtk::Button::new();
    button_stop_info.set_label("▶");
    let status = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    status.set_widget_name("statusbar");
    status.append(&times);
    status.append(&button_stop_info);
    gtk_box_g.append(&gtk_box_horizontal);
    gtk_box_g.append(&gtk_box_horizontal2);
    gtk_box_g.append(&edit_ip_address);
    gtk_box_g.set_homogeneous(false);
    gtk_box_g.set_widget_name("panel");

    gtk_box_horizontal.set_visible(false);
    gtk_box_horizontal2.set_visible(false);

    let listbox = gtk::ListBox::new();
    let flex_box_list=gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let button_list_get=gtk::Button::builder()
        .label("Запрос звонков")
        .build();
    flex_box_list.append(&button_list_get);
    let scrolled_list=gtk::ScrolledWindow::builder()
        .child(&listbox)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    flex_box_list.append(&scrolled_list);
    //*********
    let factory_phone = gtk::SignalListItemFactory::new();
    factory_phone.connect_setup( move |_, list_item| {
        let label = gtk::Label::builder().build();
        label.set_label("?o");
        list_item
            .downcast_ref::<ListItem>()
            .expect("error")
            .set_child(Some(&label));
    });
    factory_phone.connect_bind(move |_, list_item| {
        let data = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<phone_object::PhoneObject>()
            .expect("The item has to be an `IntegerObject`.");
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("The child has to be a `Label`.");
        label.set_label(data.property::<String>("phone").as_str());

    });

    let column_phone=ColumnViewColumn::new(Some("Телефонный номер"),Some(factory_phone));
    let factory_time = gtk::SignalListItemFactory::new();
    factory_time.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        list_item
            .downcast_ref::<ListItem>()
            .expect("error")
            .set_child(Some(&label));
    });
    factory_time.connect_bind(move |_, list_item| {
        let data = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<phone_object::PhoneObject>()
            .expect("The item has to be an `IntegerObject`.");
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("The child has to be a `Label`.");

        label.set_label(data.property::<String>("time").as_str());
    });
    let column_time =ColumnViewColumn::new(Some("Время"), Some(factory_time));

    let model_phone_object: gtk::gio::ListStore = gtk::gio::ListStore::new::<phone_object::PhoneObject>();

    let selection_model = gtk::NoSelection::new(Some(model_phone_object.clone()));

    let column_view = gtk::ColumnView::new(Some(selection_model));

    column_view.append_column(&column_phone);
    column_view.append_column(&column_time);

    let listbox_log = gtk::ListBox::new();
    let flex_box_log=gtk::Box::default();
    let scrolled_log=gtk::ScrolledWindow::builder()
        .child(&column_view)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    flex_box_log.append(&scrolled_log);

    stack.add_titled(&gtk_box_g, Some("6"),"Сигнал и батарейка");
    stack.add_titled(&flex_box_list,Some("8"),"✆Входящие звонки");
    stack.add_titled(&flex_box_log,Some("7"),"✎Лог");
    let stack_switcher = gtk::StackSwitcher::builder()
        .stack(&stack)
        .build();
    let gtk_box_stack=gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box_stack.append(&stack_switcher);
    gtk_box_stack.append(&stack);
    gtk_box_stack.append(&status);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ydav-gtk")
        .default_height(300)
        .child(&gtk_box_stack)
        .build();
    window.set_widget_name("window");
    load_css();
    // Show the window.
    window.present();
    let mut level: Level = Level(f64::default());
    let mut level_tep: Level = Level(f64::default());
    let (sender, receiver) = async_channel::bounded(1);

    glib::spawn_future_local( clone!(
            #[weak]
            edit_ip_address,
            #[weak]
            button_stop_info,
            async move {
                while let Ok(enable) = receiver.recv().await{
                    edit_ip_address.set_visible(enable);
                    if !enable {
                        button_stop_info.set_label("⏹");
                    }else {
                        button_stop_info.set_label("▶");
                    }
                }
            }
        ));

    let sender_info_phone = sender.clone();
    let address_ip = edit_ip_address.text().to_string().clone();

    let times_phone = times.clone();
    button_list_get.connect_clicked(move |b| {
        b.set_label("Запрос послан");
        let phone = match Phone::connect(address_ip.clone()){
            Ok(phone)=>phone,
            Err(error)=>{
                times_phone.set_markup(format!("{}", error).as_str());
                let sender = sender_info_phone.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };
        times_phone.set_markup(format!("{}", phone.phones.time).as_str());
        listbox.remove_all();
        model_phone_object.remove_all();
        for phone in &phone.phones.phone{
            let d= phone_object::PhoneObject::new();
            d.set_property("time",phone.time.to_value());
            d.set_property("phone",format!("tel: {}", phone.phone));
            model_phone_object.append(&d);

            let s=if phone.status.len()>2{
                format!("{}\t{}\n{}", phone.time, phone.phone, phone.status)
            } else {format!("{}\t{}", phone.time, phone.phone)};

            let label = gtk::Label::new(Some(&s));
            listbox.append(&label);
        }

    });


    let sender_info= sender.clone();
    let address = edit_ip_address.text().to_string().clone();
    let times_log= times.clone();
    let regular_monitoring_info= move ||{
        let log = match Info::connect(address.clone()) {
            Ok(info)=>{
                gtk_box_horizontal.set_visible(true);
                gtk_box_horizontal2.set_visible(true);
                info
            },
            Err(error)=>{
                gtk_box_horizontal.set_visible(false);
                gtk_box_horizontal2.set_visible(false);
                times_log.set_markup(format!("{}", error).as_str());
                let sender = sender_info.clone();
                sender.send_blocking(true).unwrap();
                return ControlFlow::Break
            }


        };

        label_battery_level.set_markup(format!("🔋 {}%", level.get_str(log.info.battery.level)).as_str());
        label_battery_temperature.set_markup(format!("🌡{}°C", level_tep.get_str(log.info.battery.temperature)).as_str());
        label_battery_status.set_markup(format!("{}", log.info.battery.status).as_str());
        label_network_type.set_markup(format!("📶{}",log.info.signal.network_type).as_str());
        label_sim_operator_name.set_markup(format!("{}", log.info.signal.sim_operator_name).as_str());
        label_sim_operator.set_markup(format!("{}",log.info.signal.sim_operator).as_str());
        label_sim_county_iso.set_markup(format!("{}",log.info.signal.sim_county_iso).as_str());
        label_rsrq.set_markup(format!("RSRQ: {} dB", log.info.signal.rsrq).as_str());
        label_rsrp.set_markup(format!("RSRP: {} dBm", log.info.signal.rsrp).as_str());
        label_rssi.set_markup(format!("RSSI: {}", log.info.signal.rssi).as_str());
        times_log.set_markup(format!("🕰{}", log.info.time).as_str());
        //
        // let label_log=gtk::Label::new(Some(format!("Лог:{}", log.json).as_str()));
        //listbox_log.append(&label_log);
        let flag_regular_monitoring_info =edit_ip_address.get_visible();
        if flag_regular_monitoring_info ==false{ return ControlFlow::Continue
        };
        if flag_regular_monitoring_info ==true {
            return ControlFlow::Break
        };
        ControlFlow::Break


    };
    let sender_1 = sender.clone();
    button_stop_info.connect_clicked(move|button_stop_info|{
        let sender = sender_1.clone();
        let regular_monitoring_info_clone = regular_monitoring_info.clone();
        let text=match button_stop_info.label(){
            Some(d)=>d.to_string(),
            None=>"m".to_string()
        };
        if text=="▶" {
            glib::timeout_add_seconds_local(2, regular_monitoring_info_clone);
            button_stop_info.set_label("⏹");
            sender.send_blocking(false).unwrap();
        }else {
            button_stop_info.set_label("▶");
            sender.send_blocking(true).unwrap();
        }
    });
}
fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    provider.load_from_path(Path::new("main.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}
