#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn LoadingSkeleton() -> Element {
    rsx! {
        div {
            class: "loading-skeleton",
            style: "padding: 3rem 0;",
            
            // God name skeleton
            div {
                class: "skeleton-line",
                style: "height: 32px; width: 200px; margin-bottom: 1rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
            
            // Role tabs skeleton
            div {
                style: "display: flex; gap: 1rem; margin-bottom: 2rem;",
                div {
                    class: "skeleton-line",
                    style: "height: 40px; width: 80px; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                }
                div {
                    class: "skeleton-line",
                    style: "height: 40px; width: 80px; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                }
                div {
                    class: "skeleton-line",
                    style: "height: 40px; width: 80px; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                }
            }
            
            // Build section skeleton
            div {
                style: "margin-bottom: 2rem;",
                div {
                    class: "skeleton-line",
                    style: "height: 24px; width: 120px; margin-bottom: 1rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(6, 1fr); gap: 0.5rem;",
                    {(0..6).map(|i| rsx! {
                        div {
                            key: "{i}",
                            class: "skeleton-item",
                            style: "aspect-ratio: 1; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                        }
                    })}
                }
            }
            
            // Content lines skeleton
            div {
                class: "skeleton-line",
                style: "height: 16px; width: 100%; margin-bottom: 0.75rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
            div {
                class: "skeleton-line",
                style: "height: 16px; width: 80%; margin-bottom: 0.75rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
            div {
                class: "skeleton-line",
                style: "height: 16px; width: 100%; margin-bottom: 0.75rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
            div {
                class: "skeleton-line",
                style: "height: 16px; width: 80%; margin-bottom: 0.75rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
            div {
                class: "skeleton-line",
                style: "height: 16px; width: 100%; margin-bottom: 0.75rem; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
            }
        }
    }
}

#[component]
pub fn CheatsheetLoadingSkeleton() -> Element {
    rsx! {
        div {
            class: "cheatsheet-loading",
            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 15px; padding: 20px;",
            
            {(0..12).map(|i| rsx! {
                div {
                    key: "{i}",
                    class: "cheatsheet-skeleton",
                    style: "height: 180px; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 8px;",
                }
            })}
        }
    }
}

#[component]
pub fn GodGridLoadingSkeleton() -> Element {
    rsx! {
        div {
            class: "god-grid-loading",
            style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 0.5rem; padding: 1rem 0;",
            
            {(0..9).map(|i| rsx! {
                div {
                    key: "{i}",
                    class: "god-skeleton",
                    style: "aspect-ratio: 1; background: linear-gradient(90deg, var(--color-bg-secondary) 25%, var(--color-bg-tertiary) 50%, var(--color-bg-secondary) 75%); background-size: 200% 100%; animation: skeleton-loading 1.5s infinite; border-radius: 4px;",
                }
            })}
        }
    }
}