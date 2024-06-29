use npsd::{Payload, Schema, Bitmap};

#[derive(Schema, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[test]
fn test_schema() {
    // Create an instance of `Animal`.
    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    // Serialize the `animal` instance into a packet.
    let serialized = animal.into_packet(&mut (), 1470).unwrap();

    // Deserialize the packet back into an `Animal` instance.
    let deserialized = Animal::from_packet(&mut (), serialized).unwrap();

    // Ensure the deserialized instance matches the original.
    assert_eq!(deserialized, animal);
}

#[derive(Bitmap, PartialEq, Debug)]
struct Flags {
    a: bool,
    b: bool,
    c: bool,
}

#[test]
fn test_bitmap() {
    let flags = Flags { a: true, b: false, c: true };

    let serialized = flags.into_packet(&mut (), 1470).unwrap();
    let deserialized = Flags::from_packet(&mut (), serialized).unwrap();

    assert_eq!(deserialized, flags);
}
