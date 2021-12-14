use bytes::Bytes;
use jcers::JceGet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default, JceGet)]
pub struct TestStruct {
    #[jce(0)]
    pub a:   u8,
    #[jce(1)]
    pub b:   i32,
    #[jce(2)]
    pub c:   f32,
    #[jce(3)]
    pub d:   String,
    #[jce(4)]
    pub sub: TestStruct2,
    #[jce(6)]
    pub e:   HashMap<u8, u8>,
}

// unsupported type
// #[derive(Debug, Clone, PartialEq, Default, JceGet, JcePut)]
// pub struct TestEnum(#[jce(0)] u8, #[jce(1)] u8);

#[derive(Debug, Clone, PartialEq, Default, JceGet)]
pub struct TestStruct2 {
    #[jce(0)]
    pub a: u8,
}

fn main() {
    let data = "0c1c24123456783604746573744a0c4b5c68000100011002";
    let data = hex::decode(data).unwrap();
    let mut bytes = Bytes::from(data);
    let t: TestStruct = jcers::from_buf(&mut bytes.clone()).unwrap();
    println!("{:?}", t);
    let value: jcers::JceStruct = jcers::from_buf(&mut bytes).unwrap();
    println!("{:?}", value);
}
