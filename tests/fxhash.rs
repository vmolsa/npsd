#[cfg(feature = "fxhash")]
use npsd::{Payload, Schema};

#[cfg(feature = "fxhash")]
use fxhash::*;

#[cfg(feature = "chrono")]
#[derive(Schema, Clone, PartialEq, Debug)]
struct FxMapSet {
    map1: FxHashMap<String, u32>,
    set1: FxHashSet<String>,
}

#[cfg(feature = "chrono")]
#[test]
fn test_chrono_payload() {
    use pretty_hex::PrettyHex;

    let instance = FxMapSet {
        map1: FxHashMap::from(vec![
            ("Matti".to_string(), 1337u32),
            ("Teppo".to_string(), 1337u32),
            ("Alice".to_string(), 1337u32),
            ("Bob".to_string(), 1337u32),
        ].into_iter().collect()),
        set1: FxHashSet::from(vec![
            "Matti".to_string(),
            "Teppo".to_string(),
            "Alice".to_string(),
            "Bob".to_string(),
        ].into_iter().collect()),
    };

    let serialized = instance.into_packet(&mut (), 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = FxMapSet::from_packet(&mut (), &serialized).unwrap();

    assert_eq!(instance, deserialized);
}
