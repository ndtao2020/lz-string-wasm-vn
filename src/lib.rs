#[global_allocator]
pub static GLOBAL_ALLOCATOR: &alloc_cat::AllocCat = &alloc_cat::ALLOCATOR;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "convertU16SliceToString")]
    fn convert_u16_slice_to_string(slice: &[u16]) -> String;
}

// ========================================= [HELPER] =========================================
fn from_utf16(decompressed_bytes: &Vec<u16>) -> Result<String, JsValue> {
    match String::from_utf16(&decompressed_bytes) {
        Ok(s) => Ok(s),
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Error converting invalid UTF-16: {}",
                e
            )));
        }
    }
}
// ========================================= [COMPRESS] =========================================
/// Compresses a string to UTF-16
#[wasm_bindgen]
pub fn compress(input: &str) -> Vec<u16> {
    lz_str::compress(input)
}

#[wasm_bindgen(js_name = compressToUTF16)]
pub fn compress_to_utf16(input: &str) -> String {
    lz_str::compress_to_utf16(input)
}

#[wasm_bindgen(js_name = compressToEncodedURIComponent)]
pub fn compress_to_encoded_uri_component(input: &str) -> String {
    lz_str::compress_to_encoded_uri_component(input)
}

#[wasm_bindgen(js_name = compressToBase64)]
pub fn compress_to_base64(input: &str) -> String {
    lz_str::compress_to_base64(input)
}

#[wasm_bindgen(js_name = compressToUint8Array)]
pub fn compress_to_uint8_array(input: &str) -> Vec<u8> {
    lz_str::compress_to_uint8_array(input)
}
// ========================================= [DECOMPRESS] =========================================
/// Decompresses a UTF-16 LZ-String
#[wasm_bindgen]
pub fn decompress(compressed: &[u16]) -> Result<Vec<u16>, JsValue> {
    let decompressed_bytes: Option<Vec<u16>> = lz_str::decompress(compressed);
    if decompressed_bytes.is_none() {
        return Err(JsValue::from_str(&format!("Decompression failed !")));
    }
    Ok(decompressed_bytes.unwrap())
}

#[wasm_bindgen(js_name = decompressFromUTF16)]
pub fn decompress_from_utf16(compressed: &str) -> Result<String, JsValue> {
    let decompressed_bytes: Option<Vec<u16>> = lz_str::decompress_from_utf16(compressed);
    if decompressed_bytes.is_none() {
        return Err(JsValue::from_str(&format!("Decompression failed !")));
    }
    from_utf16(&decompressed_bytes.unwrap())
}

#[wasm_bindgen(js_name = decompressFromEncodedURIComponent)]
pub fn decompress_from_encoded_uri_component(compressed: &str) -> Result<String, JsValue> {
    let decompressed_bytes: Option<Vec<u16>> =
        lz_str::decompress_from_encoded_uri_component(compressed);
    if decompressed_bytes.is_none() {
        return Err(JsValue::from_str(&format!("Decompression failed !")));
    }
    from_utf16(&decompressed_bytes.unwrap())
}

#[wasm_bindgen(js_name = decompressFromBase64)]
pub fn decompress_from_base64(compressed: &str) -> Result<String, JsValue> {
    let decompressed_bytes: Option<Vec<u16>> = lz_str::decompress_from_base64(compressed);
    if decompressed_bytes.is_none() {
        return Err(JsValue::from_str(&format!("Decompression failed !")));
    }
    from_utf16(&decompressed_bytes.unwrap())
}

#[wasm_bindgen(js_name = decompressFromUint8Array)]
pub fn decompress_from_uint8_array(compressed: &[u8]) -> Result<String, JsValue> {
    let decompressed_bytes: Option<Vec<u16>> = lz_str::decompress_from_uint8_array(compressed);
    if decompressed_bytes.is_none() {
        return Err(JsValue::from_str(&format!("Decompression failed !")));
    }
    from_utf16(&decompressed_bytes.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let original = "Hello, World! This is a test string for LZ-String compression.";
        let compressed = compress(original);
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(original, from_utf16(&decompressed).unwrap());
    }

    #[test]
    fn test_compress_decompress_utf16() {
        let original = "Hello, World!";
        let compressed = compress_to_utf16(original);
        let decompressed = decompress_from_utf16(&compressed).unwrap();
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_uri_encoding() {
        let original = "test string with spaces";
        let compressed = compress_to_encoded_uri_component(original);
        let decompressed = decompress_from_encoded_uri_component(&compressed).unwrap();
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_base64_encoding() {
        let original = "test string with spaces";
        let compressed = compress_to_base64(original);
        let decompressed = decompress_from_base64(&compressed).unwrap();
        assert_eq!(original, decompressed);
    }

    #[test]
    fn test_uint8_array() {
        let original = "test string with spaces";
        let compressed = compress_to_uint8_array(original);
        let decompressed = decompress_from_uint8_array(&compressed).unwrap();
        assert_eq!(original, decompressed);
    }
}
