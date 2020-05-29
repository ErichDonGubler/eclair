use {
    eclair::eclipse_summary::Summary,
    std::io::Cursor,
    wasm_bindgen::{prelude::wasm_bindgen, JsValue},
};

#[wasm_bindgen]
pub fn parse_ecl_summary(smspec: &[u8], unsmry: &[u8]) -> Result<JsValue, JsValue> {
    let summary = Summary::new(Cursor::new(smspec), Cursor::new(unsmry))
        .map_err(|e| e.to_string())
        .map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&summary).unwrap())
}