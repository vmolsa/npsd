#[cfg(feature = "uuid")]
use npsd::{Payload, Schema};

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "uuid")]
#[derive(Schema, PartialEq, Debug)]
struct MultiUuid {
    id0: Uuid,
    id1: Uuid,
    id2: Uuid,
    id3: Uuid,
    id4: Uuid,
    id5: Uuid,
}

#[cfg(feature = "uuid")]
#[test]
fn test_uuid_payload() {
    use pretty_hex::PrettyHex;

    let instance = MultiUuid {
        id0: Uuid::new_v4(),
        id1: Uuid::now_v7(),
        id2: Uuid::new_v4(),
        id3: Uuid::now_v7(),
        id4: Uuid::new_v4(),
        id5: Uuid::now_v7(),
    };

    let serialized = instance.into_packet(&mut (), 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = MultiUuid::from_packet(&mut (), &serialized).unwrap();

    assert_eq!(instance, deserialized);
}