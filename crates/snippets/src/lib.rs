use js_sys::Promise;
use playground_api::Client;
use playground_api::endpoints::{Channel, CrateType, Edition, ExecuteRequest, Mode};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn run_rust_snippet_async(code: String) -> Promise {
    future_to_promise(async move {
        let client = Client::new("https://play.rust-lang.org").map_err(|e| {
            JsValue::from_str(
                &json!({"error": format!("Client creation failed: {:?}", e)}).to_string(),
            )
        })?;

        let req = ExecuteRequest::new(
            Channel::Stable,
            Mode::Release,
            Edition::Edition2021,
            CrateType::Binary,
            false, // backtrace
            false, // test mode
            code,
        );

        match client.execute(&req).await {
            Ok(res) => Ok(JsValue::from_str(
                &serde_json::to_string(&res)
                    .unwrap_or_else(|_| "{\"error\":\"serialization failed\"}".to_string()),
            )),
            Err(e) => Ok(JsValue::from_str(
                &json!({"error": format!("{:?}", e)}).to_string(),
            )),
        }
    })
}
