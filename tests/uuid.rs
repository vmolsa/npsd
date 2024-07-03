#[cfg(feature = "uuid")]
use npsd::{Payload, Info, Schema, Next};

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "uuid")]
#[derive(Schema, Info, PartialEq, Debug)]
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

    let instance = MultiUuid {
        id0: Uuid::new_v4(),
        id1: Uuid::now_v7(),
        id2: Uuid::new_v4(),
        id3: Uuid::now_v7(),
        id4: Uuid::new_v4(),
        id5: Uuid::now_v7(),
    };

    instance.into_packet(&mut ctx, &mut next).unwrap();

    println!("Encoded: {:?}", next.serialized().hex_dump());

    let deserialized = MultiUuid::from_packet(&mut ctx, &mut next).unwrap();

    assert_eq!(instance, deserialized);
}