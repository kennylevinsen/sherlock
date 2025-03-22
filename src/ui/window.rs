use gtk4::{gdk, Builder, Stack};
use gtk4::{prelude::*, Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Layer, LayerShell};

use crate::CONFIG;


pub fn window(application: &Application) -> (ApplicationWindow, Stack) {
    // 618 with, 591 without notification bar
    let (width, height) = if let Some(config) = CONFIG.get() {
        (config.appearance.width, config.appearance.height)
    } else {
        (900, 591)
    };

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(application)
        .default_width(width)
        .default_height(height)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    //Build main fame here that holds logic for stacking
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/window.ui");
    let holder: Stack = builder.object("stack").unwrap();


    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => (),
        }
        false.into()
    });
    window.add_controller(event_controller);
    window.set_child(Some(&holder));
    return (window, holder);
}
