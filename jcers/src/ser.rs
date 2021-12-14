use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};

/// Jce Writer
pub struct JceMut(BytesMut);

macro_rules! impl_put {
    ($fn_name: ident, $input_type: ty) => {
        /// put value with the tag
        pub fn $fn_name(&mut self, value: $input_type, tag: u8) {
            value.jce_put(self, tag)
        }
    };
}

impl JceMut {
    pub fn new() -> Self {
        JceMut(BytesMut::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        JceMut(BytesMut::with_capacity(capacity))
    }

    pub fn put_head(&mut self, t: u8, tag: u8) {
        if tag < 15 {
            self.0.put_u8(t | (tag << 4));
        } else {
            self.0.put_u8(t | 0xf0);
            self.0.put_u8(tag);
        }
    }

    pub fn freeze(self) -> Bytes {
        self.0.freeze()
    }

    impl_put!(put_u8, u8);
    impl_put!(put_bool, bool);
    impl_put!(put_i16, i16);
    impl_put!(put_i32, i32);
    impl_put!(put_i64, i64);
    impl_put!(put_f32, f32);
    impl_put!(put_f64, f64);
    impl_put!(put_string, String);
    impl_put!(put_bytes, Bytes);

    /// put value with the tag
    pub fn put_map<K: JcePut, V: JcePut>(&mut self, value: HashMap<K, V>, tag: u8) {
        value.jce_put(self, tag)
    }

    /// put value with the tag
    pub fn put_list<V: JcePut>(&mut self, value: Vec<V>, tag: u8) {
        value.jce_put(self, tag)
    }
}

/// Serialize JceValue into Bytes
pub trait JcePut: Sized {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(10, tag);
        self.jce_put_raw(jce_mut);
        jce_mut.put_head(11, tag);
    }
    fn jce_put_raw(self, _: &mut JceMut) {}
    fn freeze(self) -> Bytes {
        let mut jce_mut = JceMut::new();
        self.jce_put_raw(&mut jce_mut);
        jce_mut.freeze()
    }
}

macro_rules! impl_freeze {
    () => {
        fn freeze(self) -> Bytes {
            let mut jce_mut = JceMut::new();
            self.jce_put(&mut jce_mut, 0);
            jce_mut.freeze()
        }
    };
}

impl JcePut for u8 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        if self == 0 {
            jce_mut.put_head(12, tag);
        } else {
            jce_mut.put_head(0, tag);
            jce_mut.0.put_u8(self);
        }
    }
    impl_freeze!();
}

impl JcePut for bool {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        {
            if self {
                1u8
            } else {
                0u8
            }
        }
        .jce_put(jce_mut, tag);
    }
}

impl JcePut for i16 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        if self >= i8::MIN as i16 && self <= i8::MAX as i16 {
            self.to_le_bytes()[0].jce_put(jce_mut, tag);
        } else {
            jce_mut.put_head(1, tag);
            jce_mut.0.put_i16(self);
        }
    }
}

impl JcePut for i32 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        if self >= i16::MIN as i32 && self <= i16::MAX as i32 {
            (self as i16).jce_put(jce_mut, tag);
        } else {
            jce_mut.put_head(2, tag);
            jce_mut.0.put_i32(self);
        }
    }
}

impl JcePut for i64 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        if self >= i32::MIN as i64 && self <= i32::MAX as i64 {
            (self as i32).jce_put(jce_mut, tag);
        } else {
            jce_mut.put_head(3, tag);
            jce_mut.0.put_i64(self);
        }
    }
}

impl JcePut for f32 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(4, tag);
        jce_mut.0.put_f32(self);
    }
}

impl JcePut for f64 {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(5, tag);
        jce_mut.0.put_f64(self);
    }
}

impl JcePut for String {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        let len = self.len();
        if len < 256 {
            jce_mut.put_head(6, tag);
            jce_mut.0.put_u8(len as u8);
            jce_mut.0.extend(self.as_bytes());
        } else {
            jce_mut.put_head(7, tag);
            jce_mut.0.put_i32(len as i32);
            jce_mut.0.extend(self.as_bytes());
        }
    }
}

impl<K, V> JcePut for HashMap<K, V>
where
    K: JcePut,
    V: JcePut,
{
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(8, tag);
        (self.len() as i32).jce_put(jce_mut, 0);
        for (k, v) in self {
            k.jce_put(jce_mut, 0);
            v.jce_put(jce_mut, 1);
        }
    }
}

impl<V> JcePut for Vec<V>
where
    V: JcePut,
{
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(9, tag);
        (self.len() as i32).jce_put(jce_mut, 0);
        for v in self {
            v.jce_put(jce_mut, 0);
        }
    }
}

impl JcePut for Bytes {
    fn jce_put(self, jce_mut: &mut JceMut, tag: u8) {
        jce_mut.put_head(13, tag);
        jce_mut.put_head(0, 0);
        (self.len() as i32).jce_put(jce_mut, 0);
        jce_mut.0.extend(self);
    }
}
