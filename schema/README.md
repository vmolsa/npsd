
# `npsd` Derive Macros

This repository provides a set of custom derive macros to simplify the implementation of traits required by the `npsd` framework. These macros automate the generation of boilerplate code for various payload processing tasks, including serialization, deserialization, and payload conversion.

## Available Macros

### `#[derive(Info)]`
Generates an implementation of the `PayloadInfo` trait, which provides metadata about the payload type.

### `#[derive(Schema)]`
Generates implementations for payload processing traits such as `IntoPayload`, `FromPayload`, and `Payload` for public use.

### `#[derive(Bitmap)]`
Generates implementations for payload processing traits for bitmap structures with up to 8 fields.

### `#[derive(AsyncSchema)]`
Generates asynchronous implementations for payload processing traits such as `AsyncIntoPayload`, `AsyncFromPayload`, and `AsyncPayload` for public use.

### `#[derive(AsyncBitmap)]`
Generates asynchronous implementations for payload processing traits for bitmap structures with up to 8 fields.

### Example

Here's a simple example demonstrating how to use the `Schema` derive macro:

```rust
use npsd::Schema;

#[derive(Schema)]
struct MyPayload {
    id: u32,
    name: String,
}
```

This will automatically generate the necessary implementations for the `IntoPayload`, `FromPayload`, and `Payload` traits for the `MyPayload` struct.

For more detailed examples and usage, please refer to the documentation for each macro.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you encounter any problems or have suggestions for improvements.

## License

This project is licensed under the Apache 2.0 License.
