use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};

use crate::{const_parse_okid, OkId};

impl JsonSchema for OkId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OkId".into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::OkId").into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        let version = env!("CARGO_PKG_VERSION");
        const TEST_OKID: &str =
            "2ːb94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        const PARSED: OkId = const_parse_okid(TEST_OKID).unwrap();
        json_schema!({
            "type": "string",
            "format": "okid",
            "description": format!(
                "[OkId v{}](https://docs.rs/okid/{})",
                version, version
            ),
            "examples": [
                PARSED.to_string()
            ]
        })
    }
}
