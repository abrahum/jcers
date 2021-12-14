use super::JceType;
use std::collections::{BTreeMap, HashMap};

use bytes::Bytes;

/// Value type covers all types in jce
#[derive(Debug, Clone, PartialEq)]
pub enum JceValue {
    Bool(bool),
    Byte(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Map(HashMap<JceMapKey, JceValue>),
    List(Vec<JceValue>),
    Struct(JceStruct),
    Empty,
    Bytes(Bytes),
}

impl super::JceGet for JceValue {
    fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut crate::de::Jce<B>) -> crate::JceResult<Self> {
        match jce.head.ty {
            JceType::Bool => Ok(Self::Bool(bool::jce_get(jce)?)),
            JceType::Byte => Ok(Self::Byte(u8::jce_get(jce)?)),
            JceType::I16 => Ok(Self::I16(i16::jce_get(jce)?)),
            JceType::I32 => Ok(Self::I32(i32::jce_get(jce)?)),
            JceType::I64 => Ok(Self::I64(i64::jce_get(jce)?)),
            JceType::F32 => Ok(Self::F32(f32::jce_get(jce)?)),
            JceType::F64 => Ok(Self::F64(f64::jce_get(jce)?)),
            JceType::ShortString | JceType::LongString => Ok(Self::String(String::jce_get(jce)?)),
            JceType::Map => Ok(Self::Map(HashMap::<JceMapKey, JceValue>::jce_get(jce)?)),
            JceType::List => Ok(Self::List(Vec::<JceValue>::jce_get(jce)?)),
            JceType::Struct => Ok(Self::Struct(JceStruct::jce_get(jce)?)),
            JceType::Empty => Ok(Self::Empty),
            JceType::Bytes => Ok(Self::Bytes(Bytes::jce_get(jce)?)),
            _ => panic!("head: {:?}", jce.head), //todo
        }
    }

    fn empty() -> crate::JceResult<Self> {
        Ok(JceValue::Empty)
    }
}

/// Key type for jce map
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JceMapKey {
    String(String),
    Byte(u8),
    I16(i16),
    I32(i32),
    I64(i64),
}

impl super::JceGet for JceMapKey {
    fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut crate::de::Jce<B>) -> crate::JceResult<Self> {
        match jce.head.ty {
            JceType::Byte => Ok(Self::Byte(u8::jce_get(jce)?)),
            JceType::I16 => Ok(Self::I16(i16::jce_get(jce)?)),
            JceType::I32 => Ok(Self::I32(i32::jce_get(jce)?)),
            JceType::I64 => Ok(Self::I64(i64::jce_get(jce)?)),
            JceType::ShortString | JceType::LongString => Ok(Self::String(String::jce_get(jce)?)),
            _ => panic!(), //todo
        }
    }

    fn empty() -> crate::JceResult<Self> {
        panic!() // todo
    }
}

/// Struct type for jce
pub type JceStruct = BTreeMap<u8, JceValue>;

impl super::JceGet for JceStruct {
    fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut crate::de::Jce<B>) -> crate::JceResult<Self> {
        let mut map = BTreeMap::new();
        while jce.has_remaining() {
            jce.read_head();
            let tag = jce.head.tag;
            if jce.head.ty == JceType::StructEnd {
                break;
            }
            let value = JceValue::jce_get(jce)?;
            map.insert(tag, value);
        }
        Ok(map)
    }

    fn empty() -> crate::JceResult<Self> {
        Ok(BTreeMap::default())
    }
}
