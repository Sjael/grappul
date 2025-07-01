use dioxus::prelude::*;
use log::Level;

mod components;
mod data;
mod routes;

use components::{Header, ClassFilters, RoleFilters};
use routes::{cheatsheet::Cheatsheet, home::Home};

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home,
    #[route("/cheatsheet")]
    Cheatsheet,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedClass(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedRole(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedGod(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedBuild(pub Option<String>);

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Info));
    dioxus::launch(app);
}

fn app() -> Element {
    use_context_provider(|| SelectedClass::default());
    use_context_provider(|| SelectedRole::default());
    use_context_provider(|| SelectedGod::default());
    use_context_provider(|| SelectedBuild::default());


    rsx! {
        div {
            // Header with dark mode toggle
            Header {}

            // Main content
            main {
                // Filters
                ClassFilters {}
                RoleFilters {}

                // Router
                div {
                    Router::<Route> {}
                }
            }
        }
    }
} 