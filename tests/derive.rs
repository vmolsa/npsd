use std::{collections::{HashMap, HashSet}, net::SocketAddr};

use npsd::{Bitmap, Payload, Schema};
use uuid::Uuid;

#[derive(Schema, PartialEq, Debug)]
struct TupleStruct(u64, usize, String);

#[derive(Schema, PartialEq, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Schema, PartialEq, Debug)]
struct Point2D(f32, f64);

#[derive(Schema, PartialEq, Debug)]
struct Inches(u64);

#[derive(Schema, PartialEq, Debug)]
struct Instance;

#[derive(Schema, PartialEq, Debug)]
enum E {
    // Use three-step process:
    //   1. serialize_struct_variant
    //   2. serialize_field
    //   3. end
    Color { r: u8, g: u8, b: u8 },

    // Use three-step process:
    //   1. serialize_tuple_variant
    //   2. serialize_field
    //   3. end
    Point2D(f32, f64),

    // Use serialize_newtype_variant.
    Inches(u64),

    // Use serialize_unit_variant.
    Instance,
}

#[derive(Schema, PartialEq, Debug)]
struct User {
    name: String,
    email: String,
    age: usize,
    postal: u16,
}

#[derive(Schema, PartialEq, Debug)]
pub struct GenericStruct<T> {
    x: T,
}

#[derive(Schema, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[derive(Schema, PartialEq, Debug, Clone)]
struct Inner {
    a: (),
    b: usize,
    c: Vec<String>,
}

#[derive(Schema, PartialEq, Debug)]
struct Outer {
    inner: Vec<Inner>,
}

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

#[derive(Schema, Clone, PartialEq, Debug)]
struct MapSet {
    map1: HashMap<String, u32>,
    set1: HashSet<String>,
}

#[derive(Schema, PartialEq, Debug)]
struct TestStruct<'a> {
    field_name1: i32,
    field_name2: String,
    field_name3: User,
    field_name4: Color,
    field_name5: Point2D,
    field_name6: Inches,
    field_name7: Instance,
    field_name8: E,
    field_name9: u64,
    field_name10: u128,
    field_name11: SocketAddr,
    field_name12: (String, String, u32, u64),
    field_name13: &'a User,
    field_name14: &'a mut User,
    field_name15: Animal,
    field_name16: Outer,
    field_name17: &'a str,
    field_name18: GenericStruct<String>,
    field_name19: Flags,
    field_name20: MapSet,
}

#[test]
fn test_schema() {
    use pretty_hex::PrettyHex;

    let pointer = &User { 
        name: "Matti".to_string(), 
        email: "matti@teppo.com".to_string(), 
        age: 42,
        postal: 1337 
    };

    let pointer2 = &mut User {  
        name: "Matti".to_string(), 
        email: "matti@teppo.com".to_string(), 
        age: 42,
        postal: 1337 
    };

    let instance = TestStruct { 
        field_name1: 42, 
        field_name2: "Hello, world!".to_string(),
        field_name3: User { 
            name: "Matti".to_string(), 
            email: "matti@teppo.com".to_string(), 
            age: 42,
            postal: 1337 
        },
        field_name4: Color { r: 0, g: 0, b: 0 },
        field_name5: Point2D(0.1, 0.2),
        field_name6: Inches(24),
        field_name7: Instance,
        field_name8: E::Point2D(0.1, 0.2),
        field_name9: 0x1337u64,
        field_name10: Uuid::new_v4().as_u128(),
        field_name11: "127.0.0.1:12345".parse().unwrap(),
        field_name12: ("Matti".to_string(), "matti@teppo.com".to_string(), 0x42, 0x1337),
        field_name13: pointer,
        field_name14: pointer2,
        field_name15: Animal::Frog("Frog".to_string(), vec![12393818, -19383812, 11111, -1093838482]),
        field_name16: Outer { inner: vec![ 
            Inner { a: (), b: 1337, c: vec![ "matti".to_string(), "teppo".to_string ()] },
            Inner { a: (), b: 183838182, c: vec![ "Alice".to_string(), "Bob".to_string ()] },
        ]},
        field_name17: "Hello World!!!",
        field_name18: GenericStruct { x: "Teppo".to_string() },
        field_name19: Flags {
            opt0: false,
            opt1: true,
            opt2: false,
            opt3: true,
            opt4: true,
            opt5: false,
            opt6: false,
            opt7: true,
        },
        field_name20: MapSet {
            map1: HashMap::from([
                ("Matti".to_string(), 1337u32),
                ("Teppo".to_string(), 1337u32),
                ("Alice".to_string(), 1337u32),
                ("Bob".to_string(), 1337u32),
            ]),
            set1: HashSet::from([
                "Matti".to_string(),
                "Teppo".to_string(),
                "Alice".to_string(),
                "Bob".to_string(),
            ]),
        },
    };

    let serialized = instance.into_packet(&mut (), 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = TestStruct::from_packet(&mut (), &serialized).unwrap();

    assert_eq!(deserialized, instance);

    dbg!(deserialized);
}
