use std::fmt::Debug;
use npsd::{Payload, PayloadContext};
use pretty_hex::PrettyHex;

fn test_send_recv<'a, 'b, C, T, E>(ctx: &mut C, src: &'a T, dst: E)
where
    C: PayloadContext,
    T: Payload<'a, C>,
    E: Payload<'a, C> + PartialEq + Debug,
    'a: 'b,
{
    let serialized = src.into_packet(ctx, 1470).unwrap();

    println!("Encoded: {:?}", serialized.hex_dump());

    let deserialized = E::from_packet(ctx, serialized).unwrap();

    assert_eq!(deserialized, dst);
}

macro_rules! test_tuples {
    ($value:expr) => {
        {
            test_send_recv(&mut (), &$value, $value);
            test_send_recv(&mut (), &($value), ($value));
            test_send_recv(&mut (), &($value, $value), ($value, $value));
            test_send_recv(&mut (), &($value, $value, $value), ($value, $value, $value));
            test_send_recv(&mut (), &($value, $value, $value, $value), ($value, $value, $value, $value));
            test_send_recv(&mut (), &($value, $value, $value, $value, $value), ($value, $value, $value, $value, $value));
            test_send_recv(&mut (), &($value, $value, $value, $value, $value, $value), ($value, $value, $value, $value, $value, $value));
            test_send_recv(&mut (), &($value, $value, $value, $value, $value, $value, $value), ($value, $value, $value, $value, $value, $value, $value));
            test_send_recv(&mut (), &($value, $value, $value, $value, $value, $value, $value, $value), ($value, $value, $value, $value, $value, $value, $value, $value));
        }
    };
}

#[test]
fn test_tuple() {
    test_tuples!(());
    test_tuples!(true);
    test_tuples!(false);
    test_tuples!('H');
    test_tuples!('\u{00e9}');

    test_tuples!(-42i8);
    test_tuples!(42u8);
    test_tuples!(-2700i16);
    test_tuples!(42203u16);
    test_tuples!(-245500i32);
    test_tuples!(42235603u32);
    test_tuples!(-24558573920i64);
    test_tuples!(42235848723603u64);
    test_tuples!(-2455857399409220i128);
    test_tuples!(42235848723983488603u128);
    test_tuples!(-1337isize);
    test_tuples!(1337usize);

    test_tuples!(Some(-42i8));
    test_tuples!(Some(42u8));
    test_tuples!(Some(-2700i16));
    test_tuples!(Some(42203u16));
    test_tuples!(Some(-245500i32));
    test_tuples!(Some(42235603u32));
    test_tuples!(Some(-24558573920i64));
    test_tuples!(Some(42235848723603u64));
    test_tuples!(Some(-2455857399409220i128));
    test_tuples!(Some(42235848723983488603u128));
    test_tuples!(Some(-1337isize));
    test_tuples!(Some(1337usize));
    test_tuples!(None::<u8>);

    test_tuples!([-42i8; 64]);
    test_tuples!([42u8; 64]);
    test_tuples!([-2700i16; 64]);
    test_tuples!([42203u16; 64]);
    test_tuples!([-245500i32; 64]);
    test_tuples!([42235603u32; 64]);
    test_tuples!([-24558573920i64; 64]);
    test_tuples!([42235848723603u64; 64]);
    test_tuples!([-2455857399409220i128; 64]);
    test_tuples!([42235848723983488603u128; 64]);
    test_tuples!([-1337isize; 64]);
    test_tuples!([1337usize; 64]);

    test_tuples!(vec![-42i8, -42i8, -42i8, -42i8, -42i8]);
    test_tuples!(vec![42u8, 42u8, 42u8, 42u8, 42u8]);
    test_tuples!(vec![-2700i16, -2700i16, -2700i16, -2700i16]);
    test_tuples!(vec![42203u16, 42203u16, 42203u16, 42203u16]);
    test_tuples!(vec![-245500i32, -245500i32, -245500i32]);
    test_tuples!(vec![42235603u32, 42235603u32, 42235603u32]);
    test_tuples!(vec![-24558573920i64, -24558573920i64, -24558573920i64]);
    test_tuples!(vec![42235848723603u64, 42235848723603u64, 42235848723603u64]);
    test_tuples!(vec![-2455857399409220i128, -2455857399409220i128]);
    test_tuples!(vec![42235848723983488603u128, 42235848723983488603u128]);
    test_tuples!(vec![-1337isize, -1337isize, -1337isize]);
    test_tuples!(vec![1337usize, 1337usize, 1337usize]);

    test_tuples!(vec![Some(-42i8), None]);
    test_tuples!(vec![Some(42u8), None]);
    test_tuples!(vec![Some(-2700i16), None]);
    test_tuples!(vec![Some(42203u16), None]);
    test_tuples!(vec![Some(-245500i32), None]);
    test_tuples!(vec![Some(42235603u32), None]);
    test_tuples!(vec![Some(-24558573920i64), None]);
    test_tuples!(vec![Some(42235848723603u64), None]);
    test_tuples!(vec![Some(-2455857399409220i128), None]);
    test_tuples!(vec![Some(42235848723983488603u128), None]);
    test_tuples!(vec![Some(-1337isize), None]);
    test_tuples!(vec![Some(1337usize), None]);

    test_tuples!("Test String".to_string());
    test_tuples!("Test String".as_bytes().to_vec());

    test_send_recv(&mut (),&vec!["Test".to_string(), "Hello".to_string()], vec!["Test".to_string(), "Hello".to_string()]);

    let data = ['H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd', '!'];

    test_send_recv(&mut (), &data, data);
    test_send_recv(&mut (), &"Hello".to_string(), "Hello");
    test_send_recv(&mut (), &"Test String".to_string(), "Test String");
}