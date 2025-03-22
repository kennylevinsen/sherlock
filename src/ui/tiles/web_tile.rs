use gtk4::ListBoxRow;

use super::util::TileBuilder;
use super::Tile;
use crate::launcher::web_launcher::Web;
use crate::launcher::Launcher;

impl Tile {
    pub fn web_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &String,
        web: &Web,
    ) -> (i32, Vec<ListBoxRow>) {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui", index, true);

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&web.icon));

        let tile_name = if web.display_name.contains("{keyword}") {
            web.display_name.replace("{keyword}", keyword)
        } else {
            web.display_name.clone()
        };

        builder.title.set_text(&tile_name);
        builder.display_tag_start(&launcher.tag_start, keyword);
        builder.display_tag_end(&launcher.tag_end, keyword);

        let mut attrs: Vec<(&str, &str)> = vec![
            ("engine", &web.engine),
        ];
        if let Some(next) = launcher.next_content.as_deref() {
            attrs.push(("next_content", next));
        }

        builder.add_default_attrs(Some(&launcher.method), Some(&keyword), Some(&keyword), None, Some(attrs));

        return (index + 1, vec![builder.object]);
    }
}
