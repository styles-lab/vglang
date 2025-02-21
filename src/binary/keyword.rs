/// Basic Keyword variant.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Keyword {
    Bool,
    String,
    Byte,
    Ubyte,
    Short,
    Ushort,
    Int,
    Uint,
    Long,
    Ulong,
    Float,
    Double,
    Seq,
    None,
    Path,
    Target,
    Pop,
}

impl Keyword {
    /// Max keyword.
    pub const MAX: Keyword = Self::Pop;

    /// Convert from opcode `type_id` to binary `type_id`.
    pub fn to_binary_type_id(id: usize) -> u8 {
        let type_id = u8::from(Self::MAX) as usize + id;

        assert!(
            !(type_id > u8::MAX as usize),
            "opcode type_id out of range: {}",
            id
        );

        type_id as u8
    }
}

impl From<Keyword> for u8 {
    fn from(value: Keyword) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for Keyword {
    type Error = usize;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Keyword::Bool),
            1 => Ok(Keyword::String),
            2 => Ok(Keyword::Byte),
            3 => Ok(Keyword::Ubyte),
            4 => Ok(Keyword::Short),
            5 => Ok(Keyword::Ushort),
            6 => Ok(Keyword::Int),
            7 => Ok(Keyword::Uint),
            8 => Ok(Keyword::Long),
            9 => Ok(Keyword::Ulong),
            10 => Ok(Keyword::Float),
            11 => Ok(Keyword::Double),
            12 => Ok(Keyword::Seq),
            13 => Ok(Keyword::None),
            14 => Ok(Keyword::Path),
            15 => Ok(Keyword::Target),
            16 => Ok(Keyword::Pop),
            _ => Err((value - u8::from(Keyword::MAX)) as usize),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::panic::catch_unwind;

    use super::*;

    #[test]
    fn test_keyword() {
        assert_eq!(Keyword::to_binary_type_id(10), u8::from(Keyword::MAX) + 10);

        catch_unwind(|| {
            Keyword::to_binary_type_id(255);
        })
        .expect_err("out of range");

        assert_eq!(Keyword::try_from(u8::from(Keyword::MAX) + 1), Err(1));

        let keywords = vec![
            Keyword::Bool,
            Keyword::String,
            Keyword::Byte,
            Keyword::Ubyte,
            Keyword::Short,
            Keyword::Ushort,
            Keyword::Int,
            Keyword::Uint,
            Keyword::Long,
            Keyword::Ulong,
            Keyword::Float,
            Keyword::Double,
            Keyword::Seq,
        ];

        for keyword in keywords {
            assert_eq!(Keyword::try_from(u8::from(keyword)), Ok(keyword));
        }
    }
}
