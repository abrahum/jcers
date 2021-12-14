use std::collections::HashMap;
use std::hash::Hash;

use super::{JceError, JceHead, JceResult, JceType, JceValue};
use bytes::{Buf, Bytes};

/// Jce Reader
pub struct Jce<'a, B>
where
    B: Buf + ?Sized,
{
    inner: &'a mut B,
    pub head: JceHead,
    readed: bool,
}

/// Deserialize Jce Value
pub trait JceGet: Sized {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self>;
    fn empty() -> JceResult<Self>;
    fn get_from_buf<B: Buf + ?Sized>(buf: &mut B) -> JceResult<Self> {
        let mut jce = Jce::new(buf);
        Self::jce_get(&mut jce)
    }
    fn get_by_tag<B: Buf + ?Sized>(jce: &mut Jce<B>, tag: u8) -> JceResult<Self> {
        jce.get_by_tag(tag)
    }
}

impl<'a, B> Jce<'a, B>
where
    B: Buf + ?Sized,
{
    pub fn new(inner: &'a mut B) -> Self {
        let mut jce = Jce {
            inner,
            head: JceHead::default(),
            readed: false,
        };
        jce.read_head();
        jce
    }

    pub fn sub_jce<'b>(&'b mut self) -> Jce<'b, B> {
        Jce::new(self.inner)
    }

    pub fn has_remaining(&self) -> bool {
        self.inner.has_remaining()
    }

    pub fn read_head(&mut self) -> JceHead {
        let byte = self.inner.get_u8();
        let ty = JceType::from(byte & 0xF);
        let mut tag = (byte & 0xF0) >> 4;
        if tag == 15 {
            let next_byte = self.inner.get_u8();
            tag = next_byte & 0xFF;
        }
        let head = JceHead { ty, tag };
        self.head = head;
        self.readed = false;
        head
    }

    pub fn pass_a_tag(&mut self) -> JceResult<()> {
        if self.head.ty != JceType::StructEnd {
            JceValue::jce_get(self).map(|_| ())
        } else {
            Ok(())
        }
    }

    pub fn go_to_tag(&mut self, tag: u8) -> JceResult<()> {
        if !self.readed {
            self.pass_a_tag()?;
        }
        while self.read_head().tag != tag {
            self.pass_a_tag()?;
            if !self.inner.has_remaining() {
                return Err(JceError::TagNotFound(tag));
            }
        }
        Ok(())
    }

    pub fn get_by_tag<T>(&mut self, tag: u8) -> JceResult<T>
    where
        T: JceGet,
    {
        if self.head.tag != tag {
            self.go_to_tag(tag)?;
        }
        self.readed = true;
        T::jce_get(self)
    }

    pub fn end_struct(&mut self) -> JceResult<()> {
        while self.read_head().ty != JceType::StructEnd {
            self.pass_a_tag()?;
        }
        Ok(())
    }
}

impl JceGet for bool {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::Bool | JceType::Byte => Ok(jce.inner.get_u8() != 0),
            _ => Err(JceError::ReadTypeError(JceType::Bool, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        panic!() //?
    }
}

impl JceGet for u8 {
    // ty: 0 or 12
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::Empty => Self::empty(),
            JceType::Byte => Ok(jce.inner.get_u8()),
            _ => Err(JceError::ReadTypeError(JceType::Byte, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0)
    }
}

impl JceGet for i16 {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::Byte => u8::jce_get(jce).map(|i| i as i16),
            JceType::I16 => Ok(jce.inner.get_i16()),
            JceType::Empty => Self::empty(),
            _ => Err(JceError::ReadTypeError(JceType::I16, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0)
    }
}

impl JceGet for i32 {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::Byte => u8::jce_get(jce).map(|i| i as i32),
            JceType::I16 => i16::jce_get(jce).map(|i| i as i32),
            JceType::I32 => Ok(jce.inner.get_i32()),
            JceType::Empty => Self::empty(),
            _ => Err(JceError::ReadTypeError(JceType::I32, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0)
    }
}

impl JceGet for i64 {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::Byte => u8::jce_get(jce).map(|i| i as i64),
            JceType::I16 => i16::jce_get(jce).map(|i| i as i64),
            JceType::I32 => i32::jce_get(jce).map(|i| i as i64),
            JceType::I64 => Ok(jce.inner.get_i64()),
            JceType::Empty => Self::empty(),
            _ => Err(JceError::ReadTypeError(JceType::I64, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0)
    }
}

impl JceGet for f32 {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::F32 => Ok(jce.inner.get_f32()),
            JceType::Empty => Self::empty(),
            _ => Err(JceError::ReadTypeError(JceType::F32, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0.0)
    }
}

impl JceGet for f64 {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        match jce.head.ty {
            JceType::F64 => Ok(jce.inner.get_f64()),
            JceType::Empty => Self::empty(),
            _ => Err(JceError::ReadTypeError(JceType::F64, jce.head.ty)),
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(0.0)
    }
}

impl JceGet for String {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        let len = match jce.head.ty {
            JceType::ShortString => jce.inner.get_u8() as usize,
            JceType::LongString => jce.inner.get_i32() as usize,
            _ => {
                return Err(JceError::ReadLenError(jce.head.ty));
            }
        };
        if len == 0 {
            Self::empty()
        } else {
            let data = jce.inner.copy_to_bytes(len);
            String::from_utf8(data.to_vec()).map_err(|_| JceError::Utf8Error)
        }
    }

    fn empty() -> JceResult<Self> {
        Ok(String::default())
    }
}

impl<K, V> JceGet for HashMap<K, V>
where
    K: JceGet + Eq + Hash,
    V: JceGet,
{
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        if jce.head.ty != JceType::Map {
            return Err(JceError::ReadTypeError(JceType::Map, jce.head.ty));
        }
        let mut jce = jce.sub_jce();
        let len = jce.get_by_tag::<i32>(0)? as usize;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            let mut jce = jce.sub_jce();
            let key = jce.get_by_tag(0)?;
            let value = jce.get_by_tag(1)?;
            map.insert(key, value);
        }
        Ok(map)
    }

    fn empty() -> JceResult<Self> {
        Ok(HashMap::new())
    }
}

impl<V> JceGet for Vec<V>
where
    V: JceGet,
{
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        if jce.head.ty != JceType::List {
            return Err(JceError::ReadTypeError(JceType::List, jce.head.ty));
        }
        let mut jce = jce.sub_jce();
        let len = jce.get_by_tag::<i32>(0)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            let mut jce = jce.sub_jce();
            let value = jce.get_by_tag(0)?;
            vec.push(value);
        }
        Ok(vec)
    }

    fn empty() -> JceResult<Self> {
        Ok(Vec::new())
    }
}

impl JceGet for Bytes {
    fn jce_get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
        jce.inner.get_u8();
        let mut jce = jce.sub_jce();
        let len = jce.get_by_tag::<i32>(0)? as usize;
        Ok(jce.inner.copy_to_bytes(len))
    }

    fn empty() -> JceResult<Self> {
        Ok(Bytes::default())
    }
}

// impl JceGet for Vec<u8> { conflicting implementations
//     fn get<B: Buf + ?Sized>(jce: &mut Jce<B>) -> JceResult<Self> {
//         jce.inner.get_u8();
//         let len = jce.get_by_tag::<i32>(0)? as usize;
//         let mut vec = Vec::with_capacity(len);
//         jce.inner.copy_to_slice(&mut vec)
//         Ok(vec)
//     }

//     fn empty() -> JceResult<Self> {
//         Ok(vec![])
//     }
// }
