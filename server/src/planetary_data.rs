//! Provenance-aware catalog of body-specific planetary map sources.
//! Catalogued sources are not represented as ingested datasets.
use axum::Json;
use serde_json::{json, Value};

const BODY_CATALOG: &str = include_str!("../../Assets/metadata/planetary-body-catalog.json");
const SOURCE_CATALOG: &str = include_str!("../../Assets/metadata/planetary-data-sources.json");

pub async fn body_catalog() -> Json<Value> {
    Json(serde_json::from_str(BODY_CATALOG).expect("planetary body catalog must be valid JSON"))
}
pub async fn source_catalog() -> Json<Value> {
    Json(serde_json::from_str(SOURCE_CATALOG).expect("planetary source catalog must be valid JSON"))
}
fn data_status_value() -> Value {
    json!({
        "schema_version": "planetary-data-status.v1",
        "policy": "Never render a body-specific scientific layer on a different celestial body.",
        "source_catalog": "catalogued",
        "raw_dataset_ingestion": "not_claimed_by_catalog",
        "body_specific_status": [
            {"body_id":"sol-earth","map_status":"existing_project_geojson_plus_source_catalogue","data_kind":"existing simplified geographic vectors"},
            {"body_id":"sol-moon","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"sol-mars","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"sol-mercury","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"sol-venus","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"sol-ceres","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"sol-titan","map_status":"catalogued_not_ingested","fallback":"seeded procedural terrain"},
            {"body_id":"procedural-exoplanet","map_status":"procedural_preview","fallback":"seeded procedural terrain"}
        ],
        "required_provenance_fields": ["body_id","layer_id","source_name","source_url","product_id","product_version","license","coordinate_reference_system","resolution_m","units","ingestion_status"],
        "warning": "Catalog entries are source discovery metadata, not evidence that external scientific products have been downloaded or validated."
    })
}
pub async fn data_status() -> Json<Value> { Json(data_status_value()) }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_in_catalogs_are_valid_json() {
        let body: Value = serde_json::from_str(BODY_CATALOG).unwrap();
        let sources: Value = serde_json::from_str(SOURCE_CATALOG).unwrap();
        assert!(body["bodies"].as_array().is_some_and(|items| !items.is_empty()));
        assert!(sources.is_object());
    }
    #[test]
    fn status_does_not_claim_data_was_ingested() {
        let status = data_status_value();
        assert_eq!(status["raw_dataset_ingestion"], "not_claimed_by_catalog");
        assert_eq!(status["body_specific_status"][1]["map_status"], "catalogued_not_ingested");
    }
}