#![windows_subsystem = "windows"]

mod header;
use crate::header::header_fun;

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use dioxus_desktop::Config;
    let config = Config::default().with_custom_index(include_str!("web/index.html").to_string());
    dioxus_desktop::launch_cfg(main_page, config);
}

fn main_page(cx: Scope) -> Element {
    cx.render(rsx!(
        header_fun(cx),

        style {include_str!("web/css/main-page-styles.css") }
        section {
            id: "search-row",
            form {
                input {
                    id: "search-row-field",
                    name: "Query",
                    r#type: "search",
                    placeholder: "search",
                }
                input {
                    id: "search-row-submit",
                    r#type: "submit",
                    "Поиск"
                }
            }
        }
        section {
            id: "route-preview-container",
            for i in 0..10{
                route_preview(cx)
                if i < 9 { rsx! {
                    br {
                        class: "routes-splitter"
                    }
                }}
            },
        }
    ))
}

fn route_preview(cx: Scope) -> Element {
    cx.render( rsx!(
        div {
            class: "route-preview",
            div{
                class: "route-name-and-star-container",
                div{
                    class: "route-name-container",
                    h3 {
                        class: "route-name",
                        "Name"
                    }
                }
                div{
                    class: "stars-container",
                    for star in 1..5{
                        img {
                            class: "stars-img",
                            src: "src/web/images/star.png",
                            alt: "star"
                        }
                    }
                }
            }
            h4{
                class: "autor-name",
                "Author"
            }
            div{
                class: "tags-container",
                for tag in 0..5{
                    div {
                        class: "simple-tag",
                            "tag"
                    }
                }
            }
        }
    ))
}


fn route_page(cx: Scope) -> Element{
    cx.render( rsx!(
        header_fun(cx),

        style {include_str!("web/css/route-page-styles.css") }
        h1 {
            id: "route-page-name"
        }
        section {
            id: "info-container",
            div{
                id: "map"
            }
            div{
                class: "autor-name",
                "Author"
            }
            div{
                class: "stars",
                for star in 1..5{
                    img {
                        src: "src/web/images/star.png",
                        alt: "star"
                    }
                }
            }
            div{
                class: "tags-container",
                for tag in 0..5{
                    div {
                        class: "simple-tag",
                        "tag"
                    }
                }
            }
        }
        section {
            id: "description-container",
            p {
                id: "description"
            }
        }
        script {include_str!("web/scripts/map-logic.ts") }
    ))
}