/// TODO: make this worth.
// use crate::parser::Section;
// use crate::{Error, Ini};
// use serde::de::{DeserializeSeed, Visitor};
// use std::iter::Peekable;
//
// #[derive(Debug, Clone)]
// enum Value<'a> {
//     SectionStart(Option<&'a str>),
//     Key(&'a str),
//     Value(&'a str),
//     Values(Vec<&'a str>),
// }
//
// struct ValueIterator<'a> {
//     inner: Peekable<Box<dyn Iterator<Item = Value<'a>> + 'a>>,
// }
//
// impl<'a> ValueIterator<'a> {
//     fn new(ini: &'a Ini<'a>) -> Self {
//         fn iter_from_section<'a>(section: &'a Section) -> impl Iterator<Item = Value<'a>> + 'a {
//             let section_start = Value::SectionStart(section.name());
//             let kv = section.entries_seq().into_iter().flat_map(|(key, values)| {
//                 if values.len() == 1 {
//                     [Value::Key(key), Value::Value(values[0])]
//                 } else {
//                     [Value::Key(key), Value::Values(values)]
//                 }
//             });
//
//             std::iter::once(section_start).chain(kv)
//         }
//
//         let root_section = iter_from_section(ini.root());
//         let sections = ini
//             .sections()
//             .flat_map(|(_, section)| iter_from_section(section));
//         let inner = root_section.chain(sections);
//
//         let inner: Box<dyn Iterator<Item = _>> = Box::new(inner);
//         Self {
//             inner: inner.peekable(),
//         }
//     }
//
//     pub fn peek(&mut self) -> Option<&Value<'a>> {
//         let r = self.inner.peek();
//         eprintln!("peek: {:?}", r);
//         r
//     }
// }
//
// impl<'a> Iterator for ValueIterator<'a> {
//     type Item = Value<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let r = self.inner.next();
//         eprintln!("next: {:?}", r);
//         r
//     }
// }
//
// pub struct Deserializer<'a> {
//     iter: ValueIterator<'a>,
// }
//
// impl<'a> Deserializer<'a> {
//     pub fn new(ini: &'a Ini<'a>) -> Self {
//         Self {
//             iter: ValueIterator::new(&ini),
//         }
//     }
//
//     fn peek_token(&mut self) -> Result<Option<Value<'a>>, Error> {
//         Ok(self.iter.peek().cloned())
//     }
//
//     fn next_token(&mut self) -> Result<Option<Value<'a>>, Error> {
//         Ok(self.iter.next())
//     }
//
//     fn next_value(&mut self) -> Result<&'a str, Error> {
//         match self.next_token()? {
//             Some(Value::Value(x)) => Ok(x),
//             Some(Value::Values(_)) => Err(Error::Message(
//                 "Expected single value, found multiple.".to_string(),
//             )),
//             Some(x) => Err(Error::Message(format!("Expected value, found {:?}", x))),
//             None => Err(Error::Message("Expected value, found EOF.".to_string())),
//         }
//     }
//
//     fn next_key(&mut self) -> Result<&'a str, Error> {
//         match self.next_token()? {
//             Some(Value::Key(x)) => Ok(x),
//             Some(Value::SectionStart(None)) => Ok(""),
//             Some(Value::SectionStart(Some(x))) => Ok(x),
//             Some(x) => Err(Error::Message(format!("Expected a key, found {:?}", x))),
//             None => Err(Error::Message("Expected a key, found EOF.".to_string())),
//         }
//     }
// }
//
// impl<'de: 'a, 'a> serde::de::Deserializer<'de> for &'a mut Deserializer<'de> {
//     type Error = Error;
//
//     fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
//         visitor.visit_str(self.next_value()?)
//     }
//
//     fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         match self.next_value()? {
//             "0" | "false" => visitor.visit_bool(false),
//             "1" | "true" => visitor.visit_bool(true),
//             x => Err(Error::Message(format!("Invalid bool value: {}", x))),
//         }
//     }
//
//     fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_i8(
//             self.next_value()?
//                 .parse()
//                 .map_err(|e| Error::Message(format!("Invalid i8 value: {}", e)))?,
//         )
//     }
//
//     fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_i16(
//             self.next_value()?
//                 .parse()
//                 .map_err(|e| Error::Message(format!("Invalid i16 value: {}", e)))?,
//         )
//     }
//
//     fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_i32(
//             self.next_value()?
//                 .parse()
//                 .map_err(|e| Error::Message(format!("Invalid i32 value: {}", e)))?,
//         )
//     }
//
//     fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_i64(
//             self.next_value()?
//                 .parse()
//                 .map_err(|e| Error::Message(format!("Invalid i64 value: {}", e)))?,
//         )
//     }
//
//     fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_u64(
//             self.next_value()?
//                 .parse()
//                 .map_err(|e| Error::Message(format!("Invalid u64 value: {}", e)))?,
//         )
//     }
//
//     fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_str(self.next_value()?)
//     }
//
//     fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_string(self.next_value()?.to_string())
//     }
//
//     fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_unit_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_newtype_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_tuple_struct<V>(
//         self,
//         name: &'static str,
//         len: usize,
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
//         visitor.visit_map(MapAccess::new(self))
//     }
//
//     fn deserialize_struct<V: Visitor<'de>>(
//         self,
//         name: &'static str,
//         fields: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value, Self::Error> {
//         eprintln!("deserialize_struct: {:?} fields: {:?}", name, fields);
//         self.deserialize_map(visitor)
//     }
//
//     fn deserialize_enum<V>(
//         self,
//         name: &'static str,
//         variants: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         todo!()
//     }
//
//     fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         match self.peek_token()? {
//             Some(Value::SectionStart(Some(name))) => visitor.visit_str(name),
//             _ => visitor.visit_str(self.next_key()?),
//         }
//     }
//
//     fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: Visitor<'de>,
//     {
//         visitor.visit_str(self.next_value()?)
//     }
// }
//
// struct MapAccess<'a, 'b: 'a> {
//     de: &'a mut Deserializer<'b>,
// }
//
// impl<'a, 'b: 'a> MapAccess<'a, 'b> {
//     fn new(de: &'a mut Deserializer<'b>) -> Self {
//         MapAccess { de }
//     }
// }
//
// impl<'de: 'a, 'a> serde::de::MapAccess<'de> for MapAccess<'a, 'de> {
//     type Error = Error;
//
//     fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
//     where
//         K: DeserializeSeed<'de>,
//     {
//         loop {
//             match self.de.peek_token()? {
//                 None => return Ok(None),
//                 Some(Value::SectionStart(x)) => {
//                     eprintln!("... s {x:?}");
//                     break;
//                 }
//                 Some(Value::Key(x)) => {
//                     eprintln!("... k {x:?}");
//                     break;
//                 }
//                 _ => {}
//             }
//         }
//         seed.deserialize(&mut *self.de).map(Some)
//     }
//
//     fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
//     where
//         V: DeserializeSeed<'de>,
//     {
//         match self.de.peek_token()? {
//             Some(Value::Value(v)) => seed.deserialize(&mut *self.de),
//             x => panic!(
//                 "{:?}",
//                 Error::Message(format!("Expected a value, got {x:?}"))
//             ),
//         }
//     }
// }
