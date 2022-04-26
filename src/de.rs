use std::io::{Cursor, Read};

use byteorder::{NetworkEndian, ReadBytesExt};
use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};
use serde::Deserialize;

use crate::error::{Error, Result};

pub struct Deserializer<'de, T: AsRef<[u8]>> {
    input: Cursor<&'de mut T>,
}

impl<'de, T: AsRef<[u8]>> Deserializer<'de, T> {
    pub fn from_reader(input: &'de mut T) -> Self {
        let cursor = Cursor::new(input);
        Deserializer { input: cursor }
    }
}

/// # Errors
pub fn from_reader<'a, T, U>(input: &'a mut T) -> Result<U>
where
    T: AsRef<[u8]>,
    U: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_reader(input);
    let u = U::deserialize(&mut deserializer)?;
    Ok(u)
}

impl<'de, 'a, T: AsRef<[u8]>> de::Deserializer<'de> for &'a mut Deserializer<'de, T> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u8().map_err(Error::io)?;
        visitor.visit_bool(value != 0)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_i8().map_err(Error::io)?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_i16::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_i32::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_i64::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u8().map_err(Error::io)?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u32::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u64::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_f32::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_f64::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let mut bytes = vec![0; length as usize];
        self.input.read_exact(&mut bytes).map_err(Error::io)?;
        let s = String::from_utf8(bytes).map_err(|_| Error::InvalidString)?;
        visitor.visit_string(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let mut bytes = Vec::with_capacity(length as usize);
        self.input.read_exact(&mut bytes).map_err(Error::io)?;
        visitor.visit_bytes(&bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.input.read_u8().map_err(Error::io)?;
        if value == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let value = visitor.visit_seq(LengthDefined::new(self, length))?;
        Ok(value)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_seq(LengthDefined::new(self, len as u16))?;
        Ok(value)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_seq(LengthDefined::new(self, len as u16))?;
        Ok(value)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let value = visitor.visit_map(LengthDefined::new(self, length))?;
        Ok(value)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_enum(Enum::new(self))?;
        Ok(value)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct LengthDefined<'a, 'de: 'a, T: AsRef<[u8]>> {
    de: &'a mut Deserializer<'de, T>,
    length: u16,
    index: u16,
}

impl<'a, 'de, T: AsRef<[u8]>> LengthDefined<'a, 'de, T> {
    fn new(de: &'a mut Deserializer<'de, T>, length: u16) -> Self {
        LengthDefined {
            de,
            length,
            index: 0,
        }
    }
}

impl<'a, 'de, T: AsRef<[u8]>> de::SeqAccess<'de> for LengthDefined<'a, 'de, T> {
    type Error = Error;

    fn next_element_seed<U>(&mut self, seed: U) -> Result<Option<U::Value>>
    where
        U: DeserializeSeed<'de>,
    {
        if self.index < self.length {
            self.index += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'de, 'a, T: AsRef<[u8]>> de::MapAccess<'de> for LengthDefined<'a, 'de, T> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.index < self.length {
            self.index += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, 'de: 'a, T: AsRef<[u8]>> {
    de: &'a mut Deserializer<'de, T>,
}

impl<'a, 'de, T: AsRef<[u8]>> Enum<'a, 'de, T> {
    fn new(de: &'a mut Deserializer<'de, T>) -> Self {
        Enum { de }
    }
}

impl<'a, 'de, T: AsRef<[u8]>> de::EnumAccess<'de> for Enum<'a, 'de, T> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let index = self.de.input.read_u8().map_err(Error::io)?;
        let value = seed.deserialize(index.into_deserializer())?;
        Ok((value, self))
    }
}

impl<'a, 'de, T: AsRef<[u8]>> de::VariantAccess<'de> for Enum<'a, 'de, T> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<U>(self, seed: U) -> Result<U::Value>
    where
        U: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}

#[cfg(test)]
mod test {
    use crate::from_reader;
    use serde::Deserialize;

    #[test]
    fn test_struct() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test {
            int: u32,
            seq: Vec<String>,
        }

        let mut j = vec![0, 0, 0, 1, 0, 2, 0, 1, 97, 0, 1, 98];
        let expected = Test {
            int: 1,
            seq: vec!["a".to_owned(), "b".to_owned()],
        };
        assert_eq!(expected, from_reader(&mut j).unwrap());
    }

    #[test]
    fn test_enum() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum E {
            Unit,
            Newtype(u32),
            Tuple(u32, u32),
            Struct { a: u32 },
        }

        let mut j = vec![0];
        let expected = E::Unit;
        assert_eq!(expected, from_reader(&mut j).unwrap());

        let mut j = vec![1, 0, 0, 0, 1];
        let expected = E::Newtype(1);
        assert_eq!(expected, from_reader(&mut j).unwrap());

        let mut j = vec![2, 0, 0, 0, 1, 0, 0, 0, 2];
        let expected = E::Tuple(1, 2);
        assert_eq!(expected, from_reader(&mut j).unwrap());

        let mut j = vec![3, 0, 0, 0, 1];
        let expected = E::Struct { a: 1 };
        assert_eq!(expected, from_reader(&mut j).unwrap());
    }
}
