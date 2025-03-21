use gtk4::{Builder, Label, ListBoxRow, glib};

use super::Tile;
use crate::launcher::Launcher;

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

        update_time(&time_label, &date_top, &date_bottom);

        glib::timeout_add_local(std::time::Duration::from_millis(5000), move || {
            update_time(&time_label, &date_top, &date_bottom);
            glib::ControlFlow::Continue
        });

        // builder.category.set_text(name);
        // builder.icon.set_icon_name(Some(icon));
        // builder.title.set_text(keyword);
        //builder.add_default_attrs(Some(&launcher.method), None, Some(keyword), None, None);
        
        let object: ListBoxRow = builder.object("holder").unwrap_or_default();
        return (index + 1, vec![object]);
    }
}
