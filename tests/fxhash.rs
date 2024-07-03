#[cfg(feature = "fxhash")]
use npsd::{Payload, Schema, Info};

#[cfg(feature = "fxhash")]
use fxhash::*;

#[cfg(feature = "chrono")]
#[derive(Schema, Info, Clone, PartialEq, Debug)]
struct FxMapSet {
    map1: FxHashMap<String, u32>,
    set1: FxHashSet<String>,
}

#[cfg(feature = "chrono")]
#[test]
fn test_chrono_payload() {
    use pretty_hex::PrettyHex;

    #[cfg(feature = "info")]
    use npsd::NextTrace;

    #[cfg(not(feature = "info"))]
    use npsd::Next;

    let mut ctx = ();

    // Create Middleware
    #[cfg(not(feature = "info"))]
    let mut next = Next::default();

    #[cfg(feature = "info")]
    let mut next = NextTrace::default();

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

    instance.into_packet(&mut ctx, &mut next).unwrap();

    println!("Encoded: {:?}", next.serialized().hex_dump());

    let deserialized = FxMapSet::from_packet(&mut ctx, &mut next).unwrap();

    assert_eq!(instance, deserialized);
}
