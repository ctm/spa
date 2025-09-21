use {
    chrono::{DateTime, TimeZone, Utc},
    chrono_tz::Tz,
    js_sys::Object,
    wasm_bindgen::{JsValue, prelude::*},
};

pub(crate) fn utc_now() -> DateTime<Utc> {
    let now = js_sys::Date::now();
    let fsecs = now / 1000.0;
    let secs = fsecs as i64;
    let nsecs = (fsecs.fract() * 1_000_000_000.0) as u32;
    Utc.timestamp_opt(secs, nsecs).unwrap()
}

fn local_timezone() -> Option<String> {
    js_sys::Reflect::get(
        &DateTimeFormat::new0().resolved_options(),
        &JsValue::from_str("timeZone"),
    )
    .ok()
    .and_then(|jsv| JsValue::as_string(&jsv))
}

pub(crate) fn timezone_from_browser_or_mountain() -> Tz {
    local_timezone()
        .and_then(|s| s.parse().ok())
        .unwrap_or(chrono_tz::US::Mountain)
}

// This code was copied from js_sys/lib.rs, so that I could add new0,
// which is new that doesn't take any arguments.  The implementation
// of new in js_sys requires two arguments, but neither are needed for
// our purpose (getting the time zone).

// Intl.DateTimeFormat
#[wasm_bindgen]
extern "C" {
    /// The `Intl.DateTimeFormat` object is a constructor for objects
    /// that enable language-sensitive date and time formatting.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat)
    #[wasm_bindgen(extends = Object, js_namespace = Intl, typescript_type = "Intl.DateTimeFormat")]
    #[derive(Clone, Debug)]
    pub type DateTimeFormat;

    /// The `Intl.DateTimeFormat.prototype.resolvedOptions()` method returns a new
    /// object with properties reflecting the locale and date and time formatting
    /// options computed during initialization of this DateTimeFormat object.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat/resolvedOptions)
    #[wasm_bindgen(method, js_namespace = Intl, js_name = resolvedOptions)]
    pub fn resolved_options(this: &DateTimeFormat) -> Object;

    /// The `Intl.DateTimeFormat` object is a constructor for objects
    /// that enable language-sensitive date and time formatting.
    ///
    /// This version, which takes no arguments, is not present in js_sys.
    ///
    /// [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DateTimeFormat)
    #[wasm_bindgen(constructor, js_namespace = Intl)]
    pub fn new0() -> DateTimeFormat;
}
