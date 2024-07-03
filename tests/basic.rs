use npsd::Info;

#[cfg(feature = "sync")]
use npsd::{Bitmap, Schema, Payload};

#[cfg(feature = "async")]
use npsd::{AsyncBitmap, AsyncSchema, AsyncPayload, Error};

#[cfg_attr(feature = "async", derive(AsyncSchema))]
#[cfg_attr(feature = "sync", derive(Schema))]
#[derive(Info, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[cfg(feature = "sync")]
#[test]
fn test_basic() {
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

    // Create an instance of `Animal`.
    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    // Serialize the `animal` instance into a packet.
    animal.into_packet(&mut ctx, &mut next).unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    // Deserialize the packet back into an `Animal` instance.
    let deserialized = Animal::from_packet(&mut ctx, &mut next).unwrap();

    // Ensure the deserialized instance matches the original.
    assert_eq!(deserialized, animal);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_tokio_basic() -> Result<(), Error> {
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

    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    animal.poll_into_packet(&mut ctx, &mut next).await?;

    let deserialized = Animal::poll_from_packet(&mut ctx, &mut next).await?;

    assert_eq!(deserialized, animal);

    Ok(())
}

#[cfg(feature = "async")]
#[async_std::test]
async fn test_async_std_basic() -> Result<(), Error> {
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

    let animal = Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]);

    animal.poll_into_packet(&mut ctx, &mut next).await?;

    let deserialized = Animal::poll_from_packet(&mut ctx, &mut next).await?;

    assert_eq!(deserialized, animal);

    Ok(())
}

#[cfg_attr(feature = "async", derive(AsyncBitmap))]
#[cfg_attr(feature = "sync", derive(Bitmap))]
#[derive(Info, PartialEq, Debug)]
struct Flags {
    a: bool,
    b: bool,
    c: bool,
}

#[cfg(feature = "sync")]
#[test]
fn test_bitmap() {
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

    let flags = Flags { a: true, b: false, c: true };

    flags.into_packet(&mut ctx, &mut next).unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    let deserialized = Flags::from_packet(&mut ctx, &mut next).unwrap();

    assert_eq!(deserialized, flags);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_tokio_bitmap() {
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

    let flags = Flags { a: true, b: false, c: true };

    flags.poll_into_packet(&mut ctx, &mut next).await.unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    let deserialized = Flags::poll_from_packet(&mut ctx, &mut next).await.unwrap();

    assert_eq!(deserialized, flags);
}

#[cfg(feature = "async")]
#[async_std::test]
async fn test_async_std_bitmap() {
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

    let flags = Flags { a: true, b: false, c: true };

    flags.poll_into_packet(&mut ctx, &mut next).await.unwrap();

    // Create copy of serialized data if needed
    let _serialized = next.serialized();

    let deserialized = Flags::poll_from_packet(&mut ctx, &mut next).await.unwrap();

    assert_eq!(deserialized, flags);
}
