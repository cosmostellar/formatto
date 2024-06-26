use std::error::Error;
use utils::Preferences;
use wasm_bindgen::prelude::*;

mod option_schema;
mod tools;
mod utils;

#[cfg(test)]
mod testing;

#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{}", s);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn error(s: &str) {
    eprintln!("{}", s);
}

mod macro_rules {
    #[macro_export]
    macro_rules! console_log {
        ($($arg:tt)*) => ($crate::log(&format!($($arg)*)));
    }
    #[macro_export]
    macro_rules! console_error {
        ($($arg:tt)*) => ($crate::error(&format!($($arg)*)));
    }
}

#[wasm_bindgen]
/// This function will be called from the TypeScript side.
pub fn format_document(input: &str, js_options: JsValue, js_locales: JsValue) -> String {
    use utils::{read_js_value, read_options};

    utils::set_panic_hook();

    let options = match read_options(js_options) {
        Ok(options) => options,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    let locales = match read_js_value(js_locales) {
        Ok(locales) => locales,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    if input.is_empty() {
        return input.to_string();
    }

    let preferences = Preferences { options, locales };

    // Return output to the TypeScript side or throw an error.
    match parse_input(input, &preferences) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    }
}

/// Parses an input and returns a formatted string.
fn parse_input(input: &str, preferences: &Preferences) -> Result<String, Box<dyn Error>> {
    let sections = tools::parsing::get_sections(input, preferences)?;
    let output = tools::formatting::get_formatted_string(sections, preferences)?;

    Ok(output)
}
