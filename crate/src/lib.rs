use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[wasm_bindgen]
pub fn run() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        "https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master",
        &opts,
    )
    .unwrap();

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            // Use serde to parse the JSON into a struct.
            let branch_info: Branch = json.into_serde().unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&branch_info).unwrap())
        });

    console::log_1(&"Hello using web-sys".into());
    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
