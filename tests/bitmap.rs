use npsd::{Bitmap, Payload};

#[derive(Bitmap, PartialEq, Debug)]
struct Flags {
    opt0: bool,
    opt1: bool,
    opt2: bool,
    opt3: bool,
    opt4: bool,
    opt5: bool,
    opt6: bool,
    opt7: bool,
}

#[derive(Bitmap, PartialEq, Debug)]
struct Flags2 {
    opt0: bool,
    opt1: bool,
    opt2: bool,
    opt3: bool,
}

#[test]
fn test_bitmaps() {
    use pretty_hex::PrettyHex;

    let flags = Flags{
        opt0: false,
        opt1: true,
        opt2: false,
        opt3: true,
        opt4: false,
        opt5: true,
        opt6: false,
        opt7: true,
    };

    let serialized = flags.into_packet(&mut (), 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = Flags::from_packet(&mut (), &serialized).unwrap();

    assert_eq!(deserialized, flags);

    dbg!(deserialized);

    let flags = Flags2{
        opt0: false,
        opt1: true,
        opt2: false,
        opt3: true,
    };

    let serialized = flags.into_packet(&mut (), 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = Flags2::from_packet(&mut (), &serialized).unwrap();

    assert_eq!(deserialized, flags);
    
    dbg!(deserialized);
}