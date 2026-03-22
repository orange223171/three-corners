#[cfg(test)]
use super::*;

#[test]
fn deserialize_building_info() {
    let json: String = std::fs::read_to_string("data/synergies/farm.json").unwrap();
    let _synergy_info: SynergyInfo = serde_json::from_str(json.as_str()).unwrap();

    assert!(true);
}
