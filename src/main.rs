use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use dioxus_desktop::Config;
    let config = Config::default().with_custom_index(include_str!("../index.html").to_string());
    dioxus_desktop::launch_cfg(app, config);
}

fn app(cx: Scope) -> Element {
    render!(
        h1 { "Hello, world!" }
        div {
            id: "map",
            style: r#"
                width: 100%;
                height: 500px;
            "#
        }
        script {
            r#"
                initMap();

                async function initMap() {{
                    // Промис `ymaps3.ready` будет зарезолвлен, когда загрузятся все компоненты основного модуля API
                    await ymaps3.ready;

                    const {{YMap, YMapDefaultSchemeLayer}} = ymaps3;

                    // Иницилиазируем карту
                    const map = new YMap(
                        // Передаём ссылку на HTMLElement контейнера
                        document.getElementById('map'),

                        // Передаём параметры инициализации карты
                        {{
                            location: {{
                                // Координаты центра карты
                                center: [37.588144, 55.733842],

                                // Уровень масштабирования
                                zoom: 10
                            }}
                        }}
                    );

                    // Добавляем слой для отображения схематической карты
                    map.addChild(new YMapDefaultSchemeLayer());
                }}
            "#
        }
    )
}
