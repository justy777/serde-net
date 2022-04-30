use std::io::{Cursor, Read};

use byteorder::{NetworkEndian, ReadBytesExt};
use serde::de::{self, DeserializeOwned, DeserializeSeed, IntoDeserializer, Visitor};

use crate::error::{Error, Result};

pub struct Deserializer<R: Read> {
    input: R,
}

impl<R: Read> Deserializer<R> {
    pub fn from_reader(input: R) -> Self {
        Deserializer { input }
    }
}

impl<T: AsRef<[u8]>> Deserializer<Cursor<T>> {
    pub fn from_bytes(input: T) -> Self {
        let cursor = Cursor::new(input);
        Deserializer { input: cursor }
    }
}

/// # Errors
pub fn from_reader<R, D>(input: R) -> Result<D>
where
    R: Read,
    D: DeserializeOwned,
{
    let mut deserialized = Deserializer::from_reader(input);
    D::deserialize(&mut deserialized)
}

/// # Errors
pub fn from_bytes<T, D>(input: &mut T) -> Result<D>
where
    T: AsRef<[u8]>,
    D: DeserializeOwned,
{
    let mut deserializer = Deserializer::from_bytes(input);
    D::deserialize(&mut deserializer)
}

impl<'de, R: Read> de::Deserializer<'de> for &mut Deserializer<R> {
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
        let value = self.input.read_u32::<NetworkEndian>().map_err(Error::io)?;
        let c = char::from_u32(value).ok_or(Error::InvalidChar)?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let mut bytes = vec![0; length as usize];
        self.input.read_exact(&mut bytes).map_err(Error::io)?;
        let s = String::from_utf8(bytes).map_err(|_| Error::InvalidString)?;
        visitor.visit_string(s)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        let mut bytes = vec![0; length as usize];
        self.input.read_exact(&mut bytes).map_err(Error::io)?;
        visitor.visit_bytes(&bytes)
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

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
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
        visitor.visit_seq(LengthDefined::new(self, length))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(LengthDefined::new(self, len as u16))
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
        visitor.visit_seq(LengthDefined::new(self, len as u16))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let length = self.input.read_u16::<NetworkEndian>().map_err(Error::io)?;
        visitor.visit_map(LengthDefined::new(self, length))
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
        visitor.visit_enum(Enum::new(self))
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

struct LengthDefined<'a, R: Read> {
    de: &'a mut Deserializer<R>,
    length: u16,
    index: u16,
}

impl<'de, 'a, R: Read> LengthDefined<'a, R> {
    fn new(de: &'a mut Deserializer<R>, length: u16) -> Self {
        LengthDefined {
            de,
            length,
            index: 0,
        }
    }

    fn next_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.index < self.length {
            self.index += 1;
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'de, 'a, R: Read> de::SeqAccess<'de> for LengthDefined<'a, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        self.next_seed::<T>(seed)
    }
}

impl<'de, 'a, R: Read> de::MapAccess<'de> for LengthDefined<'a, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.next_seed::<K>(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

struct Enum<'a, R: Read> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read> Enum<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a, R: Read> de::EnumAccess<'de> for Enum<'a, R> {
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

impl<'de, 'a, R: Read> de::VariantAccess<'de> for Enum<'a, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
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
