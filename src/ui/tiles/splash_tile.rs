use super::Tile;
use crate::launcher::Launcher;
use gio::{prelude::*, BusType, Cancellable, DBusProxy, DBusProxyFlags};
use gtk4::{glib, prelude::WidgetExt, Builder, Label, ListBoxRow, ProgressBar};

fn update_time(time_label: &Label, date_top: &Label, date_bottom: &Label) {
    let now = chrono::Local::now();
    time_label.set_text(&now.format("%R").to_string());
    date_top.set_text(&now.format("%A, week %U").to_string());
    date_bottom.set_text(&now.format("%d %B %Y").to_string());
}

impl Tile {
    pub fn splash_tile(_launcher: &Launcher, index: i32, keyword: &str) -> (i32, Vec<ListBoxRow>) {
        if index != 0 || !keyword.is_empty() {
            return (index, vec![]);
        }
        let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/splash.ui");

        let time_label: Label = builder.object("time-bar").unwrap();
        let date_top: Label = builder.object("date-top").unwrap();
        let date_bottom: Label = builder.object("date-bottom").unwrap();
        let bat_value: ProgressBar = builder.object("battery-value").unwrap();

        update_time(&time_label, &date_top, &date_bottom);

        glib::timeout_add_local(std::time::Duration::from_millis(5000), move || {
            update_time(&time_label, &date_top, &date_bottom);
            glib::ControlFlow::Continue
        });

        let cancel = Cancellable::new();
        DBusProxy::for_bus(
            BusType::System,
            DBusProxyFlags::NONE,
            None,
            "org.freedesktop.UPower",
            "/org/freedesktop/UPower/devices/DisplayDevice",
            "org.freedesktop.UPower.Device",
            Some(&cancel),
            move |res| {
                if let Ok(proxy) = res {
                    if let Some(state) = proxy.cached_property("State") {
                        let value: u32 = state.get().unwrap();
                        match value {
                            1 => bat_value.add_css_class("battery-charging"),
                            2 => bat_value.add_css_class("battery-discharging"),
                            3 => bat_value.add_css_class("battery-empty"),
                            4 => bat_value.add_css_class("battery-fully-charged"),
                            5 => bat_value.add_css_class("battery-pending-charge"),
                            6 => bat_value.add_css_class("battery-pending-discharge"),
                            _ => (),
                        };
                    }
                    if let Some(cap) = proxy.cached_property("Percentage") {
                        let value: f64 = cap.get().unwrap();
                        bat_value.set_fraction(value / 100.0);
                    }
                    if let Some(cap) = proxy.cached_property("WarningLevel") {
                        let value: u32 = cap.get().unwrap();
                        match value {
                            3 => bat_value.add_css_class("battery-low"),
                            4 => bat_value.add_css_class("battery-critical"),
                            5 => bat_value.add_css_class("battery-action"),
                            _ => (),
                        }
                    }
                }
            },
        );

        let object: ListBoxRow = builder.object("holder").unwrap_or_default();
        return (index + 2, vec![object]);
    }
}
