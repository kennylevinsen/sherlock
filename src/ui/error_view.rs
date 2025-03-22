use gtk4::{self, gdk::Key, prelude::*, Builder, EventControllerKey};
use gtk4::{Box as HVBox, ListBox, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use super::util::*;

use crate::APP_STATE;
use crate::{loader::util::SherlockError, ui::tiles::Tile};

pub fn errors(
    errors: &Vec<SherlockError>,
    non_breaking: &Vec<SherlockError>,
) {
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_view.ui");

    let vbox: HVBox = builder.object("vbox").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: ListBox = builder.object("result-frame").unwrap();

    let (_, breaking_error_tiles) = Tile::error_tile(0, errors, "🚨", "ERROR");
    let (_, error_tiles) = Tile::error_tile(0, non_breaking, "⚠️", "WARNING");

    breaking_error_tiles
        .iter()
        .for_each(|tile| results.append(tile));
    error_tiles.iter().for_each(|tile| results.append(tile));
    
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow(){
            state.add_stack_page(vbox, "error-page");
        }
    });
    nav_event(results, result_viewport);

}
fn nav_event(
    result_holder: ListBox,
    result_viewport: ScrolledWindow,
) {
    // Wrap the event controller in an Rc<RefCell> for shared mutability
    let event_controller = Rc::new(RefCell::new(EventControllerKey::new()));

    // Clone Rc references for use in the closure
    let event_controller_clone = Rc::clone(&event_controller);

    event_controller
        .borrow_mut()
        .set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller
        .borrow_mut()
        .connect_key_pressed(move |_, key, _, _| {
            match key {
                Key::Up => {
                    result_holder.focus_prev(&result_viewport);
                    true.into()
                }
                Key::Down => {
                    result_holder.focus_next(&result_viewport);
                    true.into()
                }
                Key::Return => {
                    show_stack_page("search-page", Some(gtk4::StackTransitionType::SlideLeft));

                    // Remove the event controller
                    if let Some(controller) = event_controller_clone
                        .borrow()
                        .clone()
                        .downcast_ref::<EventControllerKey>()
                    {
                        APP_STATE.with(|state|{
                            if let Some(ref state) = *state.borrow(){
                                state.remove_event_listener(controller);
                            }
                        })
                    }

                    true.into()
                }
                _ => false.into(),
            }
        });
    APP_STATE.with(|state|{
        if let Some(ref state) = *state.borrow(){
            state.add_event_listener(event_controller.borrow().clone());
        }
    })
}
