#![windows_subsystem = "windows"]
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
    cx.render(rsx!(
        div {
            id: "map",
            style: r#"
                width: 100%;
                height: 500px;
            "#
        }
        script {
            r#"
                ymaps.ready(init);
                function init(){{
                    // Создание карты.
                    var myMap = new ymaps.Map("map", {{
                        // Координаты центра карты.
                        // Порядок по умолчанию: «широта, долгота».
                        // Чтобы не определять координаты центра карты вручную,
                        // воспользуйтесь инструментом Определение координат.
                        center: [55.76, 37.64],
                        // Уровень масштабирования. Допустимые значения:
                        // от 0 (весь мир) до 19.
                        zoom: 7
                    }});

                    // Добваляем метки на клик по карте
                    myMap.events.add('click', function (e) {{
                        var coords = e.get('coords');
                        myMap.geoObjects.removeAll();
                        myMap.geoObjects.add(new ymaps.Placemark(coords, {{
                            preset: 'islands#icon',
                            iconColor: '#0095b6'
                        }}));
                    }});
                }}
            "#
        }
    ))
}
