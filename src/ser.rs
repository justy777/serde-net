use std::io::Write;

use byteorder::{NetworkEndian, WriteBytesExt};
use serde::{ser, Serialize};

use crate::error::{Error, Result};

pub struct Serializer {
    output: Vec<u8>,
}

/// # Errors
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.write_u8(u8::from(v)).map_err(Error::io)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.write_i8(v).map_err(Error::io)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output.write_i16::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output.write_i32::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.write_i64::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.write_u8(v).map_err(Error::io)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.write_u16::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.write_u32::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.write_u64::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output.write_f32::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.write_f64::<NetworkEndian>(v).map_err(Error::io)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output
            .write_u16::<NetworkEndian>(v.len() as u16)
            .map_err(Error::io)?;
        self.output.write_all(v).map_err(Error::io)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_bool(false)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.serialize_bool(true)?;
        value.serialize(&mut *self)
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.output.write_u8(variant_index as u8).map_err(Error::io)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut *self)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.output
            .write_u8(variant_index as u8)
            .map_err(Error::io)?;
        value.serialize(&mut *self)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(len) => {
                self.output
                    .write_u16::<NetworkEndian>(len as u16)
                    .map_err(Error::io)?;
                Ok(self)
            }
            None => Err(Error::LengthNotKnown),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output
            .write_u8(variant_index as u8)
            .map_err(Error::io)?;
        Ok(self)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        match len {
            Some(len) => {
                self.output
                    .write_u16::<NetworkEndian>(len as u16)
                    .map_err(Error::io)?;
                Ok(self)
            }
            None => Err(Error::LengthNotKnown),
        }
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output
            .write_u8(variant_index as u8)
            .map_err(Error::io)?;
        Ok(self)
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::to_bytes;
    use serde::Serialize;

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
            seq: Vec<&'static str>,
        }

        let test = Test {
            int: 1,
            seq: vec!["a", "b"],
        };
        let expected = vec![0, 0, 0, 1, 0, 2, 0, 1, 97, 0, 1, 98];
        assert_eq!(to_bytes(&test).unwrap(), expected);
    }

    #[test]
    fn test_enum() {
        #[derive(Serialize)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let u = E::Unit;
        let expected = vec![0];
        assert_eq!(to_bytes(&u).unwrap(), expected);

        let n = E::Newtype(1);
        let expected = vec![1, 0, 0, 0, 1];
        assert_eq!(to_bytes(&n).unwrap(), expected);

        let t = E::Tuple(1, 2);
        let expected = vec![2, 0, 0, 0, 1, 0, 0, 0, 2];
        assert_eq!(to_bytes(&t).unwrap(), expected);

        let s = E::Struct { a: 1 };
        let expected = vec![3, 0, 0, 0, 1];
        assert_eq!(to_bytes(&s).unwrap(), expected);
    }
}
