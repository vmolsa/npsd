use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{collections::{HashMap, HashSet}, net::SocketAddr, time::Instant};

use npsd::{Bitmap, Payload, Schema};

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct TupleStruct(u64, usize, String);

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct Point2D(f32, f64);

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct Inches(u64);

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct Instance;

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct User {
    name: String,
    email: String,
    age: usize,
    postal: u16,
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
pub struct GenericStruct<T> {
    x: T,
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
enum Animal {
    Dog,
    Frog(String, Vec<isize>),
    Cat { age: usize, name: String },
    AntHive(Vec<String>),
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug, Clone)]
struct Inner {
    a: (),
    b: usize,
    c: Vec<String>,
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct Outer {
    inner: Vec<Inner>,
}

#[derive(Bitmap, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Schema, Serialize, Deserialize, Clone, PartialEq, Debug)]
struct MapSet {
    map1: HashMap<String, u32>,
    set1: HashSet<String>,
}

#[derive(Schema, Serialize, Deserialize, PartialEq, Debug)]
struct T<'a> {
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
    field_name13: User,
    field_name14: User,
    field_name15: Animal,
    field_name16: Outer,
    field_name17: &'a str,
    field_name18: GenericStruct<String>,
    field_name19: Flags,
    field_name20: MapSet,
}

#[test]
fn serde_test() {
    const TEST_REPEAT: usize = 10000;

    let t = T { 
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
        field_name13: User { 
            name: "Matti".to_string(), 
            email: "matti@teppo.com".to_string(), 
            age: 42,
            postal: 1337 
        },
        field_name14: User { 
            name: "Matti".to_string(), 
            email: "matti@teppo.com".to_string(), 
            age: 42,
            postal: 1337 
        },
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

    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let serialized = serde_json::to_string(&t).unwrap();  
        let deserialized: T = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, t);
    }

    let duration = start.elapsed();

    println!("serde_json: {:?}", duration);



    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let serialized = postcard::to_allocvec(&t).unwrap();
        let deserialized: T = postcard::from_bytes(&serialized).unwrap();

        assert_eq!(deserialized, t);
    }

    let duration = start.elapsed();

    println!("postcard: {:?}", duration);

    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let serialized = t.into_packet(&mut (), 1470).unwrap();    
        let deserialized = T::from_packet(&mut (), &serialized).unwrap();

        assert_eq!(deserialized, t);
    }

    let duration = start.elapsed();

    println!("npsd: {:?}", duration);

}

#[test]
fn serde_serialize_test() {
    const TEST_REPEAT: usize = 10000;

    let t = T { 
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
        field_name13: User { 
            name: "Matti".to_string(), 
            email: "matti@teppo.com".to_string(), 
            age: 42,
            postal: 1337 
        },
        field_name14: User { 
            name: "Matti".to_string(), 
            email: "matti@teppo.com".to_string(), 
            age: 42,
            postal: 1337 
        },
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

    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let _serialized = serde_json::to_string(&t).unwrap();  
    }

    let duration = start.elapsed();

    println!("serde_json: {:?}", duration);

    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let _serialized = postcard::to_allocvec(&t).unwrap();
    }

    let duration = start.elapsed();

    println!("postcard: {:?}", duration);

    let start = Instant::now();

    for _ in 0..TEST_REPEAT {
        let _serialized = t.into_packet(&mut (), 1470).unwrap();
    }

    let duration = start.elapsed();

    println!("npsd: {:?}", duration);

}
