
# npsd (Network Payload Serializer / Deserializer)

The `npsd` crate provides a flexible and efficient way to serialize and deserialize network payloads. It supports converting Rust types into byte streams suitable for network transmission and reconstructing those types from byte streams received over the network. This is particularly useful for networked applications that require efficient and reliable data exchange.

## Features
- Serialize and deserialize complex Rust types to and from byte streams.
- Support for custom serialization contexts.
- Middleware support for extensible processing during serialization/deserialization.

## Traits

### `PayloadContext`

The `PayloadContext` trait provides a way to unwrap the context used in the payload processing.

### `Middleware`

The `Middleware` trait defines methods for converting types to and from payloads of bytes.

### `IntoPayload`

The `IntoPayload` trait is used to convert a type into a payload of bytes.

### `FromPayload`

The `FromPayload` trait is used to convert a payload of bytes back into a type.

### `PayloadInfo`

The `PayloadInfo` trait provides metadata about the payload.

### `Payload`

The `Payload` trait combines `IntoPayload` and `FromPayload` to facilitate complete serialization and deserialization of types.

## License

This project is licensed under the Apache 2.0 License.

## Procedural Macros

The `npsd` crate also provides procedural macros for deriving serialization and deserialization implementations.

### `Schema`

The `Schema` macro derives implementations for serializing and deserializing complex Rust types.

#### Example

```rust
use npsd::{Payload, Schema};

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
```

### `Bitmap`

The `Bitmap` macro derives implementations for serializing and deserializing bitmaps.

#### Example

```rust
use npsd::{Payload, Bitmap};

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
```