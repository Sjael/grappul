use dioxus::prelude::*;
use crate::data::gods::GODS;
use crate::components::{Header, ClassFilters, RoleFilters, Explain};
use crate::{SelectedGod, SelectedRole, SelectedClass};

#[component]
pub fn Cheatsheet() -> Element {
    let mut god = use_context::<Signal<SelectedGod>>();
    let role = use_context::<Signal<SelectedRole>>();
    let class = use_context::<Signal<SelectedClass>>();
    // let build = use_context::<Signal<SelectedBuild>>();
    
    // Load and filter gods data
    let filtered_gods: Vec<_> = GODS.iter()
        .filter(|(_, god_info)| {
            let class_matches = class().0.as_ref().map_or(true, |c| c == &god_info.class);
            let role_matches = role().0.as_ref().map_or(true, |r| god_info.roles.contains(r));
            class_matches && role_matches
        })
        .map(|(name, god_info)| (name.clone(), god_info.clone()))
        .collect();

    rsx! {
        div {
            class: "container mx-auto px-4",
            Header {}
            // Filters
            div {
                class: "flex gap-4 mb-8",
                ClassFilters {}
                RoleFilters {}
            }
            
            // God grid section with dark background
            div {
                class: "bg-gray-900 rounded-lg p-6",
                h1 {
                    class: "text-2xl font-bold mb-6 text-white",
                    "Select a God"
                }
                div {
                    class: "grid grid-cols-8 gap-3",
                    for (god_name, _) in filtered_gods {
                        div {
                            key: "{god_name}",
                            class: format!("aspect-square cursor-pointer rounded-lg overflow-hidden transition-all duration-200 {} hover:ring-2 hover:ring-blue-400",
                                if god().0.as_ref().map_or(false, |g| g == &god_name) { 
                                    "ring-2 ring-blue-500 scale-105" 
                                } else { 
                                    "ring-1 ring-gray-700" 
                                }
                            ),
                            onclick: {
                                let god_name = god_name.clone();
                                move |_| {
                                    let current = god().0.clone();
                                    god.set(SelectedGod(
                                        if current.as_ref().map_or(false, |g| g == &god_name) {
                                            None
                                        } else {
                                            Some(god_name.clone())
                                        }
                                    ));
                                }
                            },
                            img {
                                class: "w-full h-full object-cover",
                                src: "/gods/{god_name}.png",
                                alt: "{god_name}"
                            }
                        }
                    }
                }
            }

            // Show explain component if a god is selected
            if god().0.is_some() {
                div {
                    class: "mt-8",
                    Explain {}
                }
            }
        }
    }
} 