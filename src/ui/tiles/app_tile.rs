use gtk4::{prelude::*, ListBoxRow};
use std::collections::HashMap;

use crate::loader::util::{AppData, Config};
use crate::launcher::Launcher;

use super::util::{ensure_icon_name, TileBuilder};
use super::Tile;

impl Tile {
    pub fn app_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &String,
        commands: HashMap<String, AppData>,
        app_config: &Config,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        for (key, value) in commands.into_iter() {
            if value
                .search_string
                .to_lowercase()
                .contains(&keyword.to_lowercase())
            {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui", index_ref, true);

                let icon = if app_config.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };
                let tile_name = key.replace("{keyword}", keyword);
                builder.display_tag_start(&value.tag_start, keyword);
                builder.display_tag_end(&value.tag_end, keyword);

                if launcher.name.is_empty() {
                    builder.category.set_visible(false);
                }
                builder.category.set_text(&launcher.name);
                builder.icon.set_icon_name(Some(&icon));
                builder.title.set_markup(&tile_name);
                builder.add_default_attrs(
                    Some(&launcher.method), 
                    Some(&keyword),
                    Some(&keyword),
                    Some(&value.exec),
                    None
                );

                index_ref += 1;
                results.push(builder.object);
            }
        }
        return (index_ref, results);
    }
}
