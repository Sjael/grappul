use dioxus::prelude::*;
use log::Level;
use crate::utils::{save_to_storage, load_from_storage, clear_from_storage};

mod components;
mod data;
mod routes;
mod utils;

use routes::{cheatsheet::Cheatsheet, home::Home, guide_creator::GuideCreator};
use components::ScrollToTop;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home,
    #[route("/cheatsheet")]
    Cheatsheet,
    #[route("/guide/create")]
    GuideCreator,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilteredClass(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilteredRole(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedRole(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedGod(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectedBuild(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HoveredItem(pub Option<String>);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TooltipPos {
    pub x: i32,
    pub y: i32,
}

impl TooltipPos {
    fn calculate(mouse_x: i32, mouse_y: i32) -> Self {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
        // let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
        
        // Tooltip dimensions (adjust these based on your actual tooltip size)
        let tooltip_width = 400; // matches the CSS width
        // let tooltip_height = 600; // approximate height, adjust as needed
        // let padding = 10; // space between mouse and tooltip
        
        
        // Calculate X position, ensuring tooltip stays within viewport
        let mut x = mouse_x;
        let half_x = tooltip_width / 2;
        
        // Bound x position to keep tooltip within viewport
        if x - half_x < 0 {
            x = half_x;
        } else if x + half_x > width {
            x = width - half_x;
        }
        
        Self { x, y: mouse_y }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(Level::Info));
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    // Initialize selections from localStorage
    let filtered_class = use_context_provider(|| Signal::new(FilteredClass(
        load_from_storage("filtered_class").map(|s| s.to_string())
    )));
    
    let filtered_role = use_context_provider(|| Signal::new(FilteredRole(
        load_from_storage("filtered_role").map(|s| s.to_string())
    )));
    
    let selected_role = use_context_provider(|| Signal::new(SelectedRole(
        load_from_storage("selected_role").map(|s| s.to_string())
    )));
    
    let selected_god = use_context_provider(|| Signal::new(SelectedGod(
        load_from_storage("selected_god").map(|s| s.to_string())
    )));
    
    let selected_build = use_context_provider(|| Signal::new(SelectedBuild(
        load_from_storage("selected_build").map(|s| s.to_string())
    )));

    // Watch for changes and save to localStorage
    use_effect(move || {
        match &filtered_class.read().0 {
            Some(class) => save_to_storage("filtered_class", class),
            None => clear_from_storage("filtered_class"),
        }
        
        match &filtered_role.read().0 {
            Some(role) => save_to_storage("filtered_role", role),
            None => clear_from_storage("filtered_role"),
        }
        
        match &selected_role.read().0 {
            Some(role) => save_to_storage("selected_role", role),
            None => clear_from_storage("selected_role"),
        }
        
        match &selected_god.read().0 {
            Some(god) => save_to_storage("selected_god", god),
            None => clear_from_storage("selected_god"),
        }
        
        match &selected_build.read().0 {
            Some(build) => save_to_storage("selected_build", build),
            None => clear_from_storage("selected_build"),
        }
    });

    use_context_provider(|| Signal::new(HoveredItem::default()));
    let mut mouse_pos = use_context_provider(|| Signal::new(MousePos::default()));
    let mut tooltip_pos = use_context_provider(|| Signal::new(TooltipPos::default()));

    rsx! {
        div {
            onmousemove: move |event: Event<MouseData>| {
                let coords = event.data().page_coordinates();
                let x = coords.x as i32;
                let y = coords.y as i32;
                mouse_pos.write().x = x;
                mouse_pos.write().y = y;
                *tooltip_pos.write() = TooltipPos::calculate(x, y);
            },
            document::Link { rel: "icon", href: asset!("/assets/icons/favicon.ico") }
            document::Link { rel: "stylesheet", href: asset!("/assets/app.css") }

            Router::<Route> {}
            ScrollToTop {}
        }
    }
} 