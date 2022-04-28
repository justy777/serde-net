use serde::{de, ser, Deserialize, Serialize};
use serde_net::{from_reader, to_bytes};
use std::collections::BTreeMap;
use std::fmt::Debug;

fn test_roundtrip_ok<T>(value: T, output: Vec<u8>)
where
    T: PartialEq + Debug + ser::Serialize + de::DeserializeOwned,
{
    let mut bytes = to_bytes(&value).unwrap();
    assert_eq!(bytes, output);

    let v: T = from_reader(&mut bytes).unwrap();
    assert_eq!(v, value);
}

#[test]
fn test_roundtrip_unit() {
    test_roundtrip_ok((), vec![]);
}

#[test]
fn test_roundtrip_bool() {
    test_roundtrip_ok(true, vec![1]);
    test_roundtrip_ok(false, vec![0]);
}

#[test]
fn test_roundtrip_i8() {
    test_roundtrip_ok(-3i8, vec![253]);
    test_roundtrip_ok(0i8, vec![0]);
    test_roundtrip_ok(3i8, vec![3]);
    test_roundtrip_ok(i8::MIN, vec![128]);
    test_roundtrip_ok(i8::MAX, vec![127]);
}

#[test]
fn test_roundtrip_i16() {
    test_roundtrip_ok(-3456i16, vec![242, 128]);
    test_roundtrip_ok(0i16, vec![0, 0]);
    test_roundtrip_ok(3456i16, vec![13, 128]);
    test_roundtrip_ok(i16::MIN, vec![128, 0]);
    test_roundtrip_ok(i16::MAX, vec![127, 255]);
}

#[test]
fn test_roundtrip_i32() {
    test_roundtrip_ok(-7359i32, vec![255, 255, 227, 65]);
    test_roundtrip_ok(0i32, vec![0, 0, 0, 0]);
    test_roundtrip_ok(7359i32, vec![0, 0, 28, 191]);
    test_roundtrip_ok(i32::MIN, vec![128, 0, 0, 0]);
    test_roundtrip_ok(i32::MAX, vec![127, 255, 255, 255]);
}

#[test]
fn test_roundtrip_i64() {
    test_roundtrip_ok(-94533i64, vec![255, 255, 255, 255, 255, 254, 142, 187]);
    test_roundtrip_ok(0i64, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    test_roundtrip_ok(94533i64, vec![0, 0, 0, 0, 0, 1, 113, 69]);
    test_roundtrip_ok(i64::MIN, vec![128, 0, 0, 0, 0, 0, 0, 0]);
    test_roundtrip_ok(i64::MAX, vec![127, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_roundtrip_u8() {
    test_roundtrip_ok(3u8, vec![3]);
    test_roundtrip_ok(u8::MIN, vec![0]);
    test_roundtrip_ok(u8::MAX, vec![255]);
}

#[test]
fn test_roundtrip_u16() {
    test_roundtrip_ok(5456u16, vec![21, 80]);
    test_roundtrip_ok(u16::MIN, vec![0, 0]);
    test_roundtrip_ok(u16::MAX, vec![255, 255]);
}

#[test]
fn test_roundtrip_u32() {
    test_roundtrip_ok(7359u32, vec![0, 0, 28, 191]);
    test_roundtrip_ok(u32::MIN, vec![0, 0, 0, 0]);
    test_roundtrip_ok(u32::MAX, vec![255, 255, 255, 255]);
}

#[test]
fn test_roundtrip_u64() {
    test_roundtrip_ok(9764533u64, vec![0, 0, 0, 0, 0, 148, 254, 181]);
    test_roundtrip_ok(u64::MIN, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    test_roundtrip_ok(u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_roundtrip_f32() {
    test_roundtrip_ok(-1.333f32, vec![191, 170, 159, 190]);
    test_roundtrip_ok(0f32, vec![0, 0, 0, 0]);
    test_roundtrip_ok(1.333f32, vec![63, 170, 159, 190]);
    test_roundtrip_ok(f32::MIN, vec![255, 127, 255, 255]);
    test_roundtrip_ok(f32::MAX, vec![127, 127, 255, 255]);
}

#[test]
fn test_roundtrip_f64() {
    test_roundtrip_ok(-78.42568f64, vec![192, 83, 155, 62, 87, 83, 163, 236]);
    test_roundtrip_ok(0f64, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    test_roundtrip_ok(-78.42568f64, vec![192, 83, 155, 62, 87, 83, 163, 236]);
    test_roundtrip_ok(f64::MIN, vec![255, 239, 255, 255, 255, 255, 255, 255]);
    test_roundtrip_ok(f64::MAX, vec![127, 239, 255, 255, 255, 255, 255, 255]);
}

#[test]
fn test_roundtrip_char() {
    test_roundtrip_ok('a', vec![0, 0, 0, 97]);
    test_roundtrip_ok('💯', vec![0, 1, 244, 175]);
    test_roundtrip_ok('あ', vec![0, 0, 48, 66]);
    test_roundtrip_ok(char::MAX, vec![0, 16, 255, 255]);
}

#[test]
fn test_roundtrip_string() {
    test_roundtrip_ok(String::from(""), vec![0, 0]);
    test_roundtrip_ok(
        String::from("💯あ"),
        vec![0, 7, 240, 159, 146, 175, 227, 129, 130],
    );
    test_roundtrip_ok(
        String::from("Hello world"),
        vec![0, 11, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100],
    );
}

#[test]
fn test_roundtrip_option() {
    let value: Option<u32> = None;
    test_roundtrip_ok(value, vec![0]);
    let value: Option<u32> = Some(77);
    test_roundtrip_ok(value, vec![1, 0, 0, 0, 77]);
}

#[test]
fn test_roundtrip_seq() {
    let value: [u16; 3] = [77, 54, 13];
    test_roundtrip_ok(value, vec![0, 77, 0, 54, 0, 13]);
}

#[test]
fn test_roundtrip_tuple() {
    let value: (u8, bool, char) = (63, true, 'g');
    test_roundtrip_ok(value, vec![63, 1, 0, 0, 0, 103]);
}

#[test]
fn test_roundtrip_map() {
    let mut map: BTreeMap<String, u8> = BTreeMap::new();
    map.insert(String::from("Monkey"), 7);
    map.insert(String::from("Dog"), 3);

    test_roundtrip_ok(
        map,
        vec![
            0, 2, 0, 3, 68, 111, 103, 3, 0, 6, 77, 111, 110, 107, 101, 121, 7,
        ],
    );
}

#[test]
fn test_roundtrip_unit_struct() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Unit;

    test_roundtrip_ok(Unit {}, vec![]);
}

#[test]
fn test_roundtrip_newtype_struct() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct NewType(char);

    test_roundtrip_ok(NewType('a'), vec![0, 0, 0, 97]);
}

#[test]
fn test_roundtrip_tuple_struct() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple(char, u8, bool);

    test_roundtrip_ok(Tuple('$', 125, false), vec![0, 0, 0, 36, 125, 0]);
}

#[test]
fn test_roundtrip_struct() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Test {
        int: u32,
        seq: Vec<String>,
    }

    let value = Test {
        int: 1,
        seq: vec!["a".to_owned(), "b".to_owned()],
    };
    test_roundtrip_ok(value, vec![0, 0, 0, 1, 0, 2, 0, 1, 97, 0, 1, 98]);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum E {
    Unit,
    Newtype(u32),
    Tuple(u8, u8),
    Struct { a: u32 },
}

#[test]
fn test_roundtrip_unit_variant() {
    test_roundtrip_ok(E::Unit, vec![0]);
}

#[test]
fn test_roundtrip_newtype_variant() {
    test_roundtrip_ok(E::Newtype(1), vec![1, 0, 0, 0, 1]);
}

#[test]
fn test_roundtrip_tuple_variant() {
    test_roundtrip_ok(E::Tuple(1, 2), vec![2, 1, 2]);
}

#[test]
fn test_roundtrip_struct_variant() {
    test_roundtrip_ok(E::Struct { a: 1 }, vec![3, 0, 0, 0, 1]);
}
