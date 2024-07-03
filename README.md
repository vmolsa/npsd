
# npsd (Network Payload Serializer / Deserializer)

The `npsd` crate provides a flexible and efficient way to serialize and deserialize network payloads. It supports converting Rust types into byte streams suitable for network transmission and reconstructing those types from byte streams received over the network. This is particularly useful for networked applications that require efficient and reliable data exchange.

## Features
- Serialize and deserialize complex Rust types to and from byte streams.
- Support for custom serialization contexts.
- Middleware support for extensible processing during serialization/deserialization.

## Differences between `npsd` and `serde`

- **Purpose**: `npsd` is designed specifically for network payload serialization and deserialization, offering efficient handling of byte streams for network transmission. `serde`, on the other hand, is a general-purpose serialization framework supporting multiple formats (JSON, XML, etc.).

- **Custom Contexts**: `npsd` supports custom serialization contexts and middleware for extensible processing during serialization/deserialization, which is tailored for networked applications. `serde` focuses on format-agnostic serialization.

- **Procedural Macros**: Both libraries provide procedural macros, but `npsd` includes specific macros (`Schema`, `Bitmap`, `AsyncSchema`, `AsyncBitmap`, `Info`) for network-related serialization scenarios.

## Procedural Macros

The `npsd` crate also provides procedural macros for deriving serialization and deserialization implementations.

### `Schema`

The `Schema` macro derives implementations for serializing and deserializing complex Rust types.

#### Example

```rust
use npsd::{Payload, Schema, Next, Middleware};

#[derive(Schema, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[test]
fn test_schema() {
    // Create Middleware
    let mut next = Next::default();

    // Create an instance of `Animal`.
    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    // Serialize the `animal` instance into a packet.
    animal.into_packet(&mut (), &mut next).unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    // Deserialize the packet back into an `Animal` instance.
    let deserialized = Animal::from_packet(&mut (), &mut next).unwrap();

    // Ensure the deserialized instance matches the original.
    assert_eq!(deserialized, animal);
}
```

### `Bitmap`

The `Bitmap` macro derives implementations for serializing and deserializing bitmaps.

#### Example

```rust
use npsd::{Payload, Bitmap, Next, Middleware};

#[derive(Bitmap, PartialEq, Debug)]
struct Flags {
    a: bool,
    b: bool,
    c: bool,
}

#[test]
fn test_bitmap() {
    // Create Middleware
    let mut next = Next::default();

    // Create an u8 bitmap of `Flags`.
    let flags = Flags { a: true, b: false, c: true };

    // Serialize the `Flags` into a packet.
    flags.into_packet(&mut (), &mut next).unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    // Deserialize the packet back into an `Flags`.
    let deserialized = Flags::from_packet(&mut (), &mut next).unwrap();

    // Ensure the deserialized matches the original.
    assert_eq!(deserialized, flags);
}
```

### `AsyncSchema`

The `AsyncSchema` macro derives implementations for asynchronous serializing and deserializing complex Rust types.

#### Example

```rust
use npsd::{AsyncPayload, AsyncSchema, Next, Info};

#[derive(AsyncSchema, Info, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[tokio::test]
async fn test_schema() {
    // Create Middleware
    let mut next = Next::default();

    // Create an instance of `Animal`.
    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    // Serialize the `animal` instance into a packet.
    animal.into_packet(&mut (), &mut next).await.unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    // Deserialize the packet back into an `Animal` instance.
    let deserialized = Animal::from_packet(&mut (), &mut next).await.unwrap();

    // Ensure the deserialized instance matches the original.
    assert_eq!(deserialized, animal);
}
```

### `AsyncBitmap`

The `AsyncBitmap` macro derives implementations for asynchronous serializing and deserializing bitmaps.

#### Example

```rust
use npsd::{AsyncPayload, AsyncBitmap, Next, Info};

#[derive(Bitmap, PartialEq, Debug)]
struct Flags {
    a: bool,
    b: bool,
    c: bool,
}

#[test]
fn test_bitmap() {
    // Create Middleware
    let mut next = Next::default();

    // Create an u8 bitmap of `Flags`.
    let flags = Flags { a: true, b: false, c: true };

    // Serialize the `Flags` into a packet.
    flags.into_packet(&mut (), &mut next).await.unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    // Deserialize the packet back into an `Flags`.
    let deserialized = Flags::from_packet(&mut (), &mut next).await.unwrap();

    // Ensure the deserialized matches the original.
    assert_eq!(deserialized, flags);
}
```

## Traits

### `PayloadContext`

The `PayloadContext` trait provides a way to unwrap the context used in the payload processing.

### `Middleware`

The `Middleware` trait defines methods for converting types to and from payloads of bytes.

### `AsyncMiddleware`

The `AsyncMiddleware` trait defines asynchronous methods for converting types to and from payloads of bytes.

### `IntoPayload`

The `IntoPayload` trait is used to convert a type into a payload of bytes.

### `AsyncIntoPayload`

The `AsyncIntoPayload` trait is used for asynchronous methods for converting types into payloads of bytes.

### `FromPayload`

The `FromPayload` trait is used to convert a payload of bytes back into a type.

### `AsyncFromPayload`

The `AsyncFromPayload` trait is used for asynchronous methods for converting payloads of bytes back into types.

### `PayloadInfo`

The `PayloadInfo` trait provides metadata about the payload. Here are the associated constants and their descriptions:

- **`const HASH: u64`**: A constant hash value associated with the type. This hash is calculated using the type's string representation and provides a unique identifier for the payload type.
- **`const TYPE: &'static str`**: A string representing the type of the payload. This is used to identify the payload type in a human-readable format.
- **`const SIZE: Option<usize>`**: An optional constant representing the size of the payload. This can be used to specify a fixed size for the payload, if applicable.

### `Payload`

The `Payload` trait combines `IntoPayload` and `FromPayload` to facilitate complete serialization and deserialization of types.

### `AsyncPayload`

The `AsyncPayload` trait combines `AsyncIntoPayload` and `AsyncFromPayload` to asynchronous methods for complete serialization and deserialization of types.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you encounter any problems or have suggestions for improvements.

## License

This project is licensed under the Apache 2.0 License.