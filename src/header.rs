use dioxus::prelude::*;
use crate::asset;

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

            div {
                class: "profile-container",
                profile(cx, false, 3.5)
            }
        }
    ))
}

pub fn profile(cx: Scope, logged: bool, size: f32) -> Element {
    cx.render( rsx!(
        if logged{ rsx! {
            img {
                class: "profile-image",
                style: "height: {size*2.0}vh",
                src:
                if true{
                    asset!("/images/default_profile.png")
                } else {
                    "lol"
                },
                alt: "profile image"
            }
            div {
                class: "user-name-container",
                p {
                    class: "user-name",
                    style: "height: {size}vh",
                    "Name"
                }
            }
        }}else{ rsx! {
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
        }}
    ))
}