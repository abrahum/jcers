# Jcers

A Encode/Decode lib for Jce.

## Features

- `derive`: derive macro support

## How to use

```rust
use jcers::{JceGet, JcePut};

#[derive(Debug, Default, JceGet, JcePut)]
pub struct YouJceStruct {  // only support NamedStruct
    #[jce(0)]              // jce tag
    pub a_named_field: u8, // a field
}

let s: YouJceStruct = jcers::from_buf(&mut buf).unwrap(); // buf should impl bytes::Buf
```

## JceType

| JceCode |   JceType   |   RustType    |
| :-----: | :---------: | :-----------: |
|    0    |     u8      |      u8       |
|    1    |     i16     |      i16      |
|    2    |     i32     |      i32      |
|    3    |     i64     |      i64      |
|    4    |     f32     |      f32      |
|    5    |     f64     |      f64      |
|    6    | ShortString |    String     |
|    7    | LongString  |    String     |
|    8    |     Map     | HashMap<K, V> |
|    9    |    List     |    Vec<V\>    |
|   10    |   Struct    |       T       |
|   11    |  StructEnd  |       -       |
|   12    |    Empty    | T::default()  |
|   13    |    bytes    | bytes::Bytes  |
