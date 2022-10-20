use leptos::*;
use todomvc::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::debug!("initialized logging");

    leptos::hydrate(body().unwrap(), |cx| {
        // initial state — identical to server
        let todos = Todos(vec![
            Todo::new(cx, 0, "Buy milk".to_string()),
            Todo::new(cx, 1, "???".to_string()),
            Todo::new(cx, 2, "Profit!".to_string()),
        ]);

        view! { cx,  <TodoMVC todos=todos/> }
    });
}
