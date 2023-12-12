use dioxus::prelude::*;

use dioxus::html::{section, style, a};

pub fn header_fun(cx: Scope) -> Element {
    cx.render( rsx!(
        style {include_str!("web/css/header-styles.css") }
        section {
            id: "header",
            div {
                id: "service-name-container",
                h1{
                    id: "service-name",
                    "TRIP DRIP"
                }
            },
            profile(cx)
        }
    ))
}

fn profile(cx: Scope) -> Element {
    cx.render( rsx!(
        div {
            class: "profile-container",
            if true{ rsx! {
                a {
                    id: "registration",
                    href: "#",
                    "Registration"
                },
                a {
                    id: "login",
                    href: "#",
                    "Login"
                }
            }}else{ rsx! {
                p {
                    id: "user-name",
                    "Name"
                }
                img {
                    class: "profile-image",
                    src:
                    if true{
                        "src/web/images/star.png"
                    } else {
                        ""
                    },
                    alt: "profile image"
                }
            }}
        }
    ))
}