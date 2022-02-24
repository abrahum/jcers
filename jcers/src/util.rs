/// Jce Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JceType {
    /// 0
    U8,
    /// 0
    Bool,
    /// 1
    I16,
    /// 2
    I32,
    /// 3
    I64,
    /// 4
    F32,
    /// 5
    F64,
    /// 6
    ShortString,
    /// 7
    LongString,
    /// 8
    Map,
    /// 9
    List,
    /// 10
    Struct,
    /// 11
    StructEnd,
    /// 12
    Empty,
    /// 13
    Bytes,
    /// ..
    Unknown,
    /// for Default
    UnInit,
}

impl From<u8> for JceType {
    fn from(ty: u8) -> JceType {
        match ty {
            0 => JceType::U8,
            1 => JceType::I16,
            2 => JceType::I32,
            3 => JceType::I64,
            4 => JceType::F32,
            5 => JceType::F64,
            6 => JceType::ShortString,
            7 => JceType::LongString,
            8 => JceType::Map,
            9 => JceType::List,
            10 => JceType::Struct,
            11 => JceType::StructEnd,
            12 => JceType::Empty,
            13 => JceType::Bytes,
            _ => JceType::Unknown,
        }
    }
}

impl Default for JceType {
    fn default() -> Self {
        JceType::UnInit
    }
}

impl std::fmt::Display for JceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JceType::U8 => write!(f, "U8"),
            JceType::Bool => write!(f, "Bool"),
            JceType::I16 => write!(f, "I16"),
            JceType::I32 => write!(f, "I32"),
            JceType::I64 => write!(f, "I64"),
            JceType::F32 => write!(f, "F32"),
            JceType::F64 => write!(f, "F64"),
            JceType::ShortString => write!(f, "ShortString"),
            JceType::LongString => write!(f, "LongString"),
            JceType::Map => write!(f, "Map"),
            JceType::List => write!(f, "List"),
            JceType::Struct => write!(f, "Struct"),
            JceType::StructEnd => write!(f, "StructEnd"),
            JceType::Empty => write!(f, "Empty"),
            JceType::Bytes => write!(f, "Bytes"),
            JceType::Unknown => write!(f, "Unknown"),
            JceType::UnInit => write!(f, "UnInit"),
        }
    }
}

/// Jce head code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JceHead {
    pub ty:  JceType,
    pub tag: u8,
}
