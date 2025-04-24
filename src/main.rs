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
mod sms_output_object;
mod sms_output;
mod config;
mod ussd;

use crate::ussd::Ussd;
use crate::config::Config;
use crate::contact::Contact;
use crate::info::{Info, Level};
use crate::phone::Phone;
use crate::sms_input::SmsInput;
use crate::sms_output::SmsOutputParam;
use chrono::Local;
use gdk4 as gdk;
use gdk4::glib::{clone, home_dir, ControlFlow, Object};
use gdk4::pango::EllipsizeMode;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{ColumnViewColumn, ListItem};
use gtk4 as gtk;
use gtk4::glib::Propagation;
use gtk4::Orientation::{Horizontal, Vertical};
use gtk4::{gio, Align, Justification, WrapMode};
use sqlite::{Connection, State, Statement};
use std::io::{BufWriter, Write};

fn main() {
    gio::resources_register_include!("compiled.gresource").unwrap();
    let app = Application::builder()
        .application_id("ru.Dimon.Ydav-gtk")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let mut config = Config::new();
    if let Err(_e)=config.load(){
        let sql = include_str!("sql.in").to_string();
        Config::sql_execute(sql);
    };
    let icon_theme = gtk::IconTheme::new();
    icon_theme.add_resource_path("ru/Dimon/Ydav-gtk");
    icon_theme.add_resource_path("ru/Dimon/Ydav-gtk/icons");
    let (sender, receiver) = async_channel::bounded(1);
    let class_info=["info"];
    // Буфер обмена ОС
    let display = gdk::Display::default().unwrap();
    let clipboard = display.clipboard();
    let gtk_box_horizontal =gtk::Box::builder()
            .orientation(Horizontal)
            .halign(Align::Fill)
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
        .orientation(Horizontal)
        .halign(Align::Fill)
        .build();
    gtk_box_horizontal.set_css_classes(&["panel"]);
    gtk_box_horizontal2.set_css_classes(&["panel"]);
    gtk_box_horizontal2.append(&label_network_type);
    gtk_box_horizontal2.append(&label_sim_operator_name);
    gtk_box_horizontal2.append(&label_sim_operator);
    gtk_box_horizontal2.append(&label_sim_county_iso);

    let text_signal_param =gtk::TextBuffer::new(None);
    let textview =gtk::TextView::with_buffer(&text_signal_param);
    textview.set_widget_name("text_signal_param");
    textview.set_wrap_mode(WrapMode::Word);
    textview.set_buffer(Some(&text_signal_param));
    let scrolled_signal_param =gtk::ScrolledWindow::builder()
        .child(&textview)
        .propagate_natural_width(true)
        .build();
    scrolled_signal_param.set_vexpand(true);
    scrolled_signal_param.set_vexpand_set(true);

    let edit_ip_address = gtk::Entry::new();
    let ip = config.param.entry("ip".to_string()).or_insert_with(||{"192.168.1.91:38300".to_string()});
    edit_ip_address.set_text(ip);
    edit_ip_address.set_widget_name("edit_ip");
    
    let gtk_box_g=gtk::Box::builder()
       .orientation(Vertical)
       .build();
    let stack = gtk::Stack::new();
    let times = gtk::Label::new(Some(""));
    let button_stop_info = gtk::Button::new();
    button_stop_info.set_css_classes(&["button"]);
    button_stop_info.set_label("▶");
    let status = gtk::Box::builder()
        .orientation(Horizontal)
        .build();
    status.set_widget_name("statusbar");
    status.set_valign(Align::Start);
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

    gtk_box_g.append(&edit_ip_address);
    let label_politic = gtk::Label::new(Some("Продолжая использовать программу Вы соглашаетесь с "));
    let button_politic = gtk::Button::with_label("Политикой конфиденциальности");
    let politic = config.param.entry("politic".to_string()).or_insert_with(||{"".to_string()});
    if !politic.is_empty(){
        label_politic.set_visible(false);
        button_politic.set_visible(false);
    }
    button_politic.set_css_classes(&["button_politic"]);

    button_politic.connect_clicked(move |_x1| {
        let text_politic =gtk::TextBuffer::new(None);
        let textview =gtk::TextView::with_buffer(&text_politic);
        textview.set_widget_name("text_sms_input_body");
        textview.set_wrap_mode(WrapMode::Word);
        textview.set_buffer(Some(&text_politic));
        let scrolled_politic =gtk::ScrolledWindow::builder()
            .child(&textview)
            .propagate_natural_width(true)
            .build();
        scrolled_politic.set_vexpand(true);
        scrolled_politic.set_vexpand_set(true);
        let sql = include_str!("politic.in").to_string();
        text_politic.set_text(sql.as_str());
        let button_close = gtk::Button::with_label("Ознакомлен");
        let gtk_box_politic = gtk::Box::builder()
            .orientation(Vertical)
            .build();
        gtk_box_politic.append(&scrolled_politic);
        gtk_box_politic.append(&button_close);

        let window = gtk::Window::builder()
            .title("Ydav-gtk v1.2.0")
            .height_request(320)
            .width_request(360)
            .child(&gtk_box_politic)
            .icon_name("ru_dimon_ydav_2024")
            .build();
        button_close.connect_clicked(clone!(
            #[weak]
            window,
            move |_x1| {
            let mut config = Config::new();
            config.param.insert("politic".to_string(), "1".to_string());
            config.save();
            window.close();
        }));
        window.present();

    });
    let panel_box_politic = gtk::Box::builder()
        .orientation(Horizontal)
        .build();
    panel_box_politic.append(&label_politic);
    panel_box_politic.append(&button_politic);
    gtk_box_g.append(&panel_box_politic);
    let button_about = gtk::Button::with_label("О программе");
    button_about.set_css_classes(&["button_politic"]);
    button_about.connect_clicked(clone!(
            #[weak]
            clipboard,
            move |_x1| {
        let text_politic =gtk::TextBuffer::new(None);
        let textview =gtk::TextView::with_buffer(&text_politic);
        textview.set_wrap_mode(WrapMode::Word);
        textview.set_buffer(Some(&text_politic));
        let scrolled_politic =gtk::ScrolledWindow::builder()
            .child(&textview)
            .propagate_natural_width(true)
            .build();
        scrolled_politic.set_vexpand(true);
        scrolled_politic.set_vexpand_set(true);
        text_politic.set_text("Программа клиент Ydav-gtk4 для сервера Ydav2024 for Android версия: 1.3.0");
        let button_git = gtk::Button::with_label("https://ydav-android.p-k-53.ru/");
        button_git.set_tooltip_text(Some("Нажмите для копирования адреса в буфер обмена"));
        button_git.connect_clicked(clone!(
            #[weak]
            clipboard,
            move |b|{
                clipboard.set_text(b.label().unwrap().as_str());
            }
        ));
        let button_www = gtk::Button::with_label("https://github.com/almaz-vil/ydav-gtk4.git");
        button_www.set_tooltip_text(Some("Нажмите для копирования адреса в буфер обмена"));
        button_www.connect_clicked(clone!(
            #[weak]
            clipboard,
            move |b|{
                clipboard.set_text(b.label().unwrap().as_str());
            }
        ));
        let button_close = gtk::Button::with_label("Хорошо");

        let gtk_box_politic = gtk::Box::builder()
            .orientation(Vertical)
            .build();
        gtk_box_politic.append(&button_git);
        gtk_box_politic.append(&button_www);
        gtk_box_politic.append(&scrolled_politic);

        gtk_box_politic.append(&button_close);

        let window = gtk::Window::builder()
            .title("Ydav-gtk")
            .height_request(320)
            .width_request(360)
            .child(&gtk_box_politic)
            .icon_name("ru_dimon_ydav_2024")
            .build();
        let win = window.clone();
        button_close.connect_clicked(move |_x1| {
            win.close();
        });
        window.present();

    }));
    gtk_box_g.append(&button_about);
    gtk_box_g.set_homogeneous(false);
    gtk_box_g.set_css_classes(&["panel_win"]);
    gtk_box_g.append(&gtk_box_horizontal);
    gtk_box_g.append(&gtk_box_horizontal2);
    gtk_box_g.append(&scrolled_signal_param);

    gtk_box_horizontal.set_visible(false);
    gtk_box_horizontal2.set_visible(false);
    let edit_sms_output_phone = gtk::Entry::new();
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
   column_contact_name.set_expand(true);
    let model_contact_object: gio::ListStore = gio::ListStore::new::<contact_object::ContactObject>();
    let no_selection_contact_model = gtk::NoSelection::new(Some(model_contact_object.clone()));
    let selection_contact_model = gtk::SingleSelection::new(Some(no_selection_contact_model));
    let connection = sqlite::open(home_dir().join("ydav2024-data")).expect("Ошибка базы данных");
    let query = "SELECT name, phone FROM contact ORDER BY name ASC";
    let mut statement = match connection.prepare(query) {
        Ok(st)=> st,
        Err(e) => {
            let box_error = gtk::Box::builder().orientation(Horizontal).build();
            let label = gtk::Label::new(Some(format!("Ошибка \"{}\" базы данных. \n Будет создана новая база. Подождите ... ", e).as_str()));
            box_error.append(&label);
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Ydav-gtk")
                .default_height(300)
                .child(&box_error)
                .icon_name("ru_dimon_ydav_2024")
                .build();
            window.set_widget_name("window");
            load_css();
            window.present();
            let sql = include_str!("sql.in").to_string();
            Config::sql_execute(sql);
            label.set_label("Создана новая база данный. Перезапустите программу.");
            return;
        }
    };
    while let Ok(State::Row) = statement.next() {
        let name = statement.read::<String,_>("name").unwrap();
        let phone = statement.read::<String,_>("phone").unwrap();
        let contact_object = contact_object::ContactObject::new();
        contact_object.set_property("name", name.as_str());
        contact_object.set_property("phone", phone.as_str());
        model_contact_object.append(&contact_object);
    }
    let column_view_contact = gtk::ColumnView::new(Some(selection_contact_model.clone()));
    column_view_contact.append_column(&column_contact_name);
    column_view_contact.append_column(&column_contact_phone);
    let edit_sms_output_phone_contact = edit_sms_output_phone.clone();
    selection_contact_model.connect_selection_changed(move |x, _i, _i1| {
        let select_contact = x.item(x.selected())
            .and_downcast::<contact_object::ContactObject>()
            .expect("The item has to be an `SmsOutputObject`.");
        let phone = select_contact.property::<String>("phone");
        edit_sms_output_phone_contact.set_text(phone.as_str());
    });

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
    let model_phone_object: gio::ListStore = gio::ListStore::new::<phone_object::PhoneObject>();
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
    let model_sms_input_object: gio::ListStore = gio::ListStore::new::<sms_input_object::SmsInputObject>();
    let no_selection_sms_input_model = gtk::NoSelection::new(Some(model_sms_input_object.clone()));
    let selection_sms_input_model = gtk::SingleSelection::new(Some(no_selection_sms_input_model));
    //**************Исходящие СМС***********************************************************
    let factory_sms_output_phone = gtk::SignalListItemFactory::new();
    factory_sms_output_phone.connect_setup( move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_sms_output_phone.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_output_object::SmsOutputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "phone");
    });
    let column_sms_output_phone =ColumnViewColumn::new(Some("Номер"), Some(factory_sms_output_phone));
    let factory_sms_output_text = gtk::SignalListItemFactory::new();
    factory_sms_output_text.connect_setup( move |_, list_item| {
        add_label_is_item(list_item);
    });
    factory_sms_output_text.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_output_object::SmsOutputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion(list_item, "text");
    });
    let column_sms_output_text =ColumnViewColumn::new(Some("Текст СМС"), Some(factory_sms_output_text));
    let factory_sms_output_sent = gtk::SignalListItemFactory::new();
    factory_sms_output_sent.connect_setup( move |_, list_item| {
        add_panel_is_item(list_item);
    });
    factory_sms_output_sent.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_output_object::SmsOutputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion_dy_panel(list_item, "sent", "senttime");

    });
    let column_sms_output_sent =ColumnViewColumn::new(Some("Статус отправки"), Some(factory_sms_output_sent));
    let factory_sms_output_delivery = gtk::SignalListItemFactory::new();
    factory_sms_output_delivery.connect_setup( move |_, list_item| {
        add_panel_is_item(list_item);
    });
    factory_sms_output_delivery.connect_bind(move |_, list_item| {
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<sms_output_object::SmsOutputObject>()
            .expect("The item has to be an `IntegerObject`.")
            .factorion_dy_panel(list_item, "delivery", "deliverytime");

    });
    let column_sms_output_delivery =ColumnViewColumn::new(Some("Статус отправки"), Some(factory_sms_output_delivery));
    let model_sms_output_object: gio::ListStore = gio::ListStore::new::<sms_output_object::SmsOutputObject>();
    let no_selection_sms_output_model = gtk::NoSelection::new(Some(model_sms_output_object.clone()));
    let selection_sms_output_model = gtk::SingleSelection::new(Some(no_selection_sms_output_model));
    let sender_sms_output = sender.clone();
    let sel_sms_output_model = selection_sms_output_model.clone();
    selection_sms_output_model.connect_selection_changed(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        times,
       move |_x, _i, _i1| {
        let select_sms_output =
            sel_sms_output_model.selected_item()
            .and_downcast::<sms_output_object::SmsOutputObject>()
            .expect("The item has to be an `SmsOutputObject`.");
        let id = select_sms_output.property::<String>("id");
        let address = edit_ip_address.text().to_string();
        let sms_output = match  sms_output::SmsOutput::status(address, id.as_str()){
            Ok(sms_output)=> sms_output,
            Err(error)=>{
                times.set_markup(format!("{}", error).as_str());
                let sender = sender_sms_output.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };
        let sent = sms_output.status.status.sent.result;
        let sent_time = sms_output.status.status.sent.time;
        let delivery = sms_output.status.status.delivery.result;
        let delivery_time = sms_output.status.status.delivery.time;

        select_sms_output.set_property("sent", sent.clone());
        select_sms_output.set_property("senttime", sent_time.clone());
        select_sms_output.set_property("delivery", delivery.clone());
        select_sms_output.set_property("deliverytime", delivery_time.clone());

        let sql = format!("UPDATE sms_output SET sent='{}', sent_time='{}', delivery='{}', delivery_time='{}' WHERE _id={}", sent, sent_time, delivery, delivery_time, id);
        Config::sql_execute(sql);


    }));
    let column_view_sms_output = gtk::ColumnView::new(Some(selection_sms_output_model));
    column_view_sms_output.append_column(&column_sms_output_phone);
    column_view_sms_output.append_column(&column_sms_output_text);
    column_view_sms_output.append_column(&column_sms_output_sent);
    column_view_sms_output.append_column(&column_sms_output_delivery);

    let text_sms_input_body =gtk::TextBuffer::new(None);
    let textview_sms_input_body =gtk::TextView::with_buffer(&text_sms_input_body);
    textview_sms_input_body.set_widget_name("text_sms_input_body");
    textview_sms_input_body.set_buffer(Some(&text_sms_input_body));
    let dop_panel_for_button_body_input_smst = gtk::Box::builder()
        .orientation(Horizontal)
        .build();
    let dop_panel_for_button_body_input_sms = dop_panel_for_button_body_input_smst.clone();
       //Строка для поиска ссылок и цифр в тексте СМС
    let semver_sms_input = regex::Regex::new(r"(\d+)|([-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*))").expect("Ошибка парсера входящих СМС");
    selection_sms_input_model.connect_selection_changed(clone!(
            #[weak]
            clipboard,
            move|x, _i, _i1| {
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
            button.set_css_classes(&["button"]);
            let panel = button_one.clone();
            button.connect_clicked(clone!(
            #[weak]
            clipboard,
            move|b|{
                clipboard.set_text(b.label().unwrap().as_str());
                if let Some(u) = &panel{
                    u.set_widget_name("");
                    u.set_css_classes(&["button"]);
                    let mut last = u.next_sibling();
                    while let Some(v)  = last {
                        v.set_widget_name("");
                        v.set_css_classes(&["button"]);
                        last =  v.next_sibling();
                    }
                }
                b.set_widget_name("button");
            }));
            dop_panel_for_button_body_input_sms.append(&button);
        }) ;
    }));
    let column_view_sms_input = gtk::ColumnView::new(Some(selection_sms_input_model));
    column_view_sms_input.append_column(&column_sms_input_time);
    column_view_sms_input.append_column(&column_sms_input_phone);
    column_view_sms_input.append_column(&column_sms_input_body);

    let flex_box_list_phone =gtk::Box::builder()
        .orientation(Vertical)
        .build();
    let button_phone_get =gtk::Button::builder()
        .label("Запрос звонков")
        .build();
    flex_box_list_phone.append(&button_phone_get);
    let scrolled_list_phone =gtk::ScrolledWindow::builder()
        .child(&column_view_phone)
        .propagate_natural_width(true)
        .build();
    scrolled_list_phone.set_vexpand(true);
    scrolled_list_phone.set_vexpand_set(true);
    flex_box_list_phone.append(&scrolled_list_phone);

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
    let model_log: gio::ListStore = gio::ListStore::new::<log_object::LogObject>();
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
    column_view_sms_output.set_vexpand(true);
    column_view_sms_output.set_vexpand_set(true);
    column_view_log.set_vexpand(true);
    column_view_log.set_vexpand_set(true);
    let scrolled_log=gtk::ScrolledWindow::builder()
        .child(&column_view_log)
        .propagate_natural_width(true)
        .build();
    scrolled_log.set_vexpand(true);
    scrolled_log.set_vexpand_set(true);
    let check_log = gtk::CheckButton::with_label("Вести запись в лог");
    flex_box_log.append(&check_log);
    flex_box_log.append(&scrolled_log);

    let button_contact_get=gtk::Button::builder()
        .label("Запрос контактов")
        .build();
    button_contact_get.set_css_classes(&["button"]);
    let button_contact_csv=gtk::Button::builder()
        .label("Сохранить в формате CSV")
        .build();
    button_contact_csv.set_css_classes(&["button"]);
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
        .propagate_natural_width(true)
        .build();
    scrolled_contact.set_vexpand(true);
    scrolled_contact.set_vexpand_set(true);
    flex_box_contact.append(&scrolled_contact);


    let button_sms_input_get=gtk::Button::builder()
        .label("Входящих СМС (загрузка с телефона)")
        .sensitive(false)
        .build();
    button_sms_input_get.set_css_classes(&["button"]);

    let flex_box_sms_input =gtk::Box::builder()
        .orientation(Vertical).build();
    flex_box_sms_input.append(&button_sms_input_get);
    let scrolled_sms_input =gtk::ScrolledWindow::builder()
        .child(&column_view_sms_input)
        .propagate_natural_width(true)
        .build();
    scrolled_sms_input.set_vexpand(true);
    scrolled_sms_input.set_vexpand_set(true);
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

    let flex_box_sms_output = gtk::Box::builder()
        .orientation(Vertical).build();
    let flex_box_sms_output_box_horizontal =gtk::Box::builder()
        .orientation(Horizontal).build();
    let label = gtk::Label::new(Some("Выбор номера телефона"));
    let exp = gtk::PropertyExpression::new(
        contact_object::ContactObject::static_type(),
        None::<gtk::Expression>,
        "name",
    );
    let combo_box_phone = gtk::DropDown::new(Some(model_contact_object.clone()), Some(exp));
    combo_box_phone.set_css_classes(&["button"]);
    let edit_phone = edit_sms_output_phone.clone();
    combo_box_phone.connect_selected_notify(move |dd|{
       let binding = dd.selected_item().unwrap();
       let sel = binding.downcast_ref::<contact_object::ContactObject>().unwrap();
       edit_phone.set_text(sel.property::<String>("phone").as_str());
   });
    flex_box_sms_output_box_horizontal.append(&label);
    flex_box_sms_output_box_horizontal.append(&edit_sms_output_phone);
    flex_box_sms_output_box_horizontal.append(&combo_box_phone);
    flex_box_sms_output.append(&flex_box_sms_output_box_horizontal);
    let label = gtk::Label::new(Some("СМС (текст)"));
    let edit_sms_output_text = gtk::TextView::builder()
        .height_request(100).build();
    let buffer_sms_output_text = gtk::TextBuffer::new(None);
    edit_sms_output_text.set_buffer(Some(&buffer_sms_output_text));
    let scrolled_sms_output_text =gtk::ScrolledWindow::builder()
        .child(&edit_sms_output_text)
        .height_request(100)
        .propagate_natural_width(true)
        .build();
    let button_sms_output_send = gtk::Button::builder()
        .label("Отправить").build();
    flex_box_sms_output.append(&label);
    flex_box_sms_output.append(&scrolled_sms_output_text);
    flex_box_sms_output.append(&button_sms_output_send);
    let scrolled_sms_output=gtk::ScrolledWindow::builder()
        .child(&column_view_sms_output)
        .height_request(100)
        .propagate_natural_width(true)
        .build();
    scrolled_sms_output.set_vexpand(true);
    flex_box_sms_output.append(&scrolled_sms_output);
    button_phone_get.set_css_classes(&["button"]);
    let edit_ussd_command = gtk::Entry::new();
    edit_ussd_command.set_text("*100#");
    let button_send_ussd_command = gtk::Button::with_label("Отправить USSD команду");
    let box_send_ussd = gtk::Box::builder()
        .orientation(Horizontal)
        .build();
    box_send_ussd.append(&edit_ussd_command);
    box_send_ussd.append(&button_send_ussd_command);
    let button_result_ussd_command = gtk::Button::with_label("Чтения результата USSD команды");
    let text_result_ussd =gtk::TextBuffer::new(None);
    let textview =gtk::TextView::with_buffer(&text_result_ussd);
    textview.set_widget_name("text_signal_param");
    textview.set_wrap_mode(WrapMode::Word);
    textview.set_buffer(Some(&text_result_ussd));
    let scrolled_result_ussd=gtk::ScrolledWindow::builder()
        .child(&textview)
        .height_request(100)
        .propagate_natural_width(true)
        .build();
    scrolled_result_ussd.set_vexpand(true);
    scrolled_result_ussd.set_vexpand_set(true);
    let flex_box_ussd_command =gtk::Box::builder()
        .orientation(Vertical).build();
    flex_box_ussd_command.append(&box_send_ussd);
    flex_box_ussd_command.append(&button_result_ussd_command);
    flex_box_ussd_command.append(&scrolled_result_ussd);
    stack.set_css_classes(&["panel_win"]);
    stack.add_titled(&gtk_box_g, Some("Signal"),"Сигнал и батарейка");
    stack.add_titled(&flex_box_list_phone, Some("Phone"), "✆Входящие звонки");
    stack.add_titled(&flex_box_contact,Some("Contact"),"Контакты");
    stack.add_titled(&flex_box_sms_input,Some("InputSMS"),"Входящие СМС");
    stack.add_titled(&flex_box_sms_output,Some("OutputSMS"),"СМС");
    stack.add_titled(&flex_box_ussd_command,Some("USSD"),"USSD");
    stack.add_titled(&flex_box_log,Some("Logs"),"✎Лог");
    let stack_switcher = gtk::StackSwitcher::builder()
        .stack(&stack)
        .build();
    stack_switcher.set_css_classes(&["button"]);
    let gtk_box_stack=gtk::Box::builder()
        .orientation(Vertical)
        .halign(Align::Fill)
        .valign(Align::Fill)
        .build();
    gtk_box_stack.append(&stack_switcher);
    stack.set_vexpand(true);
    stack.set_vexpand_set(true);
    gtk_box_stack.append(&stack);
    gtk_box_stack.append(&status);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ydav-gtk v.1.3.0")
        .child(&gtk_box_stack)
        .icon_name("ru_dimon_ydav_2024")
        .build();
    window.set_widget_name("window");
    load_css();
    window.present();


    let connection = Config::sql_connection();
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
    let sender_sms_output = sender.clone();
    let times_sms_output = times.clone();
    let address_ip = edit_ip_address.clone();
    let connection = Config::sql_connection();
    if let Ok(()) = connection.execute(query) {
        let mut statement = connection.prepare( " SELECT * FROM sms_output;").unwrap();
        if statement.iter().count() > 0 {
            while let Ok(State::Row) = statement.next() {
                let sms_output_object = sms_output_object::SmsOutputObject::new();
                sms_output_object.set_property("id", statement.read::<String,_>("_id").unwrap().as_str());
                sms_output_object.set_property("phone", statement.read::<String,_>("phone").unwrap().as_str());
                sms_output_object.set_property("text", statement.read::<String,_>("text").unwrap().as_str());
                sms_output_object.set_property("sent", statement.read::<String,_>("sent").unwrap().as_str());
                sms_output_object.set_property("senttime", statement.read::<String,_>("sent_time").unwrap().as_str());
                sms_output_object.set_property("delivery", statement.read::<String,_>("delivery").unwrap().as_str());
                sms_output_object.set_property("deliverytime", statement.read::<String,_>("delivery_time").unwrap().as_str());
                model_sms_output_object.append(&sms_output_object);
            }
        }
    }


    button_sms_output_send.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        move |_| {
        let phone = edit_sms_output_phone.text().to_string();
        let text = buffer_sms_output_text.text(&buffer_sms_output_text.start_iter(), &buffer_sms_output_text.end_iter(), false).to_string();
        let query = format!("INSERT INTO sms_output (phone, text, sent, sent_time, delivery, delivery_time) VALUES (\"{}\", \"{}\", \"none\", \"none\", \"none\", \"none\");", phone, text);
        let a = " SELECT  last_insert_rowid() AS _id FROM sms_output;";
        let connection = Config::sql_connection();
        if let Ok(()) = connection.execute(query) {
            let mut statement = connection.prepare(a).unwrap();
            if statement.iter().count() > 0 {
                if let Ok(State::Row) = statement.next() {
                    let id = statement.read::<String, _>("_id").unwrap();
                    let now = Local::now();
                    let time_str =  now.format("%Y-%m-%d %H:%M:%S").to_string();
                    let sms_output_object = sms_output_object::SmsOutputObject::new();
                    sms_output_object.set_property("id", id.to_value());
                    sms_output_object.set_property("phone", phone.to_value());
                    sms_output_object.set_property("text", text.to_value());
                    sms_output_object.set_property("sent", "нет информации");
                    sms_output_object.set_property("senttime", &time_str);
                    sms_output_object.set_property("delivery", "нет информации");
                    sms_output_object.set_property("deliverytime", time_str);
                    model_sms_output_object.append(&sms_output_object);
                    let sms_output_param = SmsOutputParam { id, phone, text, };
                    let address = edit_ip_address.text().to_string();
                    let sms_output = match  sms_output::SmsOutput::send(address, sms_output_param){
                        Ok(phone)=>phone,
                        Err(error)=>{
                            times_sms_output.set_markup(format!("{}", error).as_str());
                            let sender = sender_sms_output.clone();
                            sender.send_blocking(true).unwrap();
                            return
                        }
                    };

                    let id_model = model_sms_output_object.find(&sms_output_object).expect("не найден");
                    let item = model_sms_output_object.item(id_model).expect("u");
                    let sms = item.downcast_ref::<sms_output_object::SmsOutputObject>().expect("error");
                    sms.set_property("sent", sms_output.status.status.sent.result);
                    sms.set_property("delivery", sms_output.status.status.delivery.result);
                    sms.set_property("senttime", sms_output.status.status.sent.time);
                    sms.set_property("deliverytime", sms_output.status.status.delivery.time);
                    model_sms_output_object.remove(id_model);
                    model_sms_output_object.insert(id_model,sms)

                }
            }
        }

    }));

    window.connect_close_request(clone!(
        #[weak]
        edit_ip_address,
        #[upgrade_or]
        Propagation::Proceed,
        move |_x1| {
        let mut config = Config::new();
            let address = edit_ip_address.text().to_string();
            config.param.insert("ip".to_string(), address);
        config.save();
        Propagation::Proceed
    }));

    button_contact_csv.connect_clicked(move |_x1| {
        let file_filter = gtk::FileFilter::new();
        file_filter.add_pattern("*.csv");
        file_filter.add_suffix("csv");
        let save_dialog = gtk::FileDialog::builder()
            .title("Файл для контактов")
            .default_filter(&file_filter)
            .build();
        let cancellable =gio::Cancellable::new();
        let value_csv_model_contact = csv_model_contact.clone();
        save_dialog.save(Some(&window), Some(&cancellable), move |x2| {
            let path_file = match x2 {
                Ok(f)=>f.path(),
                Err(_e)=> return
            };
            let path = path_file.unwrap();
            let file_contact_csv = std::fs::File::create(path).expect("Ошибка создания файла!");
            let mut writer = BufWriter::new(file_contact_csv);
            for object in value_csv_model_contact.into_iter(){
                if let Ok(item) = object {
                    let data = item.downcast_ref::<contact_object::ContactObject>()
                        .expect("Item not ContactObject");
                    let phone =data.property::<String>("phone");
                    let name =data.property::<String>("name");
                    if let Err(e)= writer.write_all(format!("{}, {}", name, phone).as_bytes()){
                        println!("{}",e);
                    }
                }
            }
        });

    });
    let mut level: Level = Level(f64::default());
    let mut level_tep: Level = Level(f64::default());

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
    button_phone_get.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        times,
        #[weak]
        label_met_new_phone_input,
        move |_b| {
        let address = edit_ip_address.text().to_string();

        let phone = match Phone::connect(address){
            Ok(phone)=>phone,
            Err(error)=>{
                times.set_markup(format!("{}", error).as_str());
                let sender = sender_info_phone.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };
        times.set_markup(format!("{}", phone.phones.time).as_str());
        let connection = Config::sql_connection();
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
        Config::sql_execute(sql);
        let mut param_where_sql=String::new();
        let mut where_sql=String::new();
        let query = "SELECT _id, id_na_android FROM phone_input WHERE id_na_android>0";
        let connection = Config::sql_connection();
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
            let address = edit_ip_address.text().to_string();
            let phone_input_delete = phone_delete::PhoneDelete::connect(address, &param_where_sql);
            if let Ok(_sid)= phone_input_delete {
                let sql = format!("UPDATE phone_input SET id_na_android=0 WHERE {}", where_sql);
                Config::sql_execute(sql);
                label_met_new_phone_input.set_visible(false);
            }
        }
    }));

    let sender_info_ussd = sender.clone();
    button_send_ussd_command.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        edit_ussd_command,
        #[weak]
        text_result_ussd,
        #[weak]
        times,
        move |_b|{
            let address = edit_ip_address.text().to_string();
            let ussd_command = edit_ussd_command.text().to_string();
            let ussd = match Ussd::send(address, ussd_command){
                Ok(ussd)=> ussd,
                Err(error)=>{
                    times.set_markup(format!("{}", error).as_str());
                    let sender = sender_info_ussd.clone();
                    sender.send_blocking(true).unwrap();
                    return
                }
            };
            let error = ussd.ussd.ussd.failure;
            let response = ussd.ussd.ussd.response;
            text_result_ussd.set_text(format!("{} {}", error, response).as_str());
        }
    ));

    let sender_info_ussd_result = sender.clone();
    button_result_ussd_command.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        times,
        #[weak]
        text_result_ussd,
        move |_b|{
            let address = edit_ip_address.text().to_string();
            let ussd = match Ussd::response(address){
                Ok(ussd)=> ussd,
                Err(error)=>{
                    times.set_markup(format!("{}", error).as_str());
                    let sender = sender_info_ussd_result.clone();
                    sender.send_blocking(true).unwrap();
                    return
                }
            };
            let error = ussd.ussd.ussd.failure;
            let response = ussd.ussd.ussd.response;
            text_result_ussd.set_text(format!("{} {}", error, response).as_str());
        }
    ));

    let sender_info_contact = sender.clone();
    button_contact_get.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        times,
        move |_b|{
        let address = edit_ip_address.text().to_string();
        let contact = match Contact::connect(address){
            Ok(contact)=> contact,
            Err(error)=>{
                times.set_markup(format!("{}", error).as_str());
                let sender = sender_info_contact.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };
        times.set_markup(format!("{}", contact.contacts.time).as_str());
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
        Config::sql_execute(sql);
    }));

    let sender_info_sms_input = sender.clone();
    button_sms_input_get.connect_clicked(clone!(
        #[weak]
        edit_ip_address,
        #[weak]
        times,
        #[weak]
        label_met_new_sms_input,
        move |b|{
        b.set_sensitive(false);
        let address = edit_ip_address.text().to_string();
        let sms_input = match SmsInput::connect(address){
            Ok(sms_input)=> sms_input,
            Err(error)=>{
                times.set_markup(format!("{}", error).as_str());
                let sender = sender_info_sms_input.clone();
                sender.send_blocking(true).unwrap();
                return
            }
        };

        times.set_markup(format!("{}", sms_input.sms_input.time).as_str());
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
        Config::sql_execute(sql);
        let mut param_where_sql=String::new();
        let mut where_sql=String::new();
        let query = "SELECT _id, id_na_android FROM sms_input WHERE id_na_android>0";
        let connection = Config::sql_connection();
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

            let sms_input_delete = sms_input_delete::SmsInputDelete::connect(address_ip.text().to_string().clone(),&param_where_sql);
            if let Ok(_sid)=sms_input_delete{
                let sql = format!("UPDATE sms_input SET id_na_android=0 WHERE {}", where_sql);
                Config::sql_execute(sql);
                label_met_new_sms_input.set_visible(false);
            }
        }

    }));

    let sender_info= sender.clone();
    let regular_monitoring_info= clone!(
        #[weak]
        check_log,
        #[weak]
        edit_ip_address,
        #[weak]
        times,
        #[weak]
        label_met_new_sms_input,
        #[weak]
        button_sms_input_get,
        #[weak]
        label_met_new_phone_input,
        #[upgrade_or]
        ControlFlow::Break,
        move ||{
        let address = edit_ip_address.text().to_string();
        let log= match Info::connect(address) {
            Ok(info)=>{
                gtk_box_horizontal.set_visible(true);
                gtk_box_horizontal2.set_visible(true);
                info
            },
            Err(error)=>{
                gtk_box_horizontal.set_visible(false);
                gtk_box_horizontal2.set_visible(false);
                times.set_markup(format!("{}", error).as_str());
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
        text_signal_param.set_text(format!("{}", log.info.signal.signal_param).as_str());
        times.set_markup(format!("🕰{} СМС:{}", log.info.time, log.info.sms).as_str());
        if check_log.is_active() {
            let log_object = log_object::LogObject::new();
            log_object.set_property("log", format!("Лог:{}", log.json).as_str());
            model_log.append(&log_object)
        }

        if log.info.sms>0{
            label_met_new_sms_input.set_visible(true);
            button_sms_input_get.set_sensitive(true);
            times.set_markup(format!("🕰{} СМС:{}", log.info.time, log.info.sms).as_str());
        }

        if log.info.phone>0{
            label_met_new_phone_input.set_visible(true);
        }

        let flag_regular_monitoring_info =edit_ip_address.get_visible();
        if flag_regular_monitoring_info ==false{ return ControlFlow::Continue
        };
        if flag_regular_monitoring_info ==true {
            return ControlFlow::Break
        };
        ControlFlow::Break


    });
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
    provider.load_from_resource("ru/Dimon/Ydav-gtk/main.css");
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
fn add_panel_is_item(list_item: &Object){
    let label = gtk::Label::builder().build();
    let label_1 = gtk::Label::builder().build();
    let panel = gtk::Box::builder().orientation(Vertical).build();
    panel.append(&label);
    panel.append(&label_1);
    list_item
        .downcast_ref::<ListItem>()
        .expect("error")
        .set_child(Some(&panel));
}



fn find_phone(phone: String, connection: &Connection)->String {
    let statement = connection.prepare(format!("SELECT name FROM contact WHERE phone LIKE '%{}%' ", phone.clone()).as_str()).unwrap();
    if let Ok(s) = find_phone_name(statement){
        return s
    }
    let statement = connection.prepare(format!("SELECT name FROM contact WHERE phone LIKE '%{}%' ", phone[1..].to_string()).as_str()).unwrap();
    if let Ok(s) = find_phone_name(statement){
        return s
    }

    "".to_string()
}

fn find_phone_name(mut statement: Statement) -> Result<String, sqlite::Error>
{
    let mut s="".to_string();
    if statement.iter().count() > 0 {
        while let Ok(State::Row) = statement.next() {
            s = match statement.read::<String, _>("name"){
                Ok(s)=> s,
                Err(e) => return Err(e)
            }
        }
    }
    Ok(s)

}