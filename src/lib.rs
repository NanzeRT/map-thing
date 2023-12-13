#[cfg(target_arch = "wasm32")]
pub const STATIC_PATH: &str = ".";
#[cfg(not(target_arch = "wasm32"))]
pub const STATIC_PATH: &str = "src/web";

pub mod header;


#[macro_export]
macro_rules! asset {
    ($path:literal) => {
        const_format::formatcp!("{}/{}", $crate::STATIC_PATH, $path)
    };
}

#[macro_export]
macro_rules! pic_size {
    ($size:expr) => {
        format!("height: {}vh;", $size)
    };
}
#[macro_export]
macro_rules! font_size {
    ($size:expr) => {
        format!("font-size: {}vh;", $size)
    };
}
