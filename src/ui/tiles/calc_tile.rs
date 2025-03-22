use gtk4::ListBoxRow;
use meval::eval_str;

use super::util::TileBuilder;
use super::Tile;
use crate::launcher::Launcher;

impl Tile {
    pub fn calc_tile(launcher: &Launcher, index: i32, equation: &str, result: Option<f64>) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let result = if let Some(r) = result {
            r
        } else if let Ok(r) = eval_str(equation){
            r
        } else {
            return (index, results);
        };


        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/calc_tile.ui", index, true);
        builder.equation_holder.set_text(&equation);
        builder
            .result_holder
            .set_text(format!("= {}", result.to_string()).as_str());

        let result = result.to_string();
        builder.add_default_attrs(Some(&launcher.method), Some(&result), None, None, None);
        results.push(builder.object);

        (index, results)
    }
}
