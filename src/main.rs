slint::include_modules!();
use std::cell::RefCell;
use std::rc::Rc;

struct AppState {
    counter: i32,  // Feld: Ganzzahl (32-Bit), wie int in .NET.
}

impl AppState {
    fn new() -> Self {  // Self ist ein Alias für AppState.
        AppState { counter: 0 }  // Initialisiert das Struct.
    }

    fn increment_counter(&mut self) -> i32 {
        self.counter += 1;  // Erhöht counter um 1.
        self.counter  // Gibt den Wert zurück (letzte Zeile ohne ; ist return).
    }
}

fn main() {
    let app = MainWindow::new().unwrap();
    let state = Rc::new(RefCell::new(AppState::new()));
    let app_weak = app.as_weak();
    let state_clone = state.clone();

    app.on_button_pressed(move || {
        let new_count = state_clone.borrow_mut().increment_counter();
        let app = app_weak.upgrade().unwrap();
        app.set_counter(new_count);
    });

    app.run().unwrap();
}