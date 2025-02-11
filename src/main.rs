extern crate regex;
mod info;
mod phone;
mod phone_object;
mod read_json_android;
mod contact;
mod contact_object;
mod send_command_android;
mod sms_input_object;
mod sms_input;
mod log_object;
mod sms_input_delete;
mod phone_delete;

use crate::contact::Contact;
use crate::info::{Info, Level};
use crate::phone::Phone;
use crate::sms_input::SmsInput;
use gdk4 as gdk;
use gdk4::glib::{clone, ControlFlow, Object};
use gdk4::pango::EllipsizeMode;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{ColumnViewColumn, ListItem};
use gtk4 as gtk;
use gtk4::Orientation::Vertical;
use gtk4::{Justification, Orientation, Widget, WrapMode};
use sqlite::{Connection, State};
use std::io::{BufWriter, Write};
use std::path::Path;
use regex::Regex;

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
    let label_battery_charge = gtk::Label::new(Some("_"));
    label_battery_charge.set_css_classes(&class_info);
    label_battery_charge.set_tooltip_text(Some("Тип зарядки"));
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
    gtk_box_horizontal.append(&label_battery_charge);
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
       .orientation(Vertical)
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
    let label_met_new_phone_input = gtk::Label::new(Some("Новый звонок"));
    label_met_new_phone_input.set_widget_name("new_sms_input");
    label_met_new_phone_input.set_justify(Justification::Center);
    label_met_new_phone_input.set_visible(false);
    gtk_box_g.append(&label_met_new_phone_input);
    let label_met_new_sms_input = gtk::Label::new(Some("📩\nНовое СМС"));
    label_met_new_sms_input.set_widget_name("new_sms_input");
    label_met_new_sms_input.set_justify(Justification::Center);
    label_met_new_sms_input.set_visible(false);
    gtk_box_g.append(&label_met_new_sms_input);

    gtk_box_g.append(&gtk_box_horizontal);
    gtk_box_g.append(&gtk_box_horizontal2);
    gtk_box_g.append(&edit_ip_address);
    gtk_box_g.set_homogeneous(false);
    gtk_box_g.set_widget_name("panel");

    gtk_box_horizontal.set_visible(false);
    gtk_box_horizontal2.set_visible(false);
    //****Контакты********************************************************************************
    let factory_contact_phone = gtk::SignalListItemFactory::new();
    factory_contact_phone.connect_setup( move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_contact_phone.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<contact_object::ContactObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "phone");
    });
    let column_contact_phone =ColumnViewColumn::new(Some("Телефонные номера"), Some(factory_contact_phone));
    let factory_contact_name = gtk::SignalListItemFactory::new();
    factory_contact_name.connect_setup(move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_contact_name.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<contact_object::ContactObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "name");
    });
    let column_contact_name =ColumnViewColumn::new(Some("Имя"), Some(factory_contact_name));
    let model_contact_object: gtk::gio::ListStore = gtk::gio::ListStore::new::<contact_object::ContactObject>();
    let no_selection_contact_model = gtk::NoSelection::new(Some(model_contact_object.clone()));
    let selection_contact_model = gtk::SingleSelection::new(Some(no_selection_contact_model));

    let connection = sqlite::open("data").unwrap();
    let query = "SELECT name, phone FROM contact";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        let contact_object = contact_object::ContactObject::new();
        contact_object.set_property("name", statement.read::<String,_>("name").unwrap().as_str());
        contact_object.set_property("phone", statement.read::<String,_>("phone").unwrap().as_str());
        model_contact_object.append(&contact_object);

    }
    let column_view_contact = gtk::ColumnView::new(Some(selection_contact_model));
    column_view_contact.append_column(&column_contact_name);
    column_view_contact.append_column(&column_contact_phone);

    //*********Входящие вызовы ***********************************************
    let factory_phone = gtk::SignalListItemFactory::new();
    factory_phone.connect_setup( move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_phone.connect_bind(move |_, list_item| {
         list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<phone_object::PhoneObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "phone");
    });
    let column_phone=ColumnViewColumn::new(Some("Телефонный номер"),Some(factory_phone));
    let factory_time = gtk::SignalListItemFactory::new();
    factory_time.connect_setup(move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_time.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<phone_object::PhoneObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item,"time");

    });
    let column_time =ColumnViewColumn::new(Some("Время"), Some(factory_time));
    let model_phone_object: gtk::gio::ListStore = gtk::gio::ListStore::new::<phone_object::PhoneObject>();
    let selection_model = gtk::NoSelection::new(Some(model_phone_object.clone()));
    let column_view_phone = gtk::ColumnView::new(Some(selection_model));
    column_view_phone.append_column(&column_time);
    column_view_phone.append_column(&column_phone);
    //****Входящие СМС********************************************************************************
    let factory_sms_input_phone = gtk::SignalListItemFactory::new();
    factory_sms_input_phone.connect_setup( move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_sms_input_phone.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_input_object::SmsInputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "phone");
    });
    let column_sms_input_phone =ColumnViewColumn::new(Some("От кого"), Some(factory_sms_input_phone));
    let factory_sms_input_time = gtk::SignalListItemFactory::new();
    factory_sms_input_time.connect_setup(move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_sms_input_time.connect_bind(move |_, list_item| {
         list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_input_object::SmsInputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "time");
    });
    let column_sms_input_time =ColumnViewColumn::new(Some("Когда:"), Some(factory_sms_input_time));
    let factory_sms_input_body = gtk::SignalListItemFactory::new();
    factory_sms_input_body.connect_setup(move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_sms_input_body.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_input_object::SmsInputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item,"body");
    });
    let column_sms_input_body =ColumnViewColumn::new(Some("СМС:"), Some(factory_sms_input_body));
    let model_sms_input_object: gtk::gio::ListStore = gtk::gio::ListStore::new::<sms_input_object::SmsInputObject>();
    let no_selection_sms_input_model = gtk::NoSelection::new(Some(model_sms_input_object.clone()));
    let selection_sms_input_model = gtk::SingleSelection::new(Some(no_selection_sms_input_model));


    let text_sms_input_body =gtk::TextBuffer::new(None);
    let textview_sms_input_body =gtk::TextView::with_buffer(&text_sms_input_body);
    textview_sms_input_body.set_widget_name("text_sms_input_body");
    textview_sms_input_body.set_buffer(Some(&text_sms_input_body));
    let dop_panel_for_button_body_input_smst = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let dop_panel_for_button_body_input_sms = dop_panel_for_button_body_input_smst.clone();
    // Буфер обмена ОС
    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();
    //Строка для поиска ссылок и цифр в тексте СМС
    let semver_sms_input = regex::Regex::new(r"(\d+)|([-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))").expect("Ошибка парсера входящих СМС");
    selection_sms_input_model.connect_selection_changed(move|x, _i, _i1| {
        let select_sms_input=x.item(x.selected())
            .and_downcast::<sms_input_object::SmsInputObject>()
            .expect("The item has to be an `SmsinputObject`.");
        let body = select_sms_input.property::<String>("body");
        let mut last = dop_panel_for_button_body_input_sms.last_child();
        while let Some(v)  = last {
            dop_panel_for_button_body_input_sms.remove(&v);
            last = dop_panel_for_button_body_input_sms.last_child();
        }
        text_sms_input_body.set_text(body.as_str());
        let mut button_one = None;
        semver_sms_input.find_iter(body.as_str()).filter(|u| u.len()>2)
        .for_each(|item|{
            let button =gtk::Button::builder().label(format!("📋 {}", item.as_str())).build();
            button.set_tooltip_text(Some("Для копирования нажмите!!"));
            if button_one==None{
                button_one=Some(button.clone());
            };
            button.set_css_classes(&["button_sms_input_body"]);
            let clipboardt=clipboard.clone();
            let panel = button_one.clone();
            button.connect_clicked(move|b|{
                clipboardt.set_text(b.label().unwrap().as_str());
                if let Some(u) = &panel{
                    u.set_widget_name("");
                    u.set_css_classes(&["button_sms_input_body"]);
                    let mut last = u.next_sibling();
                    while let Some(v)  = last {
                        v.set_widget_name("");
                        v.set_css_classes(&["button_sms_input_body"]);
                        last =  v.next_sibling();
                    }
                }
                b.set_widget_name("button_sms_input_body");
            });
            dop_panel_for_button_body_input_sms.append(&button);
        }) ;
    });
    let column_view_sms_input = gtk::ColumnView::new(Some(selection_sms_input_model));
    column_view_sms_input.append_column(&column_sms_input_time);
    column_view_sms_input.append_column(&column_sms_input_phone);
    column_view_sms_input.append_column(&column_sms_input_body);

    let flex_box_list=gtk::Box::builder()
        .orientation(Vertical)
        .build();
    let button_list_get=gtk::Button::builder()
        .label("Запрос звонков")
        .build();
    flex_box_list.append(&button_list_get);
    let scrolled_list=gtk::ScrolledWindow::builder()
        .child(&column_view_phone)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    flex_box_list.append(&scrolled_list);

    let factory_log = gtk::SignalListItemFactory::new();
    factory_log.connect_setup(move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_log.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<log_object::LogObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item,"log");
    });
    let column_log =ColumnViewColumn::new(Some("Лог:"), Some(factory_log));
    let model_log: gtk::gio::ListStore = gtk::gio::ListStore::new::<log_object::LogObject>();
    let no_selection_log = gtk::NoSelection::new(Some(model_log.clone()));
    let selection_log = gtk::SingleSelection::new(Some(no_selection_log));

    let text_log =gtk::TextBuffer::new(None);
    let textview_log =gtk::TextView::with_buffer(&text_log);
    textview_log.set_widget_name("text_log");
    textview_log.set_buffer(Some(&text_log));

    selection_log.connect_selection_changed(move|x, _i, _i1| {
        let select_sms_input=x.item(x.selected())
            .and_downcast::<log_object::LogObject>()
            .expect("The item has to be an `LogObject`.");
        text_log.set_text(select_sms_input.property::<String>("log").as_str());
    });
    let column_view_log = gtk::ColumnView::new(Some(selection_log));
    column_view_log.append_column(&column_log);


    let flex_box_log=gtk::Box::builder()
        .orientation(Vertical).build();
    flex_box_log.set_widget_name("log");
    let scrolled_log=gtk::ScrolledWindow::builder()
        .child(&column_view_log)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    let check_log = gtk::CheckButton::with_label("Вести запись в лог");
    flex_box_log.append(&check_log);
    flex_box_log.append(&scrolled_log);

    let button_contact_get=gtk::Button::builder()
        .label("Запрос контактов")
        .build();
    let button_contact_csv=gtk::Button::builder()
        .label("Сохранить в формате CSV")
        .build();
    let csv_model_contact = model_contact_object.clone();
    let flex_box_contact=gtk::Box::builder()
        .orientation(Vertical).build();
    flex_box_contact.append(&button_contact_get);
    flex_box_contact.append(&button_contact_csv);

    let label_count_contact = gtk::Label::new(None);
    label_count_contact.set_widget_name("label_count_contact");
    flex_box_contact.append(&label_count_contact);
    let scrolled_contact=gtk::ScrolledWindow::builder()
        .child(&column_view_contact)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    flex_box_contact.append(&scrolled_contact);


    let button_sms_input_get=gtk::Button::builder()
        .label("Входящих СМС (загрузка с телефона)")
        .sensitive(false)
        .build();

    let flex_box_sms_input =gtk::Box::builder()
        .orientation(Vertical).build();
    flex_box_sms_input.append(&button_sms_input_get);
    let scrolled_sms_input =gtk::ScrolledWindow::builder()
        .child(&column_view_sms_input)
        .height_request(250)
        .propagate_natural_width(true)
        .build();
    flex_box_sms_input.append(&scrolled_sms_input);
    textview_sms_input_body.set_wrap_mode(WrapMode::Word);
    textview_sms_input_body.set_editable(false);
    let scrolled_sms_input_text_body =gtk::ScrolledWindow::builder()
        .child(&textview_sms_input_body)
        .height_request(75)
        .propagate_natural_width(true)
        .build();

    flex_box_sms_input.append(&scrolled_sms_input_text_body);
    flex_box_sms_input.append(&dop_panel_for_button_body_input_smst);


    stack.add_titled(&gtk_box_g, Some("Signal"),"Сигнал и батарейка");
    stack.add_titled(&flex_box_list,Some("Phone"),"✆Входящие звонки");
    stack.add_titled(&flex_box_contact,Some("Contact"),"Контакты");
    stack.add_titled(&flex_box_sms_input,Some("InputSMS"),"Входящие СМС");
    stack.add_titled(&flex_box_log,Some("Logs"),"✎Лог");
    let stack_switcher = gtk::StackSwitcher::builder()
        .stack(&stack)
        .build();
    let gtk_box_stack=gtk::Box::builder()
        .orientation(Vertical)
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

    let connection = sqlite::open("data").unwrap();
    let query = "SELECT id_na_android, phone, time, body FROM sms_input ORDER BY _id DESC ";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        let sms_input_object = sms_input_object::SmsInputObject::new();
        sms_input_object.set_property("id", statement.read::<String,_>("id_na_android").unwrap().as_str());
        sms_input_object.set_property("phone", statement.read::<String,_>("phone").unwrap().as_str());
        sms_input_object.set_property("time", statement.read::<String,_>("time").unwrap().as_str());
        sms_input_object.set_property("body", statement.read::<String,_>("body").unwrap().as_str());
        model_sms_input_object.append(&sms_input_object);
    }
    let query = "SELECT id_na_android, phone, time FROM phone_input ORDER BY _id DESC ";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        let phone_input_object = phone_object::PhoneObject::new();
        phone_input_object.set_property("id", statement.read::<String,_>("id_na_android").unwrap().as_str());
        phone_input_object.set_property("phone", statement.read::<String,_>("phone").unwrap().as_str());
        phone_input_object.set_property("time", statement.read::<String,_>("time").unwrap().as_str());
        model_phone_object.append(&phone_input_object);
    }
    button_contact_csv.connect_clicked(move |_x1| {
        let file_filter = gtk::FileFilter::new();
        file_filter.add_pattern("*.csv");
        file_filter.add_suffix("csv");
        let save_dialog = gtk::FileDialog::builder()
            .title("Файл для контактов")
            .default_filter(&file_filter)
            .build();
        let cancellable =gtk::gio::Cancellable::new();
        let value_csv_model_contact = csv_model_contact.clone();
        save_dialog.save(Some(&window), Some(&cancellable), move |x2| {
            let path_file = match x2 {
                Ok(f)=>f.path(),
                Err(e)=> panic!("{}",e)
            };
            let path = path_file.unwrap();
            let file_contact_csv = std::fs::File::create(path).expect("Ошибка создания файла!");
            let mut writer = BufWriter::new(file_contact_csv);
            for object in value_csv_model_contact.into_iter(){
                if let Ok(item) = object {
                    let data = item.downcast_ref::<contact_object::ContactObject>()
                        .expect("Item not ContactObject");
                    let phohe =data.property::<String>("phone");
                    let name =data.property::<String>("name");
                    if let Err(e)= writer.write_all(format!("{}, {}", name, phohe).as_bytes()){
                        println!("{}",e);
                    }
                }
            }
        });

    });
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
    let label_met = label_met_new_phone_input.clone();
    button_list_get.connect_clicked(move |_b| {
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
        let connection = sqlite::open("data").unwrap();
        let mut sql ="BEGIN TRANSACTION;".to_string();
        for phone in &phone.phones.phone{
            let phone_object = phone_object::PhoneObject::new();
            phone_object.set_property("id", phone.id.to_value());
            phone_object.set_property("time", phone.time.to_value());
            let str_status = match phone.status.as_str() {
                "IDLE"=>"📱",
                "RINGING"=>"📲",
                _=>""

            };
            if !phone.phone.is_empty() {
                let name = find_phone(phone.phone.clone(), &connection);
                let text = if !name.is_empty(){
                    format!("{}({}) {}", name, phone.phone, str_status)
                }else{
                    format!("{} {}", phone.phone, str_status)
                };
                phone_object.set_property("phone", text);
                sql=sql+format!("INSERT INTO phone_input (id_na_android, phone, time, status) VALUES (\"{}\", \"{}\", \"{}\", \"_\");", phone.id, phone.phone, phone.time).as_str();
            }
            else {
                phone_object.set_property("phone", format!("{}", str_status));
                sql=sql+format!("INSERT INTO phone_input (id_na_android, phone, time, status) VALUES (\"{}\", \"-\", \"{}\", \"{}\");", phone.id, phone.time, phone.status).as_str();
            }
            model_phone_object.append(&phone_object);
        }
        sql=sql+"COMMIT;";
        sql_execute(sql);
        let mut param_where_sql=String::new();
        let mut where_sql=String::new();
        let query = "SELECT _id, id_na_android FROM phone_input WHERE id_na_android>0";
        let connection = sqlite::open("data").unwrap();
        let mut statement = connection.prepare(query).unwrap();
        if statement.iter().count()>0 {
            while let Ok(State::Row) = statement.next() {
                if !param_where_sql.is_empty(){
                    param_where_sql.push_str(" OR ");
                    where_sql.push_str(" OR ");
                }
                let sms_input_id=statement.read::<String,_>("id_na_android").unwrap();
                param_where_sql.push_str(format!("_id={}", sms_input_id).as_str());
                let id=statement.read::<String,_>("_id").unwrap();
                where_sql.push_str(format!("_id={}", id).as_str());
            }

            let phone_input_delete = phone_delete::PhoneDelete::connect(address_ip.clone(), &param_where_sql);
            if let Ok(_sid)= phone_input_delete {
                let sql = format!("UPDATE phone_input SET id_na_android=0 WHERE {}", where_sql);
                sql_execute(sql);
                label_met.set_visible(false);
            }
        }
    });

    let sender_info_contact = sender.clone();
    let times_contact = times.clone();
    let address_ip = edit_ip_address.text().to_string().clone();
    let label_met = label_met_new_sms_input.clone();
    button_contact_get.connect_clicked(move |_b|{
        let contact = match Contact::connect(address_ip.clone()){
            Ok(contact)=> contact,
            Err(error)=>{
                times_contact.set_markup(format!("{}", error).as_str());
                let sender = sender_info_contact.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };
        times_contact.set_markup(format!("{}", contact.contacts.time).as_str());
        let mut sql =" DELETE FROM contact;
        UPDATE sqlite_sequence SET seq=0 WHERE name='contact'; BEGIN TRANSACTION;".to_string();
        for contact in &contact.contacts.contact{
            let r =contact.phone.iter().fold(String::new(), |mut s, w|{s.push_str(
                format!("{} ",w.as_str()).as_str()
            );s});
            let contact_object = contact_object::ContactObject::new();
            contact_object.set_property("name", contact.name.to_value());
            contact_object.set_property("phone", r.to_value());
            model_contact_object.append(&contact_object);
            sql=sql+format!("INSERT INTO contact (name, phone) VALUES (\"{}\", \"{}\");", contact.name, r).as_str();
        }
        sql=sql+"COMMIT;";
        label_count_contact.set_markup(format!(" У Вас контактов <b>{}</b>.", &contact.contacts.contact.len()).as_str());
        sql_execute(sql);
    });

    let sender_info_sms_input = sender.clone();
    let times_sms_input = times.clone();
    let address_ip_sms_input = edit_ip_address.text().to_string().clone();
    button_sms_input_get.connect_clicked(move |b|{
        b.set_sensitive(false);
        let sms_input = match SmsInput::connect(address_ip_sms_input.clone()){
            Ok(sms_input)=> sms_input,
            Err(error)=>{
                times_sms_input.set_markup(format!("{}", error).as_str());
                let sender = sender_info_sms_input.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };

        times_sms_input.set_markup(format!("{}", sms_input.sms_input.time).as_str());
        let mut sql ="BEGIN TRANSACTION;".to_string();
        for sms in &sms_input.sms_input.sms{
            let sms_input_object = sms_input_object::SmsInputObject::new();
            sms_input_object.set_property("id", sms.id.to_value());
            sms_input_object.set_property("phone", sms.phone.to_value());
            sms_input_object.set_property("time", sms.time.to_value());
            sms_input_object.set_property("body", sms.body.to_value());
            model_sms_input_object.append(&sms_input_object);
            sql=sql+format!("INSERT INTO sms_input (id_na_android, phone, time, body) VALUES (\"{}\", \"{}\", \"{}\", \"{}\");", sms.id, sms.phone, sms.time, sms.body).as_str();
        }
        sql=sql+"COMMIT;";
        sql_execute(sql);
        let mut param_where_sql=String::new();
        let mut where_sql=String::new();
        let query = "SELECT _id, id_na_android FROM sms_input WHERE id_na_android>0";
        let connection = sqlite::open("data").unwrap();
        let mut statement = connection.prepare(query).unwrap();
        if statement.iter().count()>0 {
            while let Ok(State::Row) = statement.next() {
                if !param_where_sql.is_empty(){
                    param_where_sql.push_str(" OR ");
                    where_sql.push_str(" OR ");
                }
                let sms_input_id=statement.read::<String,_>("id_na_android").unwrap();
                param_where_sql.push_str(format!("_id={}", sms_input_id).as_str());
                let id=statement.read::<String,_>("_id").unwrap();
                where_sql.push_str(format!("_id={}", id).as_str());
            }

            let sms_input_delete = sms_input_delete::SmsInputDelete::connect(address_ip_sms_input.clone(),&param_where_sql);
            if let Ok(_sid)=sms_input_delete{
                let sql = format!("UPDATE sms_input SET id_na_android=0 WHERE {}", where_sql);
                sql_execute(sql);
                label_met.set_visible(false);
            }
        }

    });

    let check_log_monotoring = check_log.clone();
    let sender_info= sender.clone();
    let address = edit_ip_address.text().to_string().clone();
    let times_log= times.clone();
    let label_met = label_met_new_sms_input.clone();
    let button_input_sms = button_sms_input_get.clone();
    let label_mata_phone = label_met_new_phone_input.clone();
    let regular_monitoring_info= move ||{
        let log= match Info::connect(address.clone()) {
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
        label_battery_charge.set_markup(format!("{}", log.info.battery.charge).as_str());
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
        times_log.set_markup(format!("🕰{} СМС:{}", log.info.time, log.info.sms).as_str());
        if check_log_monotoring.is_active() {
            let log_object = log_object::LogObject::new();
            log_object.set_property("log", format!("Лог:{}", log.json).as_str());
            model_log.append(&log_object)
        }
        // TODO Есть новые СМС! Реакция?
        if log.info.sms>0{
            label_met.set_visible(true);
            button_input_sms.set_sensitive(true);
            times_log.set_markup(format!("🕰{} СМС:{}", log.info.time, log.info.sms).as_str());
        }

        if log.info.phone>0{
            label_mata_phone.set_visible(true);
        }

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
            glib::timeout_add_seconds_local(1, regular_monitoring_info_clone);
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

fn add_label_is_item(list_item: &Object){
    let label = gtk::Label::builder().build();
    label.set_justify(Justification::Left);
    label.set_ellipsize(EllipsizeMode::End);
    list_item
        .downcast_ref::<ListItem>()
        .expect("error")
        .set_child(Some(&label));
}

fn sql_execute(sql: String){
    let connection = sqlite::open("data").unwrap();
    if let Err(e)=connection.execute(&sql){
        println!("Ошибка {} тут {}", e, sql);
    }
}

fn find_phone(phone: String, connection: &Connection)->String {
    let mut statement = connection.prepare(format!("SELECT name FROM contact WHERE phone LIKE '%{}%' ", phone.clone()).as_str()).unwrap();
    if statement.iter().count() > 0 {
        while let Ok(State::Row) = statement.next() {
            return statement.read::<String, _>("name").unwrap()
        }
    }
    let mut statement = connection.prepare(format!("SELECT name FROM contact WHERE phone LIKE '%{}%' ", phone[1..].to_string()).as_str()).unwrap();
    if statement.iter().count() > 0 {
        while let Ok(State::Row) = statement.next() {
            return statement.read::<String, _>("name").unwrap()
        }
    }
    "".to_string()
}