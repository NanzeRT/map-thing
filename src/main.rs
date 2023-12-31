#![windows_subsystem = "windows"]

use std::sync::Mutex;
use dioxus::html::{section, style};
use dioxus_test::header::{Header, Profile};
use dioxus_test::asset;
use dioxus::prelude::*;

const IP: &str = "192.168.151.185";

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(MainPage);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use dioxus_desktop::Config;
    let config = Config::default().with_custom_index(include_str!("web/index.html").to_string());
    dioxus_desktop::launch_cfg(MainPage, config);
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Route {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub author: User,
    pub stars: i32,
}
impl Default for Route {
    fn default() -> Self {
        Route {
            id: -1,
            name: "Route name".into(),
            description: Some("".into()),
            author: User { id: -1, name: "Author".into()},
            stars: 5
        }
    }
}

#[component]
fn MainPage(cx: Scope) -> Element {
    let val = use_state(cx, || String::from(""));

    let fut = use_future(cx, val, |val| async move {
        serde_json::from_str::<Vec<Route>>(
            &reqwest::get(format!("http://{}:3000/routes?{}", IP, val))
                .await.unwrap()
                .text().await.unwrap()
        )
    });

    let onsubmit = move |evt: FormEvent| {
        val.set(evt.values["Query"][0].clone());
    };

    cx.render(rsx!(
        Header {}

        style { include_str!("web/css/main-page-styles.css") }
        section { id: "search-row",
            form {
                onsubmit: onsubmit,
                input {
                    id: "search-row-field",
                    name: "Query",
                    r#type: "search",
                    placeholder: "search"
                }
                input { id: "search-row-submit", r#type: "submit", "Поиск" }
            }
        }
        section { id: "route-preview-container",
            match fut.value(){
                Some(Ok(resp)) => rsx! {
                    for i in 0..resp.len() {
                        RoutePreview {r: resp[i].clone()}
                        if i < resp.len()-1 { rsx! {
                            br {
                                class: "routes-splitter"
                            }
                        }}
                    }
                },
                Some(Err(_)) => rsx! {
                    RoutePreview {r: Route::default()}
                },
                None => rsx! {RoutePreview {r: Route::default()}},
            }
        }
    ))
}

#[component]
fn RoutePreview(cx: Scope, r: Route) -> Element {
    cx.render( rsx!(
        div { class: "route-preview",
            div { class: "route-name-and-star-container",
                div { class: "route-name-container", h3 { class: "route-name", r.name.clone() } }
                div { class: "stars-container",
                    for _ in 0..r.stars {
                        img { class: "stars-img", src: asset!("/images/star.png"), alt: "star" }
                    }
                }
            }
            h4 { class: "autor-name", "Author" }
            div { class: "tags-container",
                for tag in 0..5 {
                    div { class: "simple-tag", "tag" }
                }
            }
        }
    ))
}

#[component]
fn RoutePage(cx: Scope) -> Element{
    cx.render( rsx!(
        Header {}

        style { include_str!("web/css/route-page-styles.css") }
        div { class: "left-right-container",
            div { class: "route-name-container", h1 { id: "route-page-name", "The route" } }
            div { class: "row-container",
                for star in 1..5 {
                    div { class: "row-element", img { class: "stars-img", src: asset!("/images/star.png"), alt: "star" } }
                }
            }
        }
        div { id: "map" }
        section { class: "left-right-container",
            for tag in 0..5 {
                div { class: "tag", p { class: "tag-text", "tag" } }
            }
        }
        div { class: "left-right-container", Profile { logged: true, size: 3.0 } }
        section { id: "description-container",
            p { id: "description",
                r##"Давно выяснено, что при оценке дизайна и композиции читаемый текст мешает сосредоточиться. Lorem Ipsum используют потому, что тот обеспечивает более или менее стандартное заполнение шаблона, а также реальное распределение букв и пробелов в абзацах, которое не получается при простой дубликации "Здесь ваш текст.. Здесь ваш текст.. Здесь ваш текст.." Многие программы электронной вёрстки и редакторы HTML используют Lorem Ipsum в качестве текста по умолчанию, так что поиск по ключевым словам "lorem ipsum" сразу показывает, как много веб-страниц всё ещё дожидаются своего настоящего рождения. За прошедшие годы текст Lorem Ipsum получил много версий. Некоторые версии появились по ошибке, некоторые - намеренно (например, юмористические варианты).
                сть много вариантов Lorem Ipsum, но большинство из них имеет не всегда приемлемые модификации, например, юмористические вставки или слова, которые даже отдалённо не напоминают латынь. Если вам нужен Lorem Ipsum для серьёзного проекта, вы наверняка не хотите какой-нибудь шутки, скрытой в середине абзаца. Также все другие известные генераторы Lorem Ipsum используют один и тот же текст, который они просто повторяют, пока не достигнут нужный объём. Это делает предлагаемый здесь генератор единственным настоящим Lorem Ipsum генератором. Он использует словарь из более чем 200 латинских слов, а также набор моделей предложений. В результате сгенерированный Lorem Ipsum выглядит правдоподобно, не имеет повторяющихся абзацей или "невозможных" слов.
                "But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure?"
                "On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains."
                r"##
            }
        }
        section { id: "comments",
            h3 { "Comments" }
            for _ in 0..10 {
                div { class: "comment-container",
                    div { class: "left-right-container", style: "width: 100%",
                        Profile { logged: true, size: 2.5 }
                        div { class: "row-container",
                            for star in 1..2 {
                                div { class: "row-element", img { class: "stars-img", src: asset!("/images/star.png"), alt: "star" } }
                            }
                        }
                    }
                    div { class: "comment-text",
                        p {
                            r"Dавно выяснено, что при оценке дизайна и композиции читаемый текст мешает сосредоточиться. Lorem Ipsum используют \

                            иользуют\
                            "
                        }
                    }
                }
            }
        }
        script { include_str!("web/scripts/map-logic.ts") }
    ))
}