use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TestStruct {
    pub a: u8,              // 0
    pub b: i32,             // 1
    pub c: f32,             // 2
    pub d: String,          // 3
    pub e: HashMap<u8, u8>, // 6
}

impl JceGet for TestStruct {
    fn jce_get<B: bytes::Buf + ?Sized>(jce: &mut de::Jce<B>) -> JceResult<Self> {
        fn get<B: bytes::Buf + ?Sized>(jce: &mut de::Jce<B>) -> JceResult<TestStruct> {
            Ok(TestStruct {
                a: jce.get_by_tag::<u8>(0)?,
                b: jce.get_by_tag::<i32>(1)?,
                c: jce.get_by_tag::<f32>(2)?,
                d: jce.get_by_tag::<String>(3)?,
                e: jce.get_by_tag::<HashMap<u8, u8>>(6)?,
            })
        }

        let sub = jce.head.ty == JceType::Struct;
        Ok(if sub {
            let mut sub_jce = jce.sub_jce();
            let r = get(&mut sub_jce)?;
            jce.end_struct()?;
            r
        } else {
            get(jce)?
        })
    }

    fn empty() -> JceResult<Self> {
        Ok(Self::default())
    }
}

impl JcePut for TestStruct {
    fn jce_put(self, jce_mut: &mut ser::JceMut, tag: u8) {
        jce_mut.put_head(10, tag);
        self.a.jce_put(jce_mut, 0);
        self.b.jce_put(jce_mut, 1);
        self.c.jce_put(jce_mut, 2);
        self.d.jce_put(jce_mut, 3);
        0u8.jce_put(jce_mut, 4);
        0u8.jce_put(jce_mut, 5);
        self.e.jce_put(jce_mut, 6);
        jce_mut.put_head(11, tag);
    }
}

#[test]
fn test_jce_struct() {
    use bytes::Bytes;
    let data = "0a0c1c24123456783604746573744c5c680001000110020b";
    // let data = "0c1c24123456783604746573744c5c68000100011002";
    let data = hex::decode(data).unwrap();
    let mut bytes = Bytes::from(data);
    let t: TestStruct = super::from_buf(&mut bytes).unwrap();
    println!("{:?}", t);
}
