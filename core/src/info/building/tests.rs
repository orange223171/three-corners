#[cfg(test)]
use super::*;

#[test]
fn deserialize_building_info() {
    let json: String = std::fs::read_to_string("data/buildings/townhall.json").unwrap();
    let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

    assert!(true);
}

#[test]
fn deserialize_building_info_with_effect() {
    let json: String = std::fs::read_to_string("data/buildings/windmill.json").unwrap();
    let _building_info: BuildingInfo = serde_json::from_str(json.as_str()).unwrap();

    assert!(true);
}
