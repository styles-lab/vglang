impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Angle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Deg(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "angle", "deg", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Grad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "angle", "grad", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Rad(p0) => {
                let mut serializer =
                    serializer.serialize_enum(0usize, "angle", "rad", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Angle {
    type Value = super::opcode::Angle;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Angle;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Angle::Deg(
                        node.deserialize_field::<f32>("deg", 0usize, None)?,
                    )),
                    1usize => Ok(super::opcode::Angle::Grad(
                        node.deserialize_field::<f32>("grad", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::Angle::Rad(
                        node.deserialize_field::<f32>("rad", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("angle".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "deg" => Ok(super::opcode::Angle::Deg(
                        node.deserialize_field::<f32>("deg", 0usize, None)?,
                    )),
                    "grad" => Ok(super::opcode::Angle::Grad(
                        node.deserialize_field::<f32>("grad", 0usize, None)?,
                    )),
                    "rad" => Ok(super::opcode::Angle::Rad(
                        node.deserialize_field::<f32>("rad", 0usize, None)?,
                    )),
                    _ => {
                        Err(Error::UnknownVariant("angle".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(0usize, "angle", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Length {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Em(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "em", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Ex(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "ex", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Px(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "px", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Inch(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "inch", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Cm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "cm", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Mm(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "mm", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Pt(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "pt", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Pc(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "pc", 7usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Percent(p0) => {
                let mut serializer =
                    serializer.serialize_enum(1usize, "length", "percent", 8usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Length {
    type Value = super::opcode::Length;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Length;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Length::Em(
                        node.deserialize_field::<f32>("em", 0usize, None)?,
                    )),
                    1usize => Ok(super::opcode::Length::Ex(
                        node.deserialize_field::<f32>("ex", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::Length::Px(
                        node.deserialize_field::<f32>("px", 0usize, None)?,
                    )),
                    3usize => Ok(super::opcode::Length::Inch(
                        node.deserialize_field::<f32>("inch", 0usize, None)?,
                    )),
                    4usize => Ok(super::opcode::Length::Cm(
                        node.deserialize_field::<f32>("cm", 0usize, None)?,
                    )),
                    5usize => Ok(super::opcode::Length::Mm(
                        node.deserialize_field::<f32>("mm", 0usize, None)?,
                    )),
                    6usize => Ok(super::opcode::Length::Pt(
                        node.deserialize_field::<f32>("pt", 0usize, None)?,
                    )),
                    7usize => Ok(super::opcode::Length::Pc(
                        node.deserialize_field::<f32>("pc", 0usize, None)?,
                    )),
                    8usize => Ok(super::opcode::Length::Percent(
                        node.deserialize_field::<f32>("percent", 0usize, None)?,
                    )),
                    _ => {
                        Err(Error::UnknownVariantIndex("length".to_string(), variant_index).into())
                    }
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "em" => Ok(super::opcode::Length::Em(
                        node.deserialize_field::<f32>("em", 0usize, None)?,
                    )),
                    "ex" => Ok(super::opcode::Length::Ex(
                        node.deserialize_field::<f32>("ex", 0usize, None)?,
                    )),
                    "px" => Ok(super::opcode::Length::Px(
                        node.deserialize_field::<f32>("px", 0usize, None)?,
                    )),
                    "inch" => Ok(super::opcode::Length::Inch(
                        node.deserialize_field::<f32>("inch", 0usize, None)?,
                    )),
                    "cm" => Ok(super::opcode::Length::Cm(
                        node.deserialize_field::<f32>("cm", 0usize, None)?,
                    )),
                    "mm" => Ok(super::opcode::Length::Mm(
                        node.deserialize_field::<f32>("mm", 0usize, None)?,
                    )),
                    "pt" => Ok(super::opcode::Length::Pt(
                        node.deserialize_field::<f32>("pt", 0usize, None)?,
                    )),
                    "pc" => Ok(super::opcode::Length::Pc(
                        node.deserialize_field::<f32>("pc", 0usize, None)?,
                    )),
                    "percent" => Ok(super::opcode::Length::Percent(
                        node.deserialize_field::<f32>("percent", 0usize, None)?,
                    )),
                    _ => {
                        Err(Error::UnknownVariant("length".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(1usize, "length", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::ColorKeyWord {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Aliceblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "aliceblue",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Antiquewhite => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "antiquewhite",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Aqua => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "aqua", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Aquamarine => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "aquamarine",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Azure => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "azure", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Beige => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "beige", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::Bisque => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "bisque", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::Black => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "black", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::Blanchedalmond => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "blanchedalmond",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Blue => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "blue", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::Blueviolet => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "blueviolet",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Brown => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "brown", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::Burlywood => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "burlywood",
                    12usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cadetblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "cadetblue",
                    13usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Chartreuse => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "chartreuse",
                    14usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Chocolate => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "chocolate",
                    15usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Coral => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "coral", 16usize, 0usize)?;
                serializer.finish()
            }
            Self::Cornflowerblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "cornflowerblue",
                    17usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cornsilk => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "cornsilk",
                    18usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Crimson => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "crimson",
                    19usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cyan => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "cyan", 20usize, 0usize)?;
                serializer.finish()
            }
            Self::Darkblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkblue",
                    21usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkcyan => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkcyan",
                    22usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkgoldenrod => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkgoldenrod",
                    23usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkgray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkgray",
                    24usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkgreen",
                    25usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkgrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkgrey",
                    26usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkkhaki => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkkhaki",
                    27usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkmagenta => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkmagenta",
                    28usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkolivegreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkolivegreen",
                    29usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkorange => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkorange",
                    30usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkorchid => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkorchid",
                    31usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkred",
                    32usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darksalmon => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darksalmon",
                    33usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkseagreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkseagreen",
                    34usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkslateblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkslateblue",
                    35usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkslategray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkslategray",
                    36usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkslategrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkslategrey",
                    37usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkturquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkturquoise",
                    38usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Darkviolet => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "darkviolet",
                    39usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Deeppink => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "deeppink",
                    40usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Deepskyblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "deepskyblue",
                    41usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dimgray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "dimgray",
                    42usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dimgrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "dimgrey",
                    43usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dodgerblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "dodgerblue",
                    44usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Firebrick => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "firebrick",
                    45usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Floralwhite => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "floralwhite",
                    46usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Forestgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "forestgreen",
                    47usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Fuchsia => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "fuchsia",
                    48usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Gainsboro => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "gainsboro",
                    49usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ghostwhite => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "ghostwhite",
                    50usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Gold => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "gold", 51usize, 0usize)?;
                serializer.finish()
            }
            Self::Goldenrod => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "goldenrod",
                    52usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Gray => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "gray", 53usize, 0usize)?;
                serializer.finish()
            }
            Self::Grey => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "grey", 54usize, 0usize)?;
                serializer.finish()
            }
            Self::Green => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "green", 55usize, 0usize)?;
                serializer.finish()
            }
            Self::Greenyellow => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "greenyellow",
                    56usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Honeydew => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "honeydew",
                    57usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hotpink => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "hotpink",
                    58usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Indianred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "indianred",
                    59usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Indigo => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "indigo", 60usize, 0usize)?;
                serializer.finish()
            }
            Self::Ivory => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "ivory", 61usize, 0usize)?;
                serializer.finish()
            }
            Self::Khaki => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "khaki", 62usize, 0usize)?;
                serializer.finish()
            }
            Self::Lavender => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lavender",
                    63usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lavenderblush => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lavenderblush",
                    64usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lawngreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lawngreen",
                    65usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lemonchiffon => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lemonchiffon",
                    66usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightblue",
                    67usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightcoral => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightcoral",
                    68usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightcyan => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightcyan",
                    69usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgoldenrodyellow => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightgoldenrodyellow",
                    70usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightgray",
                    71usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightgreen",
                    72usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightgrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightgrey",
                    73usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightpink => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightpink",
                    74usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightsalmon => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightsalmon",
                    75usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightseagreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightseagreen",
                    76usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightskyblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightskyblue",
                    77usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightslategray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightslategray",
                    78usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightslategrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightslategrey",
                    79usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightsteelblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightsteelblue",
                    80usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lightyellow => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "lightyellow",
                    81usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Lime => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "lime", 82usize, 0usize)?;
                serializer.finish()
            }
            Self::Limegreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "limegreen",
                    83usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Linen => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "linen", 84usize, 0usize)?;
                serializer.finish()
            }
            Self::Magenta => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "magenta",
                    85usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Maroon => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "maroon", 86usize, 0usize)?;
                serializer.finish()
            }
            Self::Mediumaquamarine => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumaquamarine",
                    87usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumblue",
                    88usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumorchid => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumorchid",
                    89usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumpurple => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumpurple",
                    90usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumseagreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumseagreen",
                    91usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumslateblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumslateblue",
                    92usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumspringgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumspringgreen",
                    93usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumturquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumturquoise",
                    94usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mediumvioletred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mediumvioletred",
                    95usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Midnightblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "midnightblue",
                    96usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mintcream => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mintcream",
                    97usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mistyrose => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "mistyrose",
                    98usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Moccasin => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "moccasin",
                    99usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Navajowhite => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "navajowhite",
                    100usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Navy => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "navy", 101usize, 0usize)?;
                serializer.finish()
            }
            Self::Oldlace => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "oldlace",
                    102usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Olive => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "olive", 103usize, 0usize)?;
                serializer.finish()
            }
            Self::Olivedrab => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "olivedrab",
                    104usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Orange => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "orange",
                    105usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Orangered => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "orangered",
                    106usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Orchid => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "orchid",
                    107usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palegoldenrod => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "palegoldenrod",
                    108usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palegreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "palegreen",
                    109usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Paleturquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "paleturquoise",
                    110usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Palevioletred => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "palevioletred",
                    111usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Papayawhip => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "papayawhip",
                    112usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Peachpuff => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "peachpuff",
                    113usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Peru => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "peru", 114usize, 0usize)?;
                serializer.finish()
            }
            Self::Pink => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "pink", 115usize, 0usize)?;
                serializer.finish()
            }
            Self::Plum => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "plum", 116usize, 0usize)?;
                serializer.finish()
            }
            Self::Powderblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "powderblue",
                    117usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Purple => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "purple",
                    118usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Red => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "red", 119usize, 0usize)?;
                serializer.finish()
            }
            Self::Rosybrown => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "rosybrown",
                    120usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Royalblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "royalblue",
                    121usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Saddlebrown => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "saddlebrown",
                    122usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Salmon => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "salmon",
                    123usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Sandybrown => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "sandybrown",
                    124usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Seagreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "seagreen",
                    125usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Seashell => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "seashell",
                    126usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Sienna => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "sienna",
                    127usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Silver => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "silver",
                    128usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Skyblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "skyblue",
                    129usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Slateblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "slateblue",
                    130usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Slategray => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "slategray",
                    131usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Slategrey => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "slategrey",
                    132usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Snow => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "snow", 133usize, 0usize)?;
                serializer.finish()
            }
            Self::Springgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "springgreen",
                    134usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Steelblue => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "steelblue",
                    135usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Tan => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "tan", 136usize, 0usize)?;
                serializer.finish()
            }
            Self::Teal => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "teal", 137usize, 0usize)?;
                serializer.finish()
            }
            Self::Thistle => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "thistle",
                    138usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Tomato => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "tomato",
                    139usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Turquoise => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "turquoise",
                    140usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Violet => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "violet",
                    141usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Wheat => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "wheat", 142usize, 0usize)?;
                serializer.finish()
            }
            Self::White => {
                let serializer =
                    serializer.serialize_enum(2usize, "colorKeyWord", "white", 143usize, 0usize)?;
                serializer.finish()
            }
            Self::Whitesmoke => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "whitesmoke",
                    144usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Yellow => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "yellow",
                    145usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Yellowgreen => {
                let serializer = serializer.serialize_enum(
                    2usize,
                    "colorKeyWord",
                    "yellowgreen",
                    146usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::ColorKeyWord {
    type Value = super::opcode::ColorKeyWord;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::ColorKeyWord;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::ColorKeyWord::Aliceblue),
                    1usize => Ok(super::opcode::ColorKeyWord::Antiquewhite),
                    2usize => Ok(super::opcode::ColorKeyWord::Aqua),
                    3usize => Ok(super::opcode::ColorKeyWord::Aquamarine),
                    4usize => Ok(super::opcode::ColorKeyWord::Azure),
                    5usize => Ok(super::opcode::ColorKeyWord::Beige),
                    6usize => Ok(super::opcode::ColorKeyWord::Bisque),
                    7usize => Ok(super::opcode::ColorKeyWord::Black),
                    8usize => Ok(super::opcode::ColorKeyWord::Blanchedalmond),
                    9usize => Ok(super::opcode::ColorKeyWord::Blue),
                    10usize => Ok(super::opcode::ColorKeyWord::Blueviolet),
                    11usize => Ok(super::opcode::ColorKeyWord::Brown),
                    12usize => Ok(super::opcode::ColorKeyWord::Burlywood),
                    13usize => Ok(super::opcode::ColorKeyWord::Cadetblue),
                    14usize => Ok(super::opcode::ColorKeyWord::Chartreuse),
                    15usize => Ok(super::opcode::ColorKeyWord::Chocolate),
                    16usize => Ok(super::opcode::ColorKeyWord::Coral),
                    17usize => Ok(super::opcode::ColorKeyWord::Cornflowerblue),
                    18usize => Ok(super::opcode::ColorKeyWord::Cornsilk),
                    19usize => Ok(super::opcode::ColorKeyWord::Crimson),
                    20usize => Ok(super::opcode::ColorKeyWord::Cyan),
                    21usize => Ok(super::opcode::ColorKeyWord::Darkblue),
                    22usize => Ok(super::opcode::ColorKeyWord::Darkcyan),
                    23usize => Ok(super::opcode::ColorKeyWord::Darkgoldenrod),
                    24usize => Ok(super::opcode::ColorKeyWord::Darkgray),
                    25usize => Ok(super::opcode::ColorKeyWord::Darkgreen),
                    26usize => Ok(super::opcode::ColorKeyWord::Darkgrey),
                    27usize => Ok(super::opcode::ColorKeyWord::Darkkhaki),
                    28usize => Ok(super::opcode::ColorKeyWord::Darkmagenta),
                    29usize => Ok(super::opcode::ColorKeyWord::Darkolivegreen),
                    30usize => Ok(super::opcode::ColorKeyWord::Darkorange),
                    31usize => Ok(super::opcode::ColorKeyWord::Darkorchid),
                    32usize => Ok(super::opcode::ColorKeyWord::Darkred),
                    33usize => Ok(super::opcode::ColorKeyWord::Darksalmon),
                    34usize => Ok(super::opcode::ColorKeyWord::Darkseagreen),
                    35usize => Ok(super::opcode::ColorKeyWord::Darkslateblue),
                    36usize => Ok(super::opcode::ColorKeyWord::Darkslategray),
                    37usize => Ok(super::opcode::ColorKeyWord::Darkslategrey),
                    38usize => Ok(super::opcode::ColorKeyWord::Darkturquoise),
                    39usize => Ok(super::opcode::ColorKeyWord::Darkviolet),
                    40usize => Ok(super::opcode::ColorKeyWord::Deeppink),
                    41usize => Ok(super::opcode::ColorKeyWord::Deepskyblue),
                    42usize => Ok(super::opcode::ColorKeyWord::Dimgray),
                    43usize => Ok(super::opcode::ColorKeyWord::Dimgrey),
                    44usize => Ok(super::opcode::ColorKeyWord::Dodgerblue),
                    45usize => Ok(super::opcode::ColorKeyWord::Firebrick),
                    46usize => Ok(super::opcode::ColorKeyWord::Floralwhite),
                    47usize => Ok(super::opcode::ColorKeyWord::Forestgreen),
                    48usize => Ok(super::opcode::ColorKeyWord::Fuchsia),
                    49usize => Ok(super::opcode::ColorKeyWord::Gainsboro),
                    50usize => Ok(super::opcode::ColorKeyWord::Ghostwhite),
                    51usize => Ok(super::opcode::ColorKeyWord::Gold),
                    52usize => Ok(super::opcode::ColorKeyWord::Goldenrod),
                    53usize => Ok(super::opcode::ColorKeyWord::Gray),
                    54usize => Ok(super::opcode::ColorKeyWord::Grey),
                    55usize => Ok(super::opcode::ColorKeyWord::Green),
                    56usize => Ok(super::opcode::ColorKeyWord::Greenyellow),
                    57usize => Ok(super::opcode::ColorKeyWord::Honeydew),
                    58usize => Ok(super::opcode::ColorKeyWord::Hotpink),
                    59usize => Ok(super::opcode::ColorKeyWord::Indianred),
                    60usize => Ok(super::opcode::ColorKeyWord::Indigo),
                    61usize => Ok(super::opcode::ColorKeyWord::Ivory),
                    62usize => Ok(super::opcode::ColorKeyWord::Khaki),
                    63usize => Ok(super::opcode::ColorKeyWord::Lavender),
                    64usize => Ok(super::opcode::ColorKeyWord::Lavenderblush),
                    65usize => Ok(super::opcode::ColorKeyWord::Lawngreen),
                    66usize => Ok(super::opcode::ColorKeyWord::Lemonchiffon),
                    67usize => Ok(super::opcode::ColorKeyWord::Lightblue),
                    68usize => Ok(super::opcode::ColorKeyWord::Lightcoral),
                    69usize => Ok(super::opcode::ColorKeyWord::Lightcyan),
                    70usize => Ok(super::opcode::ColorKeyWord::Lightgoldenrodyellow),
                    71usize => Ok(super::opcode::ColorKeyWord::Lightgray),
                    72usize => Ok(super::opcode::ColorKeyWord::Lightgreen),
                    73usize => Ok(super::opcode::ColorKeyWord::Lightgrey),
                    74usize => Ok(super::opcode::ColorKeyWord::Lightpink),
                    75usize => Ok(super::opcode::ColorKeyWord::Lightsalmon),
                    76usize => Ok(super::opcode::ColorKeyWord::Lightseagreen),
                    77usize => Ok(super::opcode::ColorKeyWord::Lightskyblue),
                    78usize => Ok(super::opcode::ColorKeyWord::Lightslategray),
                    79usize => Ok(super::opcode::ColorKeyWord::Lightslategrey),
                    80usize => Ok(super::opcode::ColorKeyWord::Lightsteelblue),
                    81usize => Ok(super::opcode::ColorKeyWord::Lightyellow),
                    82usize => Ok(super::opcode::ColorKeyWord::Lime),
                    83usize => Ok(super::opcode::ColorKeyWord::Limegreen),
                    84usize => Ok(super::opcode::ColorKeyWord::Linen),
                    85usize => Ok(super::opcode::ColorKeyWord::Magenta),
                    86usize => Ok(super::opcode::ColorKeyWord::Maroon),
                    87usize => Ok(super::opcode::ColorKeyWord::Mediumaquamarine),
                    88usize => Ok(super::opcode::ColorKeyWord::Mediumblue),
                    89usize => Ok(super::opcode::ColorKeyWord::Mediumorchid),
                    90usize => Ok(super::opcode::ColorKeyWord::Mediumpurple),
                    91usize => Ok(super::opcode::ColorKeyWord::Mediumseagreen),
                    92usize => Ok(super::opcode::ColorKeyWord::Mediumslateblue),
                    93usize => Ok(super::opcode::ColorKeyWord::Mediumspringgreen),
                    94usize => Ok(super::opcode::ColorKeyWord::Mediumturquoise),
                    95usize => Ok(super::opcode::ColorKeyWord::Mediumvioletred),
                    96usize => Ok(super::opcode::ColorKeyWord::Midnightblue),
                    97usize => Ok(super::opcode::ColorKeyWord::Mintcream),
                    98usize => Ok(super::opcode::ColorKeyWord::Mistyrose),
                    99usize => Ok(super::opcode::ColorKeyWord::Moccasin),
                    100usize => Ok(super::opcode::ColorKeyWord::Navajowhite),
                    101usize => Ok(super::opcode::ColorKeyWord::Navy),
                    102usize => Ok(super::opcode::ColorKeyWord::Oldlace),
                    103usize => Ok(super::opcode::ColorKeyWord::Olive),
                    104usize => Ok(super::opcode::ColorKeyWord::Olivedrab),
                    105usize => Ok(super::opcode::ColorKeyWord::Orange),
                    106usize => Ok(super::opcode::ColorKeyWord::Orangered),
                    107usize => Ok(super::opcode::ColorKeyWord::Orchid),
                    108usize => Ok(super::opcode::ColorKeyWord::Palegoldenrod),
                    109usize => Ok(super::opcode::ColorKeyWord::Palegreen),
                    110usize => Ok(super::opcode::ColorKeyWord::Paleturquoise),
                    111usize => Ok(super::opcode::ColorKeyWord::Palevioletred),
                    112usize => Ok(super::opcode::ColorKeyWord::Papayawhip),
                    113usize => Ok(super::opcode::ColorKeyWord::Peachpuff),
                    114usize => Ok(super::opcode::ColorKeyWord::Peru),
                    115usize => Ok(super::opcode::ColorKeyWord::Pink),
                    116usize => Ok(super::opcode::ColorKeyWord::Plum),
                    117usize => Ok(super::opcode::ColorKeyWord::Powderblue),
                    118usize => Ok(super::opcode::ColorKeyWord::Purple),
                    119usize => Ok(super::opcode::ColorKeyWord::Red),
                    120usize => Ok(super::opcode::ColorKeyWord::Rosybrown),
                    121usize => Ok(super::opcode::ColorKeyWord::Royalblue),
                    122usize => Ok(super::opcode::ColorKeyWord::Saddlebrown),
                    123usize => Ok(super::opcode::ColorKeyWord::Salmon),
                    124usize => Ok(super::opcode::ColorKeyWord::Sandybrown),
                    125usize => Ok(super::opcode::ColorKeyWord::Seagreen),
                    126usize => Ok(super::opcode::ColorKeyWord::Seashell),
                    127usize => Ok(super::opcode::ColorKeyWord::Sienna),
                    128usize => Ok(super::opcode::ColorKeyWord::Silver),
                    129usize => Ok(super::opcode::ColorKeyWord::Skyblue),
                    130usize => Ok(super::opcode::ColorKeyWord::Slateblue),
                    131usize => Ok(super::opcode::ColorKeyWord::Slategray),
                    132usize => Ok(super::opcode::ColorKeyWord::Slategrey),
                    133usize => Ok(super::opcode::ColorKeyWord::Snow),
                    134usize => Ok(super::opcode::ColorKeyWord::Springgreen),
                    135usize => Ok(super::opcode::ColorKeyWord::Steelblue),
                    136usize => Ok(super::opcode::ColorKeyWord::Tan),
                    137usize => Ok(super::opcode::ColorKeyWord::Teal),
                    138usize => Ok(super::opcode::ColorKeyWord::Thistle),
                    139usize => Ok(super::opcode::ColorKeyWord::Tomato),
                    140usize => Ok(super::opcode::ColorKeyWord::Turquoise),
                    141usize => Ok(super::opcode::ColorKeyWord::Violet),
                    142usize => Ok(super::opcode::ColorKeyWord::Wheat),
                    143usize => Ok(super::opcode::ColorKeyWord::White),
                    144usize => Ok(super::opcode::ColorKeyWord::Whitesmoke),
                    145usize => Ok(super::opcode::ColorKeyWord::Yellow),
                    146usize => Ok(super::opcode::ColorKeyWord::Yellowgreen),
                    _ => Err(
                        Error::UnknownVariantIndex("colorKeyWord".to_string(), variant_index)
                            .into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "aliceblue" => Ok(super::opcode::ColorKeyWord::Aliceblue),
                    "antiquewhite" => Ok(super::opcode::ColorKeyWord::Antiquewhite),
                    "aqua" => Ok(super::opcode::ColorKeyWord::Aqua),
                    "aquamarine" => Ok(super::opcode::ColorKeyWord::Aquamarine),
                    "azure" => Ok(super::opcode::ColorKeyWord::Azure),
                    "beige" => Ok(super::opcode::ColorKeyWord::Beige),
                    "bisque" => Ok(super::opcode::ColorKeyWord::Bisque),
                    "black" => Ok(super::opcode::ColorKeyWord::Black),
                    "blanchedalmond" => Ok(super::opcode::ColorKeyWord::Blanchedalmond),
                    "blue" => Ok(super::opcode::ColorKeyWord::Blue),
                    "blueviolet" => Ok(super::opcode::ColorKeyWord::Blueviolet),
                    "brown" => Ok(super::opcode::ColorKeyWord::Brown),
                    "burlywood" => Ok(super::opcode::ColorKeyWord::Burlywood),
                    "cadetblue" => Ok(super::opcode::ColorKeyWord::Cadetblue),
                    "chartreuse" => Ok(super::opcode::ColorKeyWord::Chartreuse),
                    "chocolate" => Ok(super::opcode::ColorKeyWord::Chocolate),
                    "coral" => Ok(super::opcode::ColorKeyWord::Coral),
                    "cornflowerblue" => Ok(super::opcode::ColorKeyWord::Cornflowerblue),
                    "cornsilk" => Ok(super::opcode::ColorKeyWord::Cornsilk),
                    "crimson" => Ok(super::opcode::ColorKeyWord::Crimson),
                    "cyan" => Ok(super::opcode::ColorKeyWord::Cyan),
                    "darkblue" => Ok(super::opcode::ColorKeyWord::Darkblue),
                    "darkcyan" => Ok(super::opcode::ColorKeyWord::Darkcyan),
                    "darkgoldenrod" => Ok(super::opcode::ColorKeyWord::Darkgoldenrod),
                    "darkgray" => Ok(super::opcode::ColorKeyWord::Darkgray),
                    "darkgreen" => Ok(super::opcode::ColorKeyWord::Darkgreen),
                    "darkgrey" => Ok(super::opcode::ColorKeyWord::Darkgrey),
                    "darkkhaki" => Ok(super::opcode::ColorKeyWord::Darkkhaki),
                    "darkmagenta" => Ok(super::opcode::ColorKeyWord::Darkmagenta),
                    "darkolivegreen" => Ok(super::opcode::ColorKeyWord::Darkolivegreen),
                    "darkorange" => Ok(super::opcode::ColorKeyWord::Darkorange),
                    "darkorchid" => Ok(super::opcode::ColorKeyWord::Darkorchid),
                    "darkred" => Ok(super::opcode::ColorKeyWord::Darkred),
                    "darksalmon" => Ok(super::opcode::ColorKeyWord::Darksalmon),
                    "darkseagreen" => Ok(super::opcode::ColorKeyWord::Darkseagreen),
                    "darkslateblue" => Ok(super::opcode::ColorKeyWord::Darkslateblue),
                    "darkslategray" => Ok(super::opcode::ColorKeyWord::Darkslategray),
                    "darkslategrey" => Ok(super::opcode::ColorKeyWord::Darkslategrey),
                    "darkturquoise" => Ok(super::opcode::ColorKeyWord::Darkturquoise),
                    "darkviolet" => Ok(super::opcode::ColorKeyWord::Darkviolet),
                    "deeppink" => Ok(super::opcode::ColorKeyWord::Deeppink),
                    "deepskyblue" => Ok(super::opcode::ColorKeyWord::Deepskyblue),
                    "dimgray" => Ok(super::opcode::ColorKeyWord::Dimgray),
                    "dimgrey" => Ok(super::opcode::ColorKeyWord::Dimgrey),
                    "dodgerblue" => Ok(super::opcode::ColorKeyWord::Dodgerblue),
                    "firebrick" => Ok(super::opcode::ColorKeyWord::Firebrick),
                    "floralwhite" => Ok(super::opcode::ColorKeyWord::Floralwhite),
                    "forestgreen" => Ok(super::opcode::ColorKeyWord::Forestgreen),
                    "fuchsia" => Ok(super::opcode::ColorKeyWord::Fuchsia),
                    "gainsboro" => Ok(super::opcode::ColorKeyWord::Gainsboro),
                    "ghostwhite" => Ok(super::opcode::ColorKeyWord::Ghostwhite),
                    "gold" => Ok(super::opcode::ColorKeyWord::Gold),
                    "goldenrod" => Ok(super::opcode::ColorKeyWord::Goldenrod),
                    "gray" => Ok(super::opcode::ColorKeyWord::Gray),
                    "grey" => Ok(super::opcode::ColorKeyWord::Grey),
                    "green" => Ok(super::opcode::ColorKeyWord::Green),
                    "greenyellow" => Ok(super::opcode::ColorKeyWord::Greenyellow),
                    "honeydew" => Ok(super::opcode::ColorKeyWord::Honeydew),
                    "hotpink" => Ok(super::opcode::ColorKeyWord::Hotpink),
                    "indianred" => Ok(super::opcode::ColorKeyWord::Indianred),
                    "indigo" => Ok(super::opcode::ColorKeyWord::Indigo),
                    "ivory" => Ok(super::opcode::ColorKeyWord::Ivory),
                    "khaki" => Ok(super::opcode::ColorKeyWord::Khaki),
                    "lavender" => Ok(super::opcode::ColorKeyWord::Lavender),
                    "lavenderblush" => Ok(super::opcode::ColorKeyWord::Lavenderblush),
                    "lawngreen" => Ok(super::opcode::ColorKeyWord::Lawngreen),
                    "lemonchiffon" => Ok(super::opcode::ColorKeyWord::Lemonchiffon),
                    "lightblue" => Ok(super::opcode::ColorKeyWord::Lightblue),
                    "lightcoral" => Ok(super::opcode::ColorKeyWord::Lightcoral),
                    "lightcyan" => Ok(super::opcode::ColorKeyWord::Lightcyan),
                    "lightgoldenrodyellow" => Ok(super::opcode::ColorKeyWord::Lightgoldenrodyellow),
                    "lightgray" => Ok(super::opcode::ColorKeyWord::Lightgray),
                    "lightgreen" => Ok(super::opcode::ColorKeyWord::Lightgreen),
                    "lightgrey" => Ok(super::opcode::ColorKeyWord::Lightgrey),
                    "lightpink" => Ok(super::opcode::ColorKeyWord::Lightpink),
                    "lightsalmon" => Ok(super::opcode::ColorKeyWord::Lightsalmon),
                    "lightseagreen" => Ok(super::opcode::ColorKeyWord::Lightseagreen),
                    "lightskyblue" => Ok(super::opcode::ColorKeyWord::Lightskyblue),
                    "lightslategray" => Ok(super::opcode::ColorKeyWord::Lightslategray),
                    "lightslategrey" => Ok(super::opcode::ColorKeyWord::Lightslategrey),
                    "lightsteelblue" => Ok(super::opcode::ColorKeyWord::Lightsteelblue),
                    "lightyellow" => Ok(super::opcode::ColorKeyWord::Lightyellow),
                    "lime" => Ok(super::opcode::ColorKeyWord::Lime),
                    "limegreen" => Ok(super::opcode::ColorKeyWord::Limegreen),
                    "linen" => Ok(super::opcode::ColorKeyWord::Linen),
                    "magenta" => Ok(super::opcode::ColorKeyWord::Magenta),
                    "maroon" => Ok(super::opcode::ColorKeyWord::Maroon),
                    "mediumaquamarine" => Ok(super::opcode::ColorKeyWord::Mediumaquamarine),
                    "mediumblue" => Ok(super::opcode::ColorKeyWord::Mediumblue),
                    "mediumorchid" => Ok(super::opcode::ColorKeyWord::Mediumorchid),
                    "mediumpurple" => Ok(super::opcode::ColorKeyWord::Mediumpurple),
                    "mediumseagreen" => Ok(super::opcode::ColorKeyWord::Mediumseagreen),
                    "mediumslateblue" => Ok(super::opcode::ColorKeyWord::Mediumslateblue),
                    "mediumspringgreen" => Ok(super::opcode::ColorKeyWord::Mediumspringgreen),
                    "mediumturquoise" => Ok(super::opcode::ColorKeyWord::Mediumturquoise),
                    "mediumvioletred" => Ok(super::opcode::ColorKeyWord::Mediumvioletred),
                    "midnightblue" => Ok(super::opcode::ColorKeyWord::Midnightblue),
                    "mintcream" => Ok(super::opcode::ColorKeyWord::Mintcream),
                    "mistyrose" => Ok(super::opcode::ColorKeyWord::Mistyrose),
                    "moccasin" => Ok(super::opcode::ColorKeyWord::Moccasin),
                    "navajowhite" => Ok(super::opcode::ColorKeyWord::Navajowhite),
                    "navy" => Ok(super::opcode::ColorKeyWord::Navy),
                    "oldlace" => Ok(super::opcode::ColorKeyWord::Oldlace),
                    "olive" => Ok(super::opcode::ColorKeyWord::Olive),
                    "olivedrab" => Ok(super::opcode::ColorKeyWord::Olivedrab),
                    "orange" => Ok(super::opcode::ColorKeyWord::Orange),
                    "orangered" => Ok(super::opcode::ColorKeyWord::Orangered),
                    "orchid" => Ok(super::opcode::ColorKeyWord::Orchid),
                    "palegoldenrod" => Ok(super::opcode::ColorKeyWord::Palegoldenrod),
                    "palegreen" => Ok(super::opcode::ColorKeyWord::Palegreen),
                    "paleturquoise" => Ok(super::opcode::ColorKeyWord::Paleturquoise),
                    "palevioletred" => Ok(super::opcode::ColorKeyWord::Palevioletred),
                    "papayawhip" => Ok(super::opcode::ColorKeyWord::Papayawhip),
                    "peachpuff" => Ok(super::opcode::ColorKeyWord::Peachpuff),
                    "peru" => Ok(super::opcode::ColorKeyWord::Peru),
                    "pink" => Ok(super::opcode::ColorKeyWord::Pink),
                    "plum" => Ok(super::opcode::ColorKeyWord::Plum),
                    "powderblue" => Ok(super::opcode::ColorKeyWord::Powderblue),
                    "purple" => Ok(super::opcode::ColorKeyWord::Purple),
                    "red" => Ok(super::opcode::ColorKeyWord::Red),
                    "rosybrown" => Ok(super::opcode::ColorKeyWord::Rosybrown),
                    "royalblue" => Ok(super::opcode::ColorKeyWord::Royalblue),
                    "saddlebrown" => Ok(super::opcode::ColorKeyWord::Saddlebrown),
                    "salmon" => Ok(super::opcode::ColorKeyWord::Salmon),
                    "sandybrown" => Ok(super::opcode::ColorKeyWord::Sandybrown),
                    "seagreen" => Ok(super::opcode::ColorKeyWord::Seagreen),
                    "seashell" => Ok(super::opcode::ColorKeyWord::Seashell),
                    "sienna" => Ok(super::opcode::ColorKeyWord::Sienna),
                    "silver" => Ok(super::opcode::ColorKeyWord::Silver),
                    "skyblue" => Ok(super::opcode::ColorKeyWord::Skyblue),
                    "slateblue" => Ok(super::opcode::ColorKeyWord::Slateblue),
                    "slategray" => Ok(super::opcode::ColorKeyWord::Slategray),
                    "slategrey" => Ok(super::opcode::ColorKeyWord::Slategrey),
                    "snow" => Ok(super::opcode::ColorKeyWord::Snow),
                    "springgreen" => Ok(super::opcode::ColorKeyWord::Springgreen),
                    "steelblue" => Ok(super::opcode::ColorKeyWord::Steelblue),
                    "tan" => Ok(super::opcode::ColorKeyWord::Tan),
                    "teal" => Ok(super::opcode::ColorKeyWord::Teal),
                    "thistle" => Ok(super::opcode::ColorKeyWord::Thistle),
                    "tomato" => Ok(super::opcode::ColorKeyWord::Tomato),
                    "turquoise" => Ok(super::opcode::ColorKeyWord::Turquoise),
                    "violet" => Ok(super::opcode::ColorKeyWord::Violet),
                    "wheat" => Ok(super::opcode::ColorKeyWord::Wheat),
                    "white" => Ok(super::opcode::ColorKeyWord::White),
                    "whitesmoke" => Ok(super::opcode::ColorKeyWord::Whitesmoke),
                    "yellow" => Ok(super::opcode::ColorKeyWord::Yellow),
                    "yellowgreen" => Ok(super::opcode::ColorKeyWord::Yellowgreen),
                    _ => Err(Error::UnknownVariant(
                        "colorKeyWord".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(2usize, "colorKeyWord", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Color {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Rgb(p0, p1, p2) => {
                let mut serializer =
                    serializer.serialize_enum(3usize, "color", "rgb", 0usize, 3usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.serialize_field(2usize, None, p2)?;
                serializer.finish()
            }
            Self::Keyword(p0) => {
                let mut serializer =
                    serializer.serialize_enum(3usize, "color", "keyword", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Color {
    type Value = super::opcode::Color;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Color;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Color::Rgb(
                        node.deserialize_field::<u8>("rgb", 0usize, None)?,
                        node.deserialize_field::<u8>("rgb", 1usize, None)?,
                        node.deserialize_field::<u8>("rgb", 2usize, None)?,
                    )),
                    1usize => Ok(super::opcode::Color::Keyword(
                        node.deserialize_field::<super::opcode::ColorKeyWord>(
                            "keyword", 0usize, None,
                        )?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("color".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "rgb" => Ok(super::opcode::Color::Rgb(
                        node.deserialize_field::<u8>("rgb", 0usize, None)?,
                        node.deserialize_field::<u8>("rgb", 1usize, None)?,
                        node.deserialize_field::<u8>("rgb", 2usize, None)?,
                    )),
                    "keyword" => Ok(super::opcode::Color::Keyword(
                        node.deserialize_field::<super::opcode::ColorKeyWord>(
                            "keyword", 0usize, None,
                        )?,
                    )),
                    _ => {
                        Err(Error::UnknownVariant("color".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(3usize, "color", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Iri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Local(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "iri", "local", 0usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Path(p0) => {
                let mut serializer =
                    serializer.serialize_enum(4usize, "iri", "path", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Iri {
    type Value = super::opcode::Iri;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Iri;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Iri::Local(
                        node.deserialize_field::<String>("local", 0usize, None)?,
                    )),
                    1usize => Ok(super::opcode::Iri::Path(
                        node.deserialize_field::<String>("path", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("iri".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "local" => Ok(super::opcode::Iri::Local(
                        node.deserialize_field::<String>("local", 0usize, None)?,
                    )),
                    "path" => Ok(super::opcode::Iri::Path(
                        node.deserialize_field::<String>("path", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariant("iri".to_string(), variant.to_string()).into()),
                }
            }
        }
        deserializer.deserialize_enum(4usize, "iri", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FuncIri {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(5usize, "funcIri", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FuncIri {
    type Value = super::opcode::FuncIri;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FuncIri;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FuncIri(data.deserialize_field::<String>("funcIri", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_data(5usize, "funcIri", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Point {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(6usize, "point", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Point {
    type Value = super::opcode::Point;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Point;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Point(
                    data.deserialize_field::<f32>("point", 0usize, None)?,
                    data.deserialize_field::<f32>("point", 1usize, None)?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_data(6usize, "point", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Percent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(7usize, "percent", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Percent {
    type Value = super::opcode::Percent;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Percent;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Percent(data.deserialize_field::<f32>("percent", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_data(7usize, "percent", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Paint {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let serializer =
                    serializer.serialize_enum(8usize, "paint", "none", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Color(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "paint", "color", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Server(p0) => {
                let mut serializer =
                    serializer.serialize_enum(8usize, "paint", "server", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Paint {
    type Value = super::opcode::Paint;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Paint;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Paint::None),
                    1usize => Ok(super::opcode::Paint::Color(
                        node.deserialize_field::<super::opcode::Color>("color", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::Paint::Server(
                        node.deserialize_field::<super::opcode::FuncIri>("server", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("paint".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "none" => Ok(super::opcode::Paint::None),
                    "color" => Ok(super::opcode::Paint::Color(
                        node.deserialize_field::<super::opcode::Color>("color", 0usize, None)?,
                    )),
                    "server" => Ok(super::opcode::Paint::Server(
                        node.deserialize_field::<super::opcode::FuncIri>("server", 0usize, None)?,
                    )),
                    _ => {
                        Err(Error::UnknownVariant("paint".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(8usize, "paint", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::NumberOptNumber {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(9usize, "numberOptNumber", 2usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.serialize_field(1usize, None, &self.1)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::NumberOptNumber {
    type Value = super::opcode::NumberOptNumber;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::NumberOptNumber;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = NumberOptNumber(
                    data.deserialize_field::<f32>("numberOptNumber", 0usize, None)?,
                    data.deserialize_field::<Option<f32>>("numberOptNumber", 1usize, None)?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_data(9usize, "numberOptNumber", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Coords {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::UserSpaceOnUse => {
                let serializer = serializer.serialize_enum(
                    10usize,
                    "coords",
                    "userSpaceOnUse",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ObjectBoundingBox => {
                let serializer = serializer.serialize_enum(
                    10usize,
                    "coords",
                    "objectBoundingBox",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Coords {
    type Value = super::opcode::Coords;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Coords;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Coords::UserSpaceOnUse),
                    1usize => Ok(super::opcode::Coords::ObjectBoundingBox),
                    _ => {
                        Err(Error::UnknownVariantIndex("coords".to_string(), variant_index).into())
                    }
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "userSpaceOnUse" => Ok(super::opcode::Coords::UserSpaceOnUse),
                    "objectBoundingBox" => Ok(super::opcode::Coords::ObjectBoundingBox),
                    _ => {
                        Err(Error::UnknownVariant("coords".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(10usize, "coords", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Transform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Translate(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "translate", 0usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Matrix(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "matrix", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Scale(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "scale", 2usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Rotate { angle, center } => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "rotate", 3usize, 2usize)?;
                serializer.serialize_field(0usize, Some("angle"), angle)?;
                serializer.serialize_field(1usize, Some("center"), center)?;
                serializer.finish()
            }
            Self::SkewX(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "skewX", 4usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::SkewY(p0) => {
                let mut serializer =
                    serializer.serialize_enum(11usize, "transform", "skewY", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Transform {
    type Value = super::opcode::Transform;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Transform;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Transform::Translate(
                        node.deserialize_field::<f32>("translate", 0usize, None)?,
                        node.deserialize_field::<Option<f32>>("translate", 1usize, None)?,
                    )),
                    1usize => Ok(super::opcode::Transform::Matrix(
                        node.deserialize_field::<[f32; 6usize]>("matrix", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::Transform::Scale(
                        node.deserialize_field::<f32>("scale", 0usize, None)?,
                        node.deserialize_field::<Option<f32>>("scale", 1usize, None)?,
                    )),
                    3usize => Ok(super::opcode::Transform::Rotate {
                        angle: node.deserialize_field::<f32>("rotate", 0usize, Some("angle"))?,
                        center: node.deserialize_field::<Option<super::opcode::Point>>(
                            "rotate",
                            1usize,
                            Some("center"),
                        )?,
                    }),
                    4usize => Ok(super::opcode::Transform::SkewX(
                        node.deserialize_field::<f32>("skewX", 0usize, None)?,
                    )),
                    5usize => Ok(super::opcode::Transform::SkewY(
                        node.deserialize_field::<f32>("skewY", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariantIndex("transform".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "translate" => Ok(super::opcode::Transform::Translate(
                        node.deserialize_field::<f32>("translate", 0usize, None)?,
                        node.deserialize_field::<Option<f32>>("translate", 1usize, None)?,
                    )),
                    "matrix" => Ok(super::opcode::Transform::Matrix(
                        node.deserialize_field::<[f32; 6usize]>("matrix", 0usize, None)?,
                    )),
                    "scale" => Ok(super::opcode::Transform::Scale(
                        node.deserialize_field::<f32>("scale", 0usize, None)?,
                        node.deserialize_field::<Option<f32>>("scale", 1usize, None)?,
                    )),
                    "rotate" => Ok(super::opcode::Transform::Rotate {
                        angle: node.deserialize_field::<f32>("rotate", 0usize, Some("angle"))?,
                        center: node.deserialize_field::<Option<super::opcode::Point>>(
                            "rotate",
                            1usize,
                            Some("center"),
                        )?,
                    }),
                    "skewX" => Ok(super::opcode::Transform::SkewX(
                        node.deserialize_field::<f32>("skewX", 0usize, None)?,
                    )),
                    "skewY" => Ok(super::opcode::Transform::SkewY(
                        node.deserialize_field::<f32>("skewY", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariant("transform".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(11usize, "transform", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Channel {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::R => {
                let serializer =
                    serializer.serialize_enum(12usize, "channel", "r", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::G => {
                let serializer =
                    serializer.serialize_enum(12usize, "channel", "g", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::B => {
                let serializer =
                    serializer.serialize_enum(12usize, "channel", "b", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::A => {
                let serializer =
                    serializer.serialize_enum(12usize, "channel", "a", 3usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Channel {
    type Value = super::opcode::Channel;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Channel;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Channel::R),
                    1usize => Ok(super::opcode::Channel::G),
                    2usize => Ok(super::opcode::Channel::B),
                    3usize => Ok(super::opcode::Channel::A),
                    _ => {
                        Err(Error::UnknownVariantIndex("channel".to_string(), variant_index).into())
                    }
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "r" => Ok(super::opcode::Channel::R),
                    "g" => Ok(super::opcode::Channel::G),
                    "b" => Ok(super::opcode::Channel::B),
                    "a" => Ok(super::opcode::Channel::A),
                    _ => Err(
                        Error::UnknownVariant("channel".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(12usize, "channel", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::ClipRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let serializer =
                    serializer.serialize_enum(13usize, "clipRule", "nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::EvenOdd => {
                let serializer =
                    serializer.serialize_enum(13usize, "clipRule", "evenOdd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::ClipRule {
    type Value = super::opcode::ClipRule;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::ClipRule;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::ClipRule::Nonzero),
                    1usize => Ok(super::opcode::ClipRule::EvenOdd),
                    _ => Err(
                        Error::UnknownVariantIndex("clipRule".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "nonzero" => Ok(super::opcode::ClipRule::Nonzero),
                    "evenOdd" => Ok(super::opcode::ClipRule::EvenOdd),
                    _ => Err(
                        Error::UnknownVariant("clipRule".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(13usize, "clipRule", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::CubicBezier {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(14usize, "cubicBezier", 3usize)?;
        serializer.serialize_field(0usize, Some("ctrl1"), &self.ctrl1)?;
        serializer.serialize_field(1usize, Some("ctrl2"), &self.ctrl2)?;
        serializer.serialize_field(2usize, Some("to"), &self.to)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::CubicBezier {
    type Value = super::opcode::CubicBezier;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::CubicBezier;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = CubicBezier {
                    ctrl1: data.deserialize_field::<Point>("cubicBezier", 0usize, Some("ctrl1"))?,
                    ctrl2: data.deserialize_field::<Point>("cubicBezier", 1usize, Some("ctrl2"))?,
                    to: data.deserialize_field::<Point>("cubicBezier", 2usize, Some("to"))?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_data(14usize, "cubicBezier", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::CubicBezierSmooth {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(15usize, "cubicBezierSmooth", 2usize)?;
        serializer.serialize_field(0usize, Some("ctrl2"), &self.ctrl2)?;
        serializer.serialize_field(1usize, Some("to"), &self.to)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::CubicBezierSmooth {
    type Value = super::opcode::CubicBezierSmooth;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::CubicBezierSmooth;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = CubicBezierSmooth {
                    ctrl2: data.deserialize_field::<Point>(
                        "cubicBezierSmooth",
                        0usize,
                        Some("ctrl2"),
                    )?,
                    to: data.deserialize_field::<Point>("cubicBezierSmooth", 1usize, Some("to"))?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_data(15usize, "cubicBezierSmooth", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::QuadraticBezier {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(16usize, "quadraticBezier", 2usize)?;
        serializer.serialize_field(0usize, Some("ctrl"), &self.ctrl)?;
        serializer.serialize_field(1usize, Some("to"), &self.to)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::QuadraticBezier {
    type Value = super::opcode::QuadraticBezier;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::QuadraticBezier;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = QuadraticBezier {
                    ctrl: data.deserialize_field::<Point>(
                        "quadraticBezier",
                        0usize,
                        Some("ctrl"),
                    )?,
                    to: data.deserialize_field::<Point>("quadraticBezier", 1usize, Some("to"))?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_data(16usize, "quadraticBezier", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Arc {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(17usize, "arc", 6usize)?;
        serializer.serialize_field(0usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(1usize, Some("ry"), &self.ry)?;
        serializer.serialize_field(2usize, Some("xRotation"), &self.x_rotation)?;
        serializer.serialize_field(3usize, Some("largeArc"), &self.large_arc)?;
        serializer.serialize_field(4usize, Some("sweep"), &self.sweep)?;
        serializer.serialize_field(5usize, Some("to"), &self.to)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Arc {
    type Value = super::opcode::Arc;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Arc;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Arc {
                    rx: data.deserialize_field::<f32>("arc", 0usize, Some("rx"))?,
                    ry: data.deserialize_field::<f32>("arc", 1usize, Some("ry"))?,
                    x_rotation: data.deserialize_field::<f32>("arc", 2usize, Some("xRotation"))?,
                    large_arc: data.deserialize_field::<bool>("arc", 3usize, Some("largeArc"))?,
                    sweep: data.deserialize_field::<bool>("arc", 4usize, Some("sweep"))?,
                    to: data.deserialize_field::<Point>("arc", 5usize, Some("to"))?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_data(17usize, "arc", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::PathEvent {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Close => {
                let serializer =
                    serializer.serialize_enum(18usize, "pathEvent", "close", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::MoveTo { points, relative } => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "pathEvent", "moveTo", 1usize, 2usize)?;
                serializer.serialize_field(0usize, Some("points"), points)?;
                serializer.serialize_field(1usize, Some("relative"), relative)?;
                serializer.finish()
            }
            Self::LineTo { points, relative } => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "pathEvent", "lineTo", 2usize, 2usize)?;
                serializer.serialize_field(0usize, Some("points"), points)?;
                serializer.serialize_field(1usize, Some("relative"), relative)?;
                serializer.finish()
            }
            Self::Horizontal(p0, p1) => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "pathEvent",
                    "horizontal",
                    3usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Vertical(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "pathEvent", "vertical", 4usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::CubicBezier(p0, p1) => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "pathEvent",
                    "cubicBezier",
                    5usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::CubicBezierSmooth(p0, p1) => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "pathEvent",
                    "cubicBezierSmooth",
                    6usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::QuadraticBezier(p0, p1) => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "pathEvent",
                    "quadraticBezier",
                    7usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::QuadraticBezierSmooth(p0, p1) => {
                let mut serializer = serializer.serialize_enum(
                    18usize,
                    "pathEvent",
                    "quadraticBezierSmooth",
                    8usize,
                    2usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
            Self::Arc(p0, p1) => {
                let mut serializer =
                    serializer.serialize_enum(18usize, "pathEvent", "arc", 9usize, 2usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.serialize_field(1usize, None, p1)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::PathEvent {
    type Value = super::opcode::PathEvent;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::PathEvent;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::PathEvent::Close),
                    1usize => Ok(super::opcode::PathEvent::MoveTo {
                        points: node.deserialize_field::<Vec<super::opcode::Point>>(
                            "moveTo",
                            0usize,
                            Some("points"),
                        )?,
                        relative: node.deserialize_field::<bool>(
                            "moveTo",
                            1usize,
                            Some("relative"),
                        )?,
                    }),
                    2usize => Ok(super::opcode::PathEvent::LineTo {
                        points: node.deserialize_field::<Vec<super::opcode::Point>>(
                            "lineTo",
                            0usize,
                            Some("points"),
                        )?,
                        relative: node.deserialize_field::<bool>(
                            "lineTo",
                            1usize,
                            Some("relative"),
                        )?,
                    }),
                    3usize => Ok(super::opcode::PathEvent::Horizontal(
                        node.deserialize_field::<f32>("horizontal", 0usize, None)?,
                        node.deserialize_field::<bool>("horizontal", 1usize, None)?,
                    )),
                    4usize => Ok(super::opcode::PathEvent::Vertical(
                        node.deserialize_field::<f32>("vertical", 0usize, None)?,
                        node.deserialize_field::<bool>("vertical", 1usize, None)?,
                    )),
                    5usize => Ok(super::opcode::PathEvent::CubicBezier(
                        node.deserialize_field::<Vec<super::opcode::CubicBezier>>(
                            "cubicBezier",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("cubicBezier", 1usize, None)?,
                    )),
                    6usize => Ok(super::opcode::PathEvent::CubicBezierSmooth(
                        node.deserialize_field::<Vec<super::opcode::CubicBezierSmooth>>(
                            "cubicBezierSmooth",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("cubicBezierSmooth", 1usize, None)?,
                    )),
                    7usize => Ok(super::opcode::PathEvent::QuadraticBezier(
                        node.deserialize_field::<Vec<super::opcode::QuadraticBezier>>(
                            "quadraticBezier",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("quadraticBezier", 1usize, None)?,
                    )),
                    8usize => Ok(super::opcode::PathEvent::QuadraticBezierSmooth(
                        node.deserialize_field::<Vec<super::opcode::Point>>(
                            "quadraticBezierSmooth",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("quadraticBezierSmooth", 1usize, None)?,
                    )),
                    9usize => Ok(super::opcode::PathEvent::Arc(
                        node.deserialize_field::<Vec<super::opcode::Arc>>("arc", 0usize, None)?,
                        node.deserialize_field::<bool>("arc", 1usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariantIndex("pathEvent".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "close" => Ok(super::opcode::PathEvent::Close),
                    "moveTo" => Ok(super::opcode::PathEvent::MoveTo {
                        points: node.deserialize_field::<Vec<super::opcode::Point>>(
                            "moveTo",
                            0usize,
                            Some("points"),
                        )?,
                        relative: node.deserialize_field::<bool>(
                            "moveTo",
                            1usize,
                            Some("relative"),
                        )?,
                    }),
                    "lineTo" => Ok(super::opcode::PathEvent::LineTo {
                        points: node.deserialize_field::<Vec<super::opcode::Point>>(
                            "lineTo",
                            0usize,
                            Some("points"),
                        )?,
                        relative: node.deserialize_field::<bool>(
                            "lineTo",
                            1usize,
                            Some("relative"),
                        )?,
                    }),
                    "horizontal" => Ok(super::opcode::PathEvent::Horizontal(
                        node.deserialize_field::<f32>("horizontal", 0usize, None)?,
                        node.deserialize_field::<bool>("horizontal", 1usize, None)?,
                    )),
                    "vertical" => Ok(super::opcode::PathEvent::Vertical(
                        node.deserialize_field::<f32>("vertical", 0usize, None)?,
                        node.deserialize_field::<bool>("vertical", 1usize, None)?,
                    )),
                    "cubicBezier" => Ok(super::opcode::PathEvent::CubicBezier(
                        node.deserialize_field::<Vec<super::opcode::CubicBezier>>(
                            "cubicBezier",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("cubicBezier", 1usize, None)?,
                    )),
                    "cubicBezierSmooth" => Ok(super::opcode::PathEvent::CubicBezierSmooth(
                        node.deserialize_field::<Vec<super::opcode::CubicBezierSmooth>>(
                            "cubicBezierSmooth",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("cubicBezierSmooth", 1usize, None)?,
                    )),
                    "quadraticBezier" => Ok(super::opcode::PathEvent::QuadraticBezier(
                        node.deserialize_field::<Vec<super::opcode::QuadraticBezier>>(
                            "quadraticBezier",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("quadraticBezier", 1usize, None)?,
                    )),
                    "quadraticBezierSmooth" => Ok(super::opcode::PathEvent::QuadraticBezierSmooth(
                        node.deserialize_field::<Vec<super::opcode::Point>>(
                            "quadraticBezierSmooth",
                            0usize,
                            None,
                        )?,
                        node.deserialize_field::<bool>("quadraticBezierSmooth", 1usize, None)?,
                    )),
                    "arc" => Ok(super::opcode::PathEvent::Arc(
                        node.deserialize_field::<Vec<super::opcode::Arc>>("arc", 0usize, None)?,
                        node.deserialize_field::<bool>("arc", 1usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariant("pathEvent".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(18usize, "pathEvent", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FillRule {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Nonzero => {
                let serializer =
                    serializer.serialize_enum(19usize, "fillRule", "nonzero", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Evenodd => {
                let serializer =
                    serializer.serialize_enum(19usize, "fillRule", "evenodd", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FillRule {
    type Value = super::opcode::FillRule;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FillRule;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FillRule::Nonzero),
                    1usize => Ok(super::opcode::FillRule::Evenodd),
                    _ => Err(
                        Error::UnknownVariantIndex("fillRule".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "nonzero" => Ok(super::opcode::FillRule::Nonzero),
                    "evenodd" => Ok(super::opcode::FillRule::Evenodd),
                    _ => Err(
                        Error::UnknownVariant("fillRule".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(19usize, "fillRule", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::StrokeLineCap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Butt => {
                let serializer =
                    serializer.serialize_enum(20usize, "stroke-linecap", "butt", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Round => {
                let serializer = serializer.serialize_enum(
                    20usize,
                    "stroke-linecap",
                    "round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Square => {
                let serializer = serializer.serialize_enum(
                    20usize,
                    "stroke-linecap",
                    "square",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::StrokeLineCap {
    type Value = super::opcode::StrokeLineCap;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::StrokeLineCap;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::StrokeLineCap::Butt),
                    1usize => Ok(super::opcode::StrokeLineCap::Round),
                    2usize => Ok(super::opcode::StrokeLineCap::Square),
                    _ => Err(Error::UnknownVariantIndex(
                        "stroke-linecap".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "butt" => Ok(super::opcode::StrokeLineCap::Butt),
                    "round" => Ok(super::opcode::StrokeLineCap::Round),
                    "square" => Ok(super::opcode::StrokeLineCap::Square),
                    _ => Err(Error::UnknownVariant(
                        "stroke-linecap".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(20usize, "stroke-linecap", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::StrokeLineJoin {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Miter(p0) => {
                let mut serializer = serializer.serialize_enum(
                    21usize,
                    "stroke-linejoin",
                    "miter",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Round => {
                let serializer = serializer.serialize_enum(
                    21usize,
                    "stroke-linejoin",
                    "round",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Bevel => {
                let serializer = serializer.serialize_enum(
                    21usize,
                    "stroke-linejoin",
                    "bevel",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::StrokeLineJoin {
    type Value = super::opcode::StrokeLineJoin;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::StrokeLineJoin;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::StrokeLineJoin::Miter(
                        node.deserialize_field::<Option<f32>>("miter", 0usize, None)?,
                    )),
                    1usize => Ok(super::opcode::StrokeLineJoin::Round),
                    2usize => Ok(super::opcode::StrokeLineJoin::Bevel),
                    _ => Err(Error::UnknownVariantIndex(
                        "stroke-linejoin".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "miter" => Ok(super::opcode::StrokeLineJoin::Miter(
                        node.deserialize_field::<Option<f32>>("miter", 0usize, None)?,
                    )),
                    "round" => Ok(super::opcode::StrokeLineJoin::Round),
                    "bevel" => Ok(super::opcode::StrokeLineJoin::Bevel),
                    _ => Err(Error::UnknownVariant(
                        "stroke-linejoin".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(21usize, "stroke-linejoin", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::SpreadMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Pad => {
                let serializer =
                    serializer.serialize_enum(22usize, "spreadMethod", "pad", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Reflect => {
                let serializer = serializer.serialize_enum(
                    22usize,
                    "spreadMethod",
                    "reflect",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Repeat => {
                let serializer =
                    serializer.serialize_enum(22usize, "spreadMethod", "repeat", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::SpreadMethod {
    type Value = super::opcode::SpreadMethod;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::SpreadMethod;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::SpreadMethod::Pad),
                    1usize => Ok(super::opcode::SpreadMethod::Reflect),
                    2usize => Ok(super::opcode::SpreadMethod::Repeat),
                    _ => Err(
                        Error::UnknownVariantIndex("spreadMethod".to_string(), variant_index)
                            .into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "pad" => Ok(super::opcode::SpreadMethod::Pad),
                    "reflect" => Ok(super::opcode::SpreadMethod::Reflect),
                    "repeat" => Ok(super::opcode::SpreadMethod::Repeat),
                    _ => Err(Error::UnknownVariant(
                        "spreadMethod".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(22usize, "spreadMethod", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FontStyle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(23usize, "fontStyle", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Italic => {
                let serializer =
                    serializer.serialize_enum(23usize, "fontStyle", "italic", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Oblique => {
                let serializer =
                    serializer.serialize_enum(23usize, "fontStyle", "oblique", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FontStyle {
    type Value = super::opcode::FontStyle;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FontStyle;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FontStyle::Normal),
                    1usize => Ok(super::opcode::FontStyle::Italic),
                    2usize => Ok(super::opcode::FontStyle::Oblique),
                    _ => Err(
                        Error::UnknownVariantIndex("fontStyle".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::FontStyle::Normal),
                    "italic" => Ok(super::opcode::FontStyle::Italic),
                    "oblique" => Ok(super::opcode::FontStyle::Oblique),
                    _ => Err(
                        Error::UnknownVariant("fontStyle".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(23usize, "fontStyle", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FontVariant {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(24usize, "fontVariant", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SmallCaps => {
                let serializer = serializer.serialize_enum(
                    24usize,
                    "fontVariant",
                    "smallCaps",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FontVariant {
    type Value = super::opcode::FontVariant;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FontVariant;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FontVariant::Normal),
                    1usize => Ok(super::opcode::FontVariant::SmallCaps),
                    _ => Err(
                        Error::UnknownVariantIndex("fontVariant".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::FontVariant::Normal),
                    "smallCaps" => Ok(super::opcode::FontVariant::SmallCaps),
                    _ => Err(
                        Error::UnknownVariant("fontVariant".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(24usize, "fontVariant", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Bold => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "bold", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Bolder => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "bolder", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighter => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "lighter", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::W100 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w100", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::W200 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w200", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::W300 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w300", 6usize, 0usize)?;
                serializer.finish()
            }
            Self::W400 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w400", 7usize, 0usize)?;
                serializer.finish()
            }
            Self::W500 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w500", 8usize, 0usize)?;
                serializer.finish()
            }
            Self::W600 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w600", 9usize, 0usize)?;
                serializer.finish()
            }
            Self::W700 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w700", 10usize, 0usize)?;
                serializer.finish()
            }
            Self::W800 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w800", 11usize, 0usize)?;
                serializer.finish()
            }
            Self::W900 => {
                let serializer =
                    serializer.serialize_enum(25usize, "fontWeight", "w900", 12usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FontWeight {
    type Value = super::opcode::FontWeight;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FontWeight;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FontWeight::Normal),
                    1usize => Ok(super::opcode::FontWeight::Bold),
                    2usize => Ok(super::opcode::FontWeight::Bolder),
                    3usize => Ok(super::opcode::FontWeight::Lighter),
                    4usize => Ok(super::opcode::FontWeight::W100),
                    5usize => Ok(super::opcode::FontWeight::W200),
                    6usize => Ok(super::opcode::FontWeight::W300),
                    7usize => Ok(super::opcode::FontWeight::W400),
                    8usize => Ok(super::opcode::FontWeight::W500),
                    9usize => Ok(super::opcode::FontWeight::W600),
                    10usize => Ok(super::opcode::FontWeight::W700),
                    11usize => Ok(super::opcode::FontWeight::W800),
                    12usize => Ok(super::opcode::FontWeight::W900),
                    _ => Err(
                        Error::UnknownVariantIndex("fontWeight".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::FontWeight::Normal),
                    "bold" => Ok(super::opcode::FontWeight::Bold),
                    "bolder" => Ok(super::opcode::FontWeight::Bolder),
                    "lighter" => Ok(super::opcode::FontWeight::Lighter),
                    "w100" => Ok(super::opcode::FontWeight::W100),
                    "w200" => Ok(super::opcode::FontWeight::W200),
                    "w300" => Ok(super::opcode::FontWeight::W300),
                    "w400" => Ok(super::opcode::FontWeight::W400),
                    "w500" => Ok(super::opcode::FontWeight::W500),
                    "w600" => Ok(super::opcode::FontWeight::W600),
                    "w700" => Ok(super::opcode::FontWeight::W700),
                    "w800" => Ok(super::opcode::FontWeight::W800),
                    "w900" => Ok(super::opcode::FontWeight::W900),
                    _ => Err(
                        Error::UnknownVariant("fontWeight".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(25usize, "fontWeight", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FontFamily {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Serif => {
                let serializer =
                    serializer.serialize_enum(26usize, "fontFamily", "serif", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SansSerif => {
                let serializer = serializer.serialize_enum(
                    26usize,
                    "fontFamily",
                    "sansSerif",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Cursive => {
                let serializer =
                    serializer.serialize_enum(26usize, "fontFamily", "cursive", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Fantasy => {
                let serializer =
                    serializer.serialize_enum(26usize, "fontFamily", "fantasy", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Monospace => {
                let serializer = serializer.serialize_enum(
                    26usize,
                    "fontFamily",
                    "monospace",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Generic(p0) => {
                let mut serializer =
                    serializer.serialize_enum(26usize, "fontFamily", "generic", 5usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FontFamily {
    type Value = super::opcode::FontFamily;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FontFamily;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FontFamily::Serif),
                    1usize => Ok(super::opcode::FontFamily::SansSerif),
                    2usize => Ok(super::opcode::FontFamily::Cursive),
                    3usize => Ok(super::opcode::FontFamily::Fantasy),
                    4usize => Ok(super::opcode::FontFamily::Monospace),
                    5usize => Ok(super::opcode::FontFamily::Generic(
                        node.deserialize_field::<String>("generic", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariantIndex("fontFamily".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "serif" => Ok(super::opcode::FontFamily::Serif),
                    "sansSerif" => Ok(super::opcode::FontFamily::SansSerif),
                    "cursive" => Ok(super::opcode::FontFamily::Cursive),
                    "fantasy" => Ok(super::opcode::FontFamily::Fantasy),
                    "monospace" => Ok(super::opcode::FontFamily::Monospace),
                    "generic" => Ok(super::opcode::FontFamily::Generic(
                        node.deserialize_field::<String>("generic", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariant("fontFamily".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(26usize, "fontFamily", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FontStretch {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(27usize, "fontStretch", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Wider => {
                let serializer =
                    serializer.serialize_enum(27usize, "fontStretch", "wider", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::Narrower => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "narrower",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraCondensed => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "ultraCondensed",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraCondensed => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "extraCondensed",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Condensed => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "condensed",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiCondensed => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "semiCondensed",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SemiExpanded => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "semiExpanded",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Expanded => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "expanded",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ExtraExpanded => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "extraExpanded",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UltraExpanded => {
                let serializer = serializer.serialize_enum(
                    27usize,
                    "fontStretch",
                    "ultraExpanded",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FontStretch {
    type Value = super::opcode::FontStretch;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FontStretch;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FontStretch::Normal),
                    1usize => Ok(super::opcode::FontStretch::Wider),
                    2usize => Ok(super::opcode::FontStretch::Narrower),
                    3usize => Ok(super::opcode::FontStretch::UltraCondensed),
                    4usize => Ok(super::opcode::FontStretch::ExtraCondensed),
                    5usize => Ok(super::opcode::FontStretch::Condensed),
                    6usize => Ok(super::opcode::FontStretch::SemiCondensed),
                    7usize => Ok(super::opcode::FontStretch::SemiExpanded),
                    8usize => Ok(super::opcode::FontStretch::Expanded),
                    9usize => Ok(super::opcode::FontStretch::ExtraExpanded),
                    10usize => Ok(super::opcode::FontStretch::UltraExpanded),
                    _ => Err(
                        Error::UnknownVariantIndex("fontStretch".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::FontStretch::Normal),
                    "wider" => Ok(super::opcode::FontStretch::Wider),
                    "narrower" => Ok(super::opcode::FontStretch::Narrower),
                    "ultraCondensed" => Ok(super::opcode::FontStretch::UltraCondensed),
                    "extraCondensed" => Ok(super::opcode::FontStretch::ExtraCondensed),
                    "condensed" => Ok(super::opcode::FontStretch::Condensed),
                    "semiCondensed" => Ok(super::opcode::FontStretch::SemiCondensed),
                    "semiExpanded" => Ok(super::opcode::FontStretch::SemiExpanded),
                    "expanded" => Ok(super::opcode::FontStretch::Expanded),
                    "extraExpanded" => Ok(super::opcode::FontStretch::ExtraExpanded),
                    "ultraExpanded" => Ok(super::opcode::FontStretch::UltraExpanded),
                    _ => Err(
                        Error::UnknownVariant("fontStretch".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(27usize, "fontStretch", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Background {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Accumulate => {
                let serializer = serializer.serialize_enum(
                    28usize,
                    "background",
                    "accumulate",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::New(p0) => {
                let mut serializer =
                    serializer.serialize_enum(28usize, "background", "new", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Background {
    type Value = super::opcode::Background;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Background;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::Background::Accumulate),
                    1usize => Ok(super::opcode::Background::New(
                        node.deserialize_field::<Option<super::opcode::BackgroundNew>>(
                            "new", 0usize, None,
                        )?,
                    )),
                    _ => Err(
                        Error::UnknownVariantIndex("background".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "accumulate" => Ok(super::opcode::Background::Accumulate),
                    "new" => Ok(super::opcode::Background::New(
                        node.deserialize_field::<Option<super::opcode::BackgroundNew>>(
                            "new", 0usize, None,
                        )?,
                    )),
                    _ => Err(
                        Error::UnknownVariant("background".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(28usize, "background", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::BackgroundNew {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_data(29usize, "backgroundNew", 4usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::BackgroundNew {
    type Value = super::opcode::BackgroundNew;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::BackgroundNew;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = BackgroundNew {
                    x: data.deserialize_field::<f32>("backgroundNew", 0usize, Some("x"))?,
                    y: data.deserialize_field::<f32>("backgroundNew", 1usize, Some("y"))?,
                    width: data.deserialize_field::<f32>("backgroundNew", 2usize, Some("width"))?,
                    height: data.deserialize_field::<f32>(
                        "backgroundNew",
                        3usize,
                        Some("height"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_data(29usize, "backgroundNew", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeIn {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::SourceGraphic => {
                let serializer =
                    serializer.serialize_enum(30usize, "feIn", "SourceGraphic", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::SourceAlpha => {
                let serializer =
                    serializer.serialize_enum(30usize, "feIn", "SourceAlpha", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BackgroundImage => {
                let serializer = serializer.serialize_enum(
                    30usize,
                    "feIn",
                    "BackgroundImage",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BackgroundAlpha => {
                let serializer = serializer.serialize_enum(
                    30usize,
                    "feIn",
                    "BackgroundAlpha",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::FillPaint => {
                let serializer =
                    serializer.serialize_enum(30usize, "feIn", "FillPaint", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::StrokePaint => {
                let serializer =
                    serializer.serialize_enum(30usize, "feIn", "StrokePaint", 5usize, 0usize)?;
                serializer.finish()
            }
            Self::Result(p0) => {
                let mut serializer =
                    serializer.serialize_enum(30usize, "feIn", "result", 6usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeIn {
    type Value = super::opcode::FeIn;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeIn;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeIn::SourceGraphic),
                    1usize => Ok(super::opcode::FeIn::SourceAlpha),
                    2usize => Ok(super::opcode::FeIn::BackgroundImage),
                    3usize => Ok(super::opcode::FeIn::BackgroundAlpha),
                    4usize => Ok(super::opcode::FeIn::FillPaint),
                    5usize => Ok(super::opcode::FeIn::StrokePaint),
                    6usize => Ok(super::opcode::FeIn::Result(
                        node.deserialize_field::<String>("result", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("feIn".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "SourceGraphic" => Ok(super::opcode::FeIn::SourceGraphic),
                    "SourceAlpha" => Ok(super::opcode::FeIn::SourceAlpha),
                    "BackgroundImage" => Ok(super::opcode::FeIn::BackgroundImage),
                    "BackgroundAlpha" => Ok(super::opcode::FeIn::BackgroundAlpha),
                    "FillPaint" => Ok(super::opcode::FeIn::FillPaint),
                    "StrokePaint" => Ok(super::opcode::FeIn::StrokePaint),
                    "result" => Ok(super::opcode::FeIn::Result(
                        node.deserialize_field::<String>("result", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariant("feIn".to_string(), variant.to_string()).into()),
                }
            }
        }
        deserializer.deserialize_enum(30usize, "feIn", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeOut {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Position => {
                let serializer =
                    serializer.serialize_enum(31usize, "feOut", "position", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Named(p0) => {
                let mut serializer =
                    serializer.serialize_enum(31usize, "feOut", "named", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeOut {
    type Value = super::opcode::FeOut;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeOut;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeOut::Position),
                    1usize => Ok(super::opcode::FeOut::Named(
                        node.deserialize_field::<String>("named", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex("feOut".to_string(), variant_index).into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "position" => Ok(super::opcode::FeOut::Position),
                    "named" => Ok(super::opcode::FeOut::Named(
                        node.deserialize_field::<String>("named", 0usize, None)?,
                    )),
                    _ => {
                        Err(Error::UnknownVariant("feOut".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(31usize, "feOut", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeBlendMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(32usize, "feBlendMode", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Multiply => {
                let serializer = serializer.serialize_enum(
                    32usize,
                    "feBlendMode",
                    "multiply",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Screen => {
                let serializer =
                    serializer.serialize_enum(32usize, "feBlendMode", "screen", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Darken => {
                let serializer =
                    serializer.serialize_enum(32usize, "feBlendMode", "darken", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Lighten => {
                let serializer =
                    serializer.serialize_enum(32usize, "feBlendMode", "lighten", 4usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeBlendMode {
    type Value = super::opcode::FeBlendMode;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeBlendMode;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeBlendMode::Normal),
                    1usize => Ok(super::opcode::FeBlendMode::Multiply),
                    2usize => Ok(super::opcode::FeBlendMode::Screen),
                    3usize => Ok(super::opcode::FeBlendMode::Darken),
                    4usize => Ok(super::opcode::FeBlendMode::Lighten),
                    _ => Err(
                        Error::UnknownVariantIndex("feBlendMode".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::FeBlendMode::Normal),
                    "multiply" => Ok(super::opcode::FeBlendMode::Multiply),
                    "screen" => Ok(super::opcode::FeBlendMode::Screen),
                    "darken" => Ok(super::opcode::FeBlendMode::Darken),
                    "lighten" => Ok(super::opcode::FeBlendMode::Lighten),
                    _ => Err(
                        Error::UnknownVariant("feBlendMode".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(32usize, "feBlendMode", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextLengthAdjust {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Spacing => {
                let serializer = serializer.serialize_enum(
                    33usize,
                    "textLengthAdjust",
                    "spacing",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SpacingAndGlyphs => {
                let serializer = serializer.serialize_enum(
                    33usize,
                    "textLengthAdjust",
                    "spacingAndGlyphs",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextLengthAdjust {
    type Value = super::opcode::TextLengthAdjust;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextLengthAdjust;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextLengthAdjust::Spacing),
                    1usize => Ok(super::opcode::TextLengthAdjust::SpacingAndGlyphs),
                    _ => Err(Error::UnknownVariantIndex(
                        "textLengthAdjust".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "spacing" => Ok(super::opcode::TextLengthAdjust::Spacing),
                    "spacingAndGlyphs" => Ok(super::opcode::TextLengthAdjust::SpacingAndGlyphs),
                    _ => Err(Error::UnknownVariant(
                        "textLengthAdjust".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(33usize, "textLengthAdjust", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WritingMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::LrTb => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "lrTb", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::RlTb => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "rlTb", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::TbRl => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "tbRl", 2usize, 0usize)?;
                serializer.finish()
            }
            Self::Lr => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "lr", 3usize, 0usize)?;
                serializer.finish()
            }
            Self::Rl => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "rl", 4usize, 0usize)?;
                serializer.finish()
            }
            Self::Tb => {
                let serializer =
                    serializer.serialize_enum(34usize, "writingMode", "tb", 5usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WritingMode {
    type Value = super::opcode::WritingMode;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WritingMode;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::WritingMode::LrTb),
                    1usize => Ok(super::opcode::WritingMode::RlTb),
                    2usize => Ok(super::opcode::WritingMode::TbRl),
                    3usize => Ok(super::opcode::WritingMode::Lr),
                    4usize => Ok(super::opcode::WritingMode::Rl),
                    5usize => Ok(super::opcode::WritingMode::Tb),
                    _ => Err(
                        Error::UnknownVariantIndex("writingMode".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "lrTb" => Ok(super::opcode::WritingMode::LrTb),
                    "rlTb" => Ok(super::opcode::WritingMode::RlTb),
                    "tbRl" => Ok(super::opcode::WritingMode::TbRl),
                    "lr" => Ok(super::opcode::WritingMode::Lr),
                    "rl" => Ok(super::opcode::WritingMode::Rl),
                    "tb" => Ok(super::opcode::WritingMode::Tb),
                    _ => Err(
                        Error::UnknownVariant("writingMode".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(34usize, "writingMode", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Ltr => {
                let serializer =
                    serializer.serialize_enum(35usize, "textDirection", "ltr", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Rtl => {
                let serializer =
                    serializer.serialize_enum(35usize, "textDirection", "rtl", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextDirection {
    type Value = super::opcode::TextDirection;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextDirection;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextDirection::Ltr),
                    1usize => Ok(super::opcode::TextDirection::Rtl),
                    _ => Err(Error::UnknownVariantIndex(
                        "textDirection".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "ltr" => Ok(super::opcode::TextDirection::Ltr),
                    "rtl" => Ok(super::opcode::TextDirection::Rtl),
                    _ => Err(Error::UnknownVariant(
                        "textDirection".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(35usize, "textDirection", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::UnicodeBidi {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(36usize, "unicodeBidi", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Embed => {
                let serializer =
                    serializer.serialize_enum(36usize, "unicodeBidi", "embed", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::BidiOverride => {
                let serializer = serializer.serialize_enum(
                    36usize,
                    "unicodeBidi",
                    "bidiOverride",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::UnicodeBidi {
    type Value = super::opcode::UnicodeBidi;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::UnicodeBidi;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::UnicodeBidi::Normal),
                    1usize => Ok(super::opcode::UnicodeBidi::Embed),
                    2usize => Ok(super::opcode::UnicodeBidi::BidiOverride),
                    _ => Err(
                        Error::UnknownVariantIndex("unicodeBidi".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::UnicodeBidi::Normal),
                    "embed" => Ok(super::opcode::UnicodeBidi::Embed),
                    "bidiOverride" => Ok(super::opcode::UnicodeBidi::BidiOverride),
                    _ => Err(
                        Error::UnknownVariant("unicodeBidi".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(36usize, "unicodeBidi", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Start => {
                let serializer =
                    serializer.serialize_enum(37usize, "textAnchor", "start", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer =
                    serializer.serialize_enum(37usize, "textAnchor", "middle", 1usize, 0usize)?;
                serializer.finish()
            }
            Self::End => {
                let serializer =
                    serializer.serialize_enum(37usize, "textAnchor", "end", 2usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextAnchor {
    type Value = super::opcode::TextAnchor;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextAnchor;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextAnchor::Start),
                    1usize => Ok(super::opcode::TextAnchor::Middle),
                    2usize => Ok(super::opcode::TextAnchor::End),
                    _ => Err(
                        Error::UnknownVariantIndex("textAnchor".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "start" => Ok(super::opcode::TextAnchor::Start),
                    "middle" => Ok(super::opcode::TextAnchor::Middle),
                    "end" => Ok(super::opcode::TextAnchor::End),
                    _ => Err(
                        Error::UnknownVariant("textAnchor".to_string(), variant.to_string()).into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(37usize, "textAnchor", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::DominantBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::UseScript => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "useScript",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoChange => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "noChange",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::ResetSize => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "resetSize",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "ideographic",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "alphabetic",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "hanging",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "mathematical",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "central",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "middle",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "textAfterEdge",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let serializer = serializer.serialize_enum(
                    38usize,
                    "dominantBaseline",
                    "textBeforeEdge",
                    11usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::DominantBaseline {
    type Value = super::opcode::DominantBaseline;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::DominantBaseline;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::DominantBaseline::Auto),
                    1usize => Ok(super::opcode::DominantBaseline::UseScript),
                    2usize => Ok(super::opcode::DominantBaseline::NoChange),
                    3usize => Ok(super::opcode::DominantBaseline::ResetSize),
                    4usize => Ok(super::opcode::DominantBaseline::Ideographic),
                    5usize => Ok(super::opcode::DominantBaseline::Alphabetic),
                    6usize => Ok(super::opcode::DominantBaseline::Hanging),
                    7usize => Ok(super::opcode::DominantBaseline::Mathematical),
                    8usize => Ok(super::opcode::DominantBaseline::Central),
                    9usize => Ok(super::opcode::DominantBaseline::Middle),
                    10usize => Ok(super::opcode::DominantBaseline::TextAfterEdge),
                    11usize => Ok(super::opcode::DominantBaseline::TextBeforeEdge),
                    _ => Err(Error::UnknownVariantIndex(
                        "dominantBaseline".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "auto" => Ok(super::opcode::DominantBaseline::Auto),
                    "useScript" => Ok(super::opcode::DominantBaseline::UseScript),
                    "noChange" => Ok(super::opcode::DominantBaseline::NoChange),
                    "resetSize" => Ok(super::opcode::DominantBaseline::ResetSize),
                    "ideographic" => Ok(super::opcode::DominantBaseline::Ideographic),
                    "alphabetic" => Ok(super::opcode::DominantBaseline::Alphabetic),
                    "hanging" => Ok(super::opcode::DominantBaseline::Hanging),
                    "mathematical" => Ok(super::opcode::DominantBaseline::Mathematical),
                    "central" => Ok(super::opcode::DominantBaseline::Central),
                    "middle" => Ok(super::opcode::DominantBaseline::Middle),
                    "textAfterEdge" => Ok(super::opcode::DominantBaseline::TextAfterEdge),
                    "textBeforeEdge" => Ok(super::opcode::DominantBaseline::TextBeforeEdge),
                    _ => Err(Error::UnknownVariant(
                        "dominantBaseline".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(38usize, "dominantBaseline", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::AlignmentBaseline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Baseline => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "baseline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::BeforeEdge => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "beforeEdge",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextBeforeEdge => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "textBeforeEdge",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Middle => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "middle",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Central => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "central",
                    5usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::AfterEdge => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "afterEdge",
                    6usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::TextAfterEdge => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "textAfterEdge",
                    7usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Ideographic => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "ideographic",
                    8usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Alphabetic => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "alphabetic",
                    9usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Hanging => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "hanging",
                    10usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Mathematical => {
                let serializer = serializer.serialize_enum(
                    39usize,
                    "alignmentBaseline",
                    "mathematical",
                    11usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::AlignmentBaseline {
    type Value = super::opcode::AlignmentBaseline;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::AlignmentBaseline;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::AlignmentBaseline::Auto),
                    1usize => Ok(super::opcode::AlignmentBaseline::Baseline),
                    2usize => Ok(super::opcode::AlignmentBaseline::BeforeEdge),
                    3usize => Ok(super::opcode::AlignmentBaseline::TextBeforeEdge),
                    4usize => Ok(super::opcode::AlignmentBaseline::Middle),
                    5usize => Ok(super::opcode::AlignmentBaseline::Central),
                    6usize => Ok(super::opcode::AlignmentBaseline::AfterEdge),
                    7usize => Ok(super::opcode::AlignmentBaseline::TextAfterEdge),
                    8usize => Ok(super::opcode::AlignmentBaseline::Ideographic),
                    9usize => Ok(super::opcode::AlignmentBaseline::Alphabetic),
                    10usize => Ok(super::opcode::AlignmentBaseline::Hanging),
                    11usize => Ok(super::opcode::AlignmentBaseline::Mathematical),
                    _ => Err(Error::UnknownVariantIndex(
                        "alignmentBaseline".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "auto" => Ok(super::opcode::AlignmentBaseline::Auto),
                    "baseline" => Ok(super::opcode::AlignmentBaseline::Baseline),
                    "beforeEdge" => Ok(super::opcode::AlignmentBaseline::BeforeEdge),
                    "textBeforeEdge" => Ok(super::opcode::AlignmentBaseline::TextBeforeEdge),
                    "middle" => Ok(super::opcode::AlignmentBaseline::Middle),
                    "central" => Ok(super::opcode::AlignmentBaseline::Central),
                    "afterEdge" => Ok(super::opcode::AlignmentBaseline::AfterEdge),
                    "textAfterEdge" => Ok(super::opcode::AlignmentBaseline::TextAfterEdge),
                    "ideographic" => Ok(super::opcode::AlignmentBaseline::Ideographic),
                    "alphabetic" => Ok(super::opcode::AlignmentBaseline::Alphabetic),
                    "hanging" => Ok(super::opcode::AlignmentBaseline::Hanging),
                    "mathematical" => Ok(super::opcode::AlignmentBaseline::Mathematical),
                    _ => Err(Error::UnknownVariant(
                        "alignmentBaseline".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(39usize, "alignmentBaseline", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::BaselineShift {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Baseline => {
                let serializer = serializer.serialize_enum(
                    40usize,
                    "baselineShift",
                    "baseline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SubScripts => {
                let serializer = serializer.serialize_enum(
                    40usize,
                    "baselineShift",
                    "subScripts",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::SuperScripts => {
                let serializer = serializer.serialize_enum(
                    40usize,
                    "baselineShift",
                    "superScripts",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Value(p0) => {
                let mut serializer =
                    serializer.serialize_enum(40usize, "baselineShift", "value", 3usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::BaselineShift {
    type Value = super::opcode::BaselineShift;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::BaselineShift;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::BaselineShift::Baseline),
                    1usize => Ok(super::opcode::BaselineShift::SubScripts),
                    2usize => Ok(super::opcode::BaselineShift::SuperScripts),
                    3usize => Ok(super::opcode::BaselineShift::Value(
                        node.deserialize_field::<super::opcode::Length>("value", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex(
                        "baselineShift".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "baseline" => Ok(super::opcode::BaselineShift::Baseline),
                    "subScripts" => Ok(super::opcode::BaselineShift::SubScripts),
                    "superScripts" => Ok(super::opcode::BaselineShift::SuperScripts),
                    "value" => Ok(super::opcode::BaselineShift::Value(
                        node.deserialize_field::<super::opcode::Length>("value", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariant(
                        "baselineShift".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(40usize, "baselineShift", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextDecoration {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Underline => {
                let serializer = serializer.serialize_enum(
                    41usize,
                    "textDecoration",
                    "underline",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Overline => {
                let serializer = serializer.serialize_enum(
                    41usize,
                    "textDecoration",
                    "overline",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::LineThrough => {
                let serializer = serializer.serialize_enum(
                    41usize,
                    "textDecoration",
                    "line-through",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Blink => {
                let serializer = serializer.serialize_enum(
                    41usize,
                    "textDecoration",
                    "blink",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextDecoration {
    type Value = super::opcode::TextDecoration;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextDecoration;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextDecoration::Underline),
                    1usize => Ok(super::opcode::TextDecoration::Overline),
                    2usize => Ok(super::opcode::TextDecoration::LineThrough),
                    3usize => Ok(super::opcode::TextDecoration::Blink),
                    _ => Err(Error::UnknownVariantIndex(
                        "textDecoration".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "underline" => Ok(super::opcode::TextDecoration::Underline),
                    "overline" => Ok(super::opcode::TextDecoration::Overline),
                    "line-through" => Ok(super::opcode::TextDecoration::LineThrough),
                    "blink" => Ok(super::opcode::TextDecoration::Blink),
                    _ => Err(Error::UnknownVariant(
                        "textDecoration".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(41usize, "textDecoration", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextPathMethod {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Align => {
                let serializer = serializer.serialize_enum(
                    42usize,
                    "textPathMethod",
                    "align",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Stretch => {
                let serializer = serializer.serialize_enum(
                    42usize,
                    "textPathMethod",
                    "stretch",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextPathMethod {
    type Value = super::opcode::TextPathMethod;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextPathMethod;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextPathMethod::Align),
                    1usize => Ok(super::opcode::TextPathMethod::Stretch),
                    _ => Err(Error::UnknownVariantIndex(
                        "textPathMethod".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "align" => Ok(super::opcode::TextPathMethod::Align),
                    "stretch" => Ok(super::opcode::TextPathMethod::Stretch),
                    _ => Err(Error::UnknownVariant(
                        "textPathMethod".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(42usize, "textPathMethod", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextPathSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Auto => {
                let serializer = serializer.serialize_enum(
                    43usize,
                    "textPathSpacing",
                    "auto",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Exact => {
                let serializer = serializer.serialize_enum(
                    43usize,
                    "textPathSpacing",
                    "exact",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextPathSpacing {
    type Value = super::opcode::TextPathSpacing;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextPathSpacing;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::TextPathSpacing::Auto),
                    1usize => Ok(super::opcode::TextPathSpacing::Exact),
                    _ => Err(Error::UnknownVariantIndex(
                        "textPathSpacing".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "auto" => Ok(super::opcode::TextPathSpacing::Auto),
                    "exact" => Ok(super::opcode::TextPathSpacing::Exact),
                    _ => Err(Error::UnknownVariant(
                        "textPathSpacing".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(43usize, "textPathSpacing", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::LetterSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer = serializer.serialize_enum(
                    44usize,
                    "letterSpacing",
                    "normal",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Length(p0) => {
                let mut serializer = serializer.serialize_enum(
                    44usize,
                    "letterSpacing",
                    "length",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::LetterSpacing {
    type Value = super::opcode::LetterSpacing;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::LetterSpacing;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::LetterSpacing::Normal),
                    1usize => Ok(super::opcode::LetterSpacing::Length(
                        node.deserialize_field::<super::opcode::Length>("length", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariantIndex(
                        "letterSpacing".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::LetterSpacing::Normal),
                    "length" => Ok(super::opcode::LetterSpacing::Length(
                        node.deserialize_field::<super::opcode::Length>("length", 0usize, None)?,
                    )),
                    _ => Err(Error::UnknownVariant(
                        "letterSpacing".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(44usize, "letterSpacing", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WordSpacing {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Normal => {
                let serializer =
                    serializer.serialize_enum(45usize, "wordSpacing", "normal", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Length(p0) => {
                let mut serializer =
                    serializer.serialize_enum(45usize, "wordSpacing", "length", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WordSpacing {
    type Value = super::opcode::WordSpacing;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WordSpacing;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::WordSpacing::Normal),
                    1usize => Ok(super::opcode::WordSpacing::Length(
                        node.deserialize_field::<super::opcode::Length>("length", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariantIndex("wordSpacing".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "normal" => Ok(super::opcode::WordSpacing::Normal),
                    "length" => Ok(super::opcode::WordSpacing::Length(
                        node.deserialize_field::<super::opcode::Length>("length", 0usize, None)?,
                    )),
                    _ => Err(
                        Error::UnknownVariant("wordSpacing".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(45usize, "wordSpacing", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::MeetOrSlice {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Meet => {
                let serializer =
                    serializer.serialize_enum(46usize, "meetOrSlice", "meet", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Slice => {
                let serializer =
                    serializer.serialize_enum(46usize, "meetOrSlice", "slice", 1usize, 0usize)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::MeetOrSlice {
    type Value = super::opcode::MeetOrSlice;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::MeetOrSlice;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::MeetOrSlice::Meet),
                    1usize => Ok(super::opcode::MeetOrSlice::Slice),
                    _ => Err(
                        Error::UnknownVariantIndex("meetOrSlice".to_string(), variant_index).into(),
                    ),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "meet" => Ok(super::opcode::MeetOrSlice::Meet),
                    "slice" => Ok(super::opcode::MeetOrSlice::Slice),
                    _ => Err(
                        Error::UnknownVariant("meetOrSlice".to_string(), variant.to_string())
                            .into(),
                    ),
                }
            }
        }
        deserializer.deserialize_enum(46usize, "meetOrSlice", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::PreserveAspectRatio {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::None => {
                let serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "none",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::XMinYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMinYMin",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMidYMin",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMin(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMaxYMin",
                    3usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMinYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMinYMid",
                    4usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMidYMid",
                    5usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMid(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMaxYMid",
                    6usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMinYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMinYMax",
                    7usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMidYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMidYMax",
                    8usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::XMaxYMax(p0) => {
                let mut serializer = serializer.serialize_enum(
                    47usize,
                    "preserveAspectRatio",
                    "xMaxYMax",
                    9usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::PreserveAspectRatio {
    type Value = super::opcode::PreserveAspectRatio;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::PreserveAspectRatio;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::PreserveAspectRatio::None),
                    1usize => Ok(super::opcode::PreserveAspectRatio::XMinYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMin", 0usize, None,
                        )?,
                    )),
                    2usize => Ok(super::opcode::PreserveAspectRatio::XMidYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMin", 0usize, None,
                        )?,
                    )),
                    3usize => Ok(super::opcode::PreserveAspectRatio::XMaxYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMin", 0usize, None,
                        )?,
                    )),
                    4usize => Ok(super::opcode::PreserveAspectRatio::XMinYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMid", 0usize, None,
                        )?,
                    )),
                    5usize => Ok(super::opcode::PreserveAspectRatio::XMidYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMid", 0usize, None,
                        )?,
                    )),
                    6usize => Ok(super::opcode::PreserveAspectRatio::XMaxYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMid", 0usize, None,
                        )?,
                    )),
                    7usize => Ok(super::opcode::PreserveAspectRatio::XMinYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMax", 0usize, None,
                        )?,
                    )),
                    8usize => Ok(super::opcode::PreserveAspectRatio::XMidYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMax", 0usize, None,
                        )?,
                    )),
                    9usize => Ok(super::opcode::PreserveAspectRatio::XMaxYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMax", 0usize, None,
                        )?,
                    )),
                    _ => Err(Error::UnknownVariantIndex(
                        "preserveAspectRatio".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "none" => Ok(super::opcode::PreserveAspectRatio::None),
                    "xMinYMin" => Ok(super::opcode::PreserveAspectRatio::XMinYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMin", 0usize, None,
                        )?,
                    )),
                    "xMidYMin" => Ok(super::opcode::PreserveAspectRatio::XMidYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMin", 0usize, None,
                        )?,
                    )),
                    "xMaxYMin" => Ok(super::opcode::PreserveAspectRatio::XMaxYMin(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMin", 0usize, None,
                        )?,
                    )),
                    "xMinYMid" => Ok(super::opcode::PreserveAspectRatio::XMinYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMid", 0usize, None,
                        )?,
                    )),
                    "xMidYMid" => Ok(super::opcode::PreserveAspectRatio::XMidYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMid", 0usize, None,
                        )?,
                    )),
                    "xMaxYMid" => Ok(super::opcode::PreserveAspectRatio::XMaxYMid(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMid", 0usize, None,
                        )?,
                    )),
                    "xMinYMax" => Ok(super::opcode::PreserveAspectRatio::XMinYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMinYMax", 0usize, None,
                        )?,
                    )),
                    "xMidYMax" => Ok(super::opcode::PreserveAspectRatio::XMidYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMidYMax", 0usize, None,
                        )?,
                    )),
                    "xMaxYMax" => Ok(super::opcode::PreserveAspectRatio::XMaxYMax(
                        node.deserialize_field::<super::opcode::MeetOrSlice>(
                            "xMaxYMax", 0usize, None,
                        )?,
                    )),
                    _ => Err(Error::UnknownVariant(
                        "preserveAspectRatio".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(47usize, "preserveAspectRatio", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextLayout {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(48usize, "textLayout", 10usize)?;
        serializer.serialize_field(0usize, Some("writeMode"), &self.write_mode)?;
        serializer.serialize_field(1usize, Some("direction"), &self.direction)?;
        serializer.serialize_field(2usize, Some("unicodeBidi"), &self.unicode_bidi)?;
        serializer.serialize_field(3usize, Some("text-anchor"), &self.anchor)?;
        serializer.serialize_field(4usize, Some("dominantBaseline"), &self.dominant_baseline)?;
        serializer.serialize_field(5usize, Some("alignmentBaseline"), &self.alignment_baseline)?;
        serializer.serialize_field(6usize, Some("baselineShift"), &self.baseline_shift)?;
        serializer.serialize_field(7usize, Some("text-decoration"), &self.decoration)?;
        serializer.serialize_field(8usize, Some("letterSpacing"), &self.letter_spacing)?;
        serializer.serialize_field(9usize, Some("wordSpacing"), &self.word_spacing)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextLayout {
    type Value = super::opcode::TextLayout;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextLayout;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = TextLayout { write_mode : data . deserialize_field :: < Option < WritingMode > > ("textLayout" , 0usize , Some ("writeMode")) ? , direction : data . deserialize_field :: < Option < TextDirection > > ("textLayout" , 1usize , Some ("direction")) ? , unicode_bidi : data . deserialize_field :: < Option < UnicodeBidi > > ("textLayout" , 2usize , Some ("unicodeBidi")) ? , anchor : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextAnchor > > > ("textLayout" , 3usize , Some ("text-anchor")) ? , dominant_baseline : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < DominantBaseline > > > ("textLayout" , 4usize , Some ("dominantBaseline")) ? , alignment_baseline : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < AlignmentBaseline > > > ("textLayout" , 5usize , Some ("alignmentBaseline")) ? , baseline_shift : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < BaselineShift > > > ("textLayout" , 6usize , Some ("baselineShift")) ? , decoration : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextDecoration > > > ("textLayout" , 7usize , Some ("text-decoration")) ? , letter_spacing : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < LetterSpacing > > > ("textLayout" , 8usize , Some ("letterSpacing")) ? , word_spacing : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < WordSpacing > > > ("textLayout" , 9usize , Some ("wordSpacing")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_attr(48usize, "textLayout", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WithTransform {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(49usize, "withTransform", 1usize)?;
        serializer.serialize_field(0usize, Some("transform"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WithTransform {
    type Value = super::opcode::WithTransform;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WithTransform;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = WithTransform(data.deserialize_field::<Vec<Transform>>(
                    "withTransform",
                    0usize,
                    Some("transform"),
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(49usize, "withTransform", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Id {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(50usize, "id", 1usize)?;
        serializer.serialize_field(0usize, Some("id"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Id {
    type Value = super::opcode::Id;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Id;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Id(data.deserialize_field::<String>("id", 0usize, Some("id"))?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(50usize, "id", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Fill {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(51usize, "fill", 3usize)?;
        serializer.serialize_field(0usize, Some("fill"), &self.paint)?;
        serializer.serialize_field(1usize, Some("fill-rule"), &self.rule)?;
        serializer.serialize_field(2usize, Some("fill-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Fill {
    type Value = super::opcode::Fill;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Fill;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Fill {
                    paint: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Paint>>>(
                            "fill",
                            0usize,
                            Some("fill"),
                        )?,
                    rule: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FillRule>>>(
                            "fill",
                            1usize,
                            Some("fill-rule"),
                        )?,
                    opacity: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "fill",
                            2usize,
                            Some("fill-opacity"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_attr(51usize, "fill", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(52usize, "stroke", 7usize)?;
        serializer.serialize_field(0usize, Some("stroke"), &self.paint)?;
        serializer.serialize_field(1usize, Some("stroke-width"), &self.width)?;
        serializer.serialize_field(2usize, Some("stroke-linecap"), &self.linecap)?;
        serializer.serialize_field(3usize, Some("stroke-linejoin"), &self.linejoin)?;
        serializer.serialize_field(4usize, Some("stroke-dasharray"), &self.dasharray)?;
        serializer.serialize_field(5usize, Some("stroke-dashoffset"), &self.dashoffset)?;
        serializer.serialize_field(6usize, Some("stroke-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Stroke {
    type Value = super::opcode::Stroke;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Stroke;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Stroke { paint : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Paint > > > ("stroke" , 0usize , Some ("stroke")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("stroke" , 1usize , Some ("stroke-width")) ? , linecap : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < StrokeLineCap > > > ("stroke" , 2usize , Some ("stroke-linecap")) ? , linejoin : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < StrokeLineJoin > > > ("stroke" , 3usize , Some ("stroke-linejoin")) ? , dasharray : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("stroke" , 4usize , Some ("stroke-dasharray")) ? , dashoffset : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("stroke" , 5usize , Some ("stroke-dashoffset")) ? , opacity : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < f32 > > > ("stroke" , 6usize , Some ("stroke-opacity")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_attr(52usize, "stroke", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Font {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(53usize, "font", 6usize)?;
        serializer.serialize_field(0usize, Some("font-family"), &self.family)?;
        serializer.serialize_field(1usize, Some("font-style"), &self.style)?;
        serializer.serialize_field(2usize, Some("font-variant"), &self.variant)?;
        serializer.serialize_field(3usize, Some("font-weight"), &self.weight)?;
        serializer.serialize_field(4usize, Some("font-size"), &self.size)?;
        serializer.serialize_field(5usize, Some("font-stretch"), &self.stretch)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Font {
    type Value = super::opcode::Font;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Font;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Font { family : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < FontFamily > > > > ("font" , 0usize , Some ("font-family")) ? , style : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FontStyle > > > ("font" , 1usize , Some ("font-style")) ? , variant : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FontVariant > > > ("font" , 2usize , Some ("font-variant")) ? , weight : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FontWeight > > > ("font" , 3usize , Some ("font-weight")) ? , size : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("font" , 4usize , Some ("font-size")) ? , stretch : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FontStretch > > > ("font" , 5usize , Some ("font-stretch")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_attr(53usize, "font", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::EnableBackground {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(54usize, "enableBackground", 1usize)?;
        serializer.serialize_field(0usize, Some("enable-background"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::EnableBackground {
    type Value = super::opcode::EnableBackground;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::EnableBackground;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = EnableBackground(data.deserialize_field::<Background>(
                    "enableBackground",
                    0usize,
                    Some("enable-background"),
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(54usize, "enableBackground", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WithFilter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(55usize, "withFilter", 1usize)?;
        serializer.serialize_field(0usize, Some("filter"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WithFilter {
    type Value = super::opcode::WithFilter;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WithFilter;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = WithFilter(data.deserialize_field::<FuncIri>(
                    "withFilter",
                    0usize,
                    Some("filter"),
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(55usize, "withFilter", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WithClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(56usize, "withClipPath", 1usize)?;
        serializer.serialize_field(0usize, Some("clip-path"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WithClipPath {
    type Value = super::opcode::WithClipPath;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WithClipPath;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = WithClipPath(data.deserialize_field::<FuncIri>(
                    "withClipPath",
                    0usize,
                    Some("clip-path"),
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(56usize, "withClipPath", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::WithMask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(57usize, "withMask", 1usize)?;
        serializer.serialize_field(0usize, Some("mask"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::WithMask {
    type Value = super::opcode::WithMask;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::WithMask;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = WithMask(data.deserialize_field::<FuncIri>(
                    "withMask",
                    0usize,
                    Some("mask"),
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(57usize, "withMask", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Opacity {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(58usize, "opacity", 1usize)?;
        serializer.serialize_field(0usize, Some("opacity"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Opacity {
    type Value = super::opcode::Opacity;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Opacity;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    Opacity(data.deserialize_field::<f32>("opacity", 0usize, Some("opacity"))?);
                Ok(value)
            }
        }
        deserializer.deserialize_attr(58usize, "opacity", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::ViewBox {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_attr(59usize, "viewBox", 5usize)?;
        serializer.serialize_field(0usize, Some("minx"), &self.minx)?;
        serializer.serialize_field(1usize, Some("miny"), &self.miny)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("aspect"), &self.aspect)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::ViewBox {
    type Value = super::opcode::ViewBox;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::ViewBox;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = ViewBox { minx : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < f32 > > ("viewBox" , 0usize , Some ("minx")) ? , miny : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < f32 > > ("viewBox" , 1usize , Some ("miny")) ? , width : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < f32 > > ("viewBox" , 2usize , Some ("width")) ? , height : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < f32 > > ("viewBox" , 3usize , Some ("height")) ? , aspect : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < PreserveAspectRatio > > > ("viewBox" , 4usize , Some ("aspect")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_attr(59usize, "viewBox", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Canvas {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(61usize, "svg", 2usize)?;
        serializer.serialize_field(0usize, Some("width"), &self.width)?;
        serializer.serialize_field(1usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Canvas {
    type Value = super::opcode::Canvas;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Canvas;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Canvas {
                    width: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "svg",
                        0usize,
                        Some("width"),
                    )?,
                    height: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "svg",
                        1usize,
                        Some("height"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_element(61usize, "svg", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Mask {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(62usize, "mask", 6usize)?;
        serializer.serialize_field(0usize, Some("maskUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("contentUnits"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Mask {
    type Value = super::opcode::Mask;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Mask;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Mask {
                    units: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Coords>>>(
                            "mask",
                            0usize,
                            Some("maskUnits"),
                        )?,
                    content_units: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Coords>>>(
                            "mask",
                            1usize,
                            Some("contentUnits"),
                        )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "mask",
                        2usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "mask",
                        3usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "mask",
                            4usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "mask",
                            5usize,
                            Some("height"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_element(62usize, "mask", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::ClipPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(63usize, "clipPath", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::ClipPath {
    type Value = super::opcode::ClipPath;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::ClipPath;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = ClipPath(
                    data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Coords>>>(
                        "clipPath", 0usize, None,
                    )?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_element(63usize, "clipPath", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Filter {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(64usize, "filter", 7usize)?;
        serializer.serialize_field(0usize, Some("filterUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("primitiveUnits"), &self.primitive_units)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("res"), &self.res)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Filter {
    type Value = super::opcode::Filter;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Filter;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Filter { units : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Coords > > > ("filter" , 0usize , Some ("filterUnits")) ? , primitive_units : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Coords > > > ("filter" , 1usize , Some ("primitiveUnits")) ? , x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("filter" , 2usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("filter" , 3usize , Some ("y")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("filter" , 4usize , Some ("width")) ? , height : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("filter" , 5usize , Some ("height")) ? , res : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < NumberOptNumber > > > ("filter" , 6usize , Some ("res")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(64usize, "filter", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeDistantLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(65usize, "feDistantLight", 2usize)?;
        serializer.serialize_field(0usize, Some("azimuth"), &self.azimuth)?;
        serializer.serialize_field(1usize, Some("elevation"), &self.elevation)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeDistantLight {
    type Value = super::opcode::FeDistantLight;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeDistantLight;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeDistantLight {
                    azimuth: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feDistantLight",
                            0usize,
                            Some("azimuth"),
                        )?,
                    elevation: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feDistantLight",
                            1usize,
                            Some("elevation"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(65usize, "feDistantLight", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FePointLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(66usize, "fePointLight", 3usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FePointLight {
    type Value = super::opcode::FePointLight;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FePointLight;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FePointLight {
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "fePointLight",
                        0usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "fePointLight",
                        1usize,
                        Some("y"),
                    )?,
                    z: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "fePointLight",
                        2usize,
                        Some("z"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(66usize, "fePointLight", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeSpotLight {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(67usize, "feSpotLight", 8usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("z"), &self.z)?;
        serializer.serialize_field(3usize, Some("pointAtX"), &self.point_at_x)?;
        serializer.serialize_field(4usize, Some("pointAtY"), &self.point_at_y)?;
        serializer.serialize_field(5usize, Some("pointAtZ"), &self.point_at_z)?;
        serializer.serialize_field(6usize, Some("specularExponent"), &self.specular_exponent)?;
        serializer.serialize_field(7usize, Some("limitingConeAngle"), &self.limiting_cone_angle)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeSpotLight {
    type Value = super::opcode::FeSpotLight;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeSpotLight;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeSpotLight {
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feSpotLight",
                        0usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feSpotLight",
                        1usize,
                        Some("y"),
                    )?,
                    z: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feSpotLight",
                        2usize,
                        Some("z"),
                    )?,
                    point_at_x: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feSpotLight",
                            3usize,
                            Some("pointAtX"),
                        )?,
                    point_at_y: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feSpotLight",
                            4usize,
                            Some("pointAtY"),
                        )?,
                    point_at_z: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feSpotLight",
                            5usize,
                            Some("pointAtZ"),
                        )?,
                    specular_exponent: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feSpotLight",
                            6usize,
                            Some("specularExponent"),
                        )?,
                    limiting_cone_angle: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feSpotLight",
                            7usize,
                            Some("limitingConeAngle"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(67usize, "feSpotLight", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeBlend {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(68usize, "feBlend", 8usize)?;
        serializer.serialize_field(0usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(1usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(2usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeBlend {
    type Value = super::opcode::FeBlend;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeBlend;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeBlend {
                    mode: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeBlendMode>>>(
                            "feBlend",
                            0usize,
                            Some("mode"),
                        )?,
                    r#in: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feBlend",
                        1usize,
                        Some("in"),
                    )?,
                    in2: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feBlend",
                        2usize,
                        Some("in2"),
                    )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feBlend",
                        3usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feBlend",
                        4usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feBlend",
                            5usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feBlend",
                            6usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feBlend",
                            7usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(68usize, "feBlend", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeColorMatrixValues {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Matrix(p0) => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "feColorMatrixValues",
                    "matrix",
                    0usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Saturate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "feColorMatrixValues",
                    "saturate",
                    1usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::HueRotate(p0) => {
                let mut serializer = serializer.serialize_enum(
                    69usize,
                    "feColorMatrixValues",
                    "hueRotate",
                    2usize,
                    1usize,
                )?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::LuminanceToAlpha => {
                let serializer = serializer.serialize_enum(
                    69usize,
                    "feColorMatrixValues",
                    "luminanceToAlpha",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeColorMatrixValues {
    type Value = super::opcode::FeColorMatrixValues;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeColorMatrixValues;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeColorMatrixValues::Matrix(
                        node.deserialize_field::<[f32; 20usize]>("matrix", 0usize, None)?,
                    )),
                    1usize => Ok(super::opcode::FeColorMatrixValues::Saturate(
                        node.deserialize_field::<f32>("saturate", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::FeColorMatrixValues::HueRotate(
                        node.deserialize_field::<f32>("hueRotate", 0usize, None)?,
                    )),
                    3usize => Ok(super::opcode::FeColorMatrixValues::LuminanceToAlpha),
                    _ => Err(Error::UnknownVariantIndex(
                        "feColorMatrixValues".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "matrix" => Ok(super::opcode::FeColorMatrixValues::Matrix(
                        node.deserialize_field::<[f32; 20usize]>("matrix", 0usize, None)?,
                    )),
                    "saturate" => Ok(super::opcode::FeColorMatrixValues::Saturate(
                        node.deserialize_field::<f32>("saturate", 0usize, None)?,
                    )),
                    "hueRotate" => Ok(super::opcode::FeColorMatrixValues::HueRotate(
                        node.deserialize_field::<f32>("hueRotate", 0usize, None)?,
                    )),
                    "luminanceToAlpha" => Ok(super::opcode::FeColorMatrixValues::LuminanceToAlpha),
                    _ => Err(Error::UnknownVariant(
                        "feColorMatrixValues".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(69usize, "feColorMatrixValues", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeColorMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(70usize, "feColorMatrix", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("values"), &self.values)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeColorMatrix {
    type Value = super::opcode::FeColorMatrix;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeColorMatrix;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeColorMatrix {
                    r#in: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feColorMatrix",
                        0usize,
                        Some("in"),
                    )?,
                    values: data
                        .deserialize_field::<mlang_rs::rt::opcode::Variable<FeColorMatrixValues>>(
                            "feColorMatrix",
                            1usize,
                            Some("values"),
                        )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feColorMatrix",
                        2usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feColorMatrix",
                        3usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feColorMatrix",
                            4usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feColorMatrix",
                            5usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feColorMatrix",
                            6usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(70usize, "feColorMatrix", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFunc {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Identity => {
                let serializer =
                    serializer.serialize_enum(71usize, "feFunc", "identity", 0usize, 0usize)?;
                serializer.finish()
            }
            Self::Table(p0) => {
                let mut serializer =
                    serializer.serialize_enum(71usize, "feFunc", "table", 1usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Discrete(p0) => {
                let mut serializer =
                    serializer.serialize_enum(71usize, "feFunc", "discrete", 2usize, 1usize)?;
                serializer.serialize_field(0usize, None, p0)?;
                serializer.finish()
            }
            Self::Linear { slope, intercept } => {
                let mut serializer =
                    serializer.serialize_enum(71usize, "feFunc", "linear", 3usize, 2usize)?;
                serializer.serialize_field(0usize, Some("slope"), slope)?;
                serializer.serialize_field(1usize, Some("intercept"), intercept)?;
                serializer.finish()
            }
            Self::Gamma {
                amplitude,
                exponent,
                offset,
            } => {
                let mut serializer =
                    serializer.serialize_enum(71usize, "feFunc", "gamma", 4usize, 3usize)?;
                serializer.serialize_field(0usize, Some("amplitude"), amplitude)?;
                serializer.serialize_field(1usize, Some("exponent"), exponent)?;
                serializer.serialize_field(2usize, Some("offset"), offset)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFunc {
    type Value = super::opcode::FeFunc;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFunc;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeFunc::Identity),
                    1usize => Ok(super::opcode::FeFunc::Table(
                        node.deserialize_field::<Vec<f32>>("table", 0usize, None)?,
                    )),
                    2usize => Ok(super::opcode::FeFunc::Discrete(
                        node.deserialize_field::<Vec<f32>>("discrete", 0usize, None)?,
                    )),
                    3usize => Ok(super::opcode::FeFunc::Linear {
                        slope: node.deserialize_field::<f32>("linear", 0usize, Some("slope"))?,
                        intercept: node.deserialize_field::<f32>(
                            "linear",
                            1usize,
                            Some("intercept"),
                        )?,
                    }),
                    4usize => Ok(super::opcode::FeFunc::Gamma {
                        amplitude: node.deserialize_field::<f32>(
                            "gamma",
                            0usize,
                            Some("amplitude"),
                        )?,
                        exponent: node.deserialize_field::<f32>(
                            "gamma",
                            1usize,
                            Some("exponent"),
                        )?,
                        offset: node.deserialize_field::<f32>("gamma", 2usize, Some("offset"))?,
                    }),
                    _ => {
                        Err(Error::UnknownVariantIndex("feFunc".to_string(), variant_index).into())
                    }
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "identity" => Ok(super::opcode::FeFunc::Identity),
                    "table" => Ok(super::opcode::FeFunc::Table(
                        node.deserialize_field::<Vec<f32>>("table", 0usize, None)?,
                    )),
                    "discrete" => Ok(super::opcode::FeFunc::Discrete(
                        node.deserialize_field::<Vec<f32>>("discrete", 0usize, None)?,
                    )),
                    "linear" => Ok(super::opcode::FeFunc::Linear {
                        slope: node.deserialize_field::<f32>("linear", 0usize, Some("slope"))?,
                        intercept: node.deserialize_field::<f32>(
                            "linear",
                            1usize,
                            Some("intercept"),
                        )?,
                    }),
                    "gamma" => Ok(super::opcode::FeFunc::Gamma {
                        amplitude: node.deserialize_field::<f32>(
                            "gamma",
                            0usize,
                            Some("amplitude"),
                        )?,
                        exponent: node.deserialize_field::<f32>(
                            "gamma",
                            1usize,
                            Some("exponent"),
                        )?,
                        offset: node.deserialize_field::<f32>("gamma", 2usize, Some("offset"))?,
                    }),
                    _ => {
                        Err(Error::UnknownVariant("feFunc".to_string(), variant.to_string()).into())
                    }
                }
            }
        }
        deserializer.deserialize_enum(71usize, "feFunc", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeCompositeOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Over => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "over",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::In => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "in",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Out => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "out",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Atop => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "atop",
                    3usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Xor => {
                let serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "xor",
                    4usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Arithmetic { k1, k2, k3, k4 } => {
                let mut serializer = serializer.serialize_enum(
                    72usize,
                    "feCompositeOperator",
                    "arithmetic",
                    5usize,
                    4usize,
                )?;
                serializer.serialize_field(0usize, Some("k1"), k1)?;
                serializer.serialize_field(1usize, Some("k2"), k2)?;
                serializer.serialize_field(2usize, Some("k3"), k3)?;
                serializer.serialize_field(3usize, Some("k4"), k4)?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeCompositeOperator {
    type Value = super::opcode::FeCompositeOperator;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeCompositeOperator;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeCompositeOperator::Over),
                    1usize => Ok(super::opcode::FeCompositeOperator::In),
                    2usize => Ok(super::opcode::FeCompositeOperator::Out),
                    3usize => Ok(super::opcode::FeCompositeOperator::Atop),
                    4usize => Ok(super::opcode::FeCompositeOperator::Xor),
                    5usize => Ok(super::opcode::FeCompositeOperator::Arithmetic {
                        k1: node.deserialize_field::<f32>("arithmetic", 0usize, Some("k1"))?,
                        k2: node.deserialize_field::<f32>("arithmetic", 1usize, Some("k2"))?,
                        k3: node.deserialize_field::<f32>("arithmetic", 2usize, Some("k3"))?,
                        k4: node.deserialize_field::<f32>("arithmetic", 3usize, Some("k4"))?,
                    }),
                    _ => Err(Error::UnknownVariantIndex(
                        "feCompositeOperator".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "over" => Ok(super::opcode::FeCompositeOperator::Over),
                    "in" => Ok(super::opcode::FeCompositeOperator::In),
                    "out" => Ok(super::opcode::FeCompositeOperator::Out),
                    "atop" => Ok(super::opcode::FeCompositeOperator::Atop),
                    "xor" => Ok(super::opcode::FeCompositeOperator::Xor),
                    "arithmetic" => Ok(super::opcode::FeCompositeOperator::Arithmetic {
                        k1: node.deserialize_field::<f32>("arithmetic", 0usize, Some("k1"))?,
                        k2: node.deserialize_field::<f32>("arithmetic", 1usize, Some("k2"))?,
                        k3: node.deserialize_field::<f32>("arithmetic", 2usize, Some("k3"))?,
                        k4: node.deserialize_field::<f32>("arithmetic", 3usize, Some("k4"))?,
                    }),
                    _ => Err(Error::UnknownVariant(
                        "feCompositeOperator".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(72usize, "feCompositeOperator", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrixEdgeMode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Duplicate => {
                let serializer = serializer.serialize_enum(
                    73usize,
                    "feConvolveMatrixEdgeMode",
                    "duplicate",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Wrap => {
                let serializer = serializer.serialize_enum(
                    73usize,
                    "feConvolveMatrixEdgeMode",
                    "wrap",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::None => {
                let serializer = serializer.serialize_enum(
                    73usize,
                    "feConvolveMatrixEdgeMode",
                    "none",
                    2usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeConvolveMatrixEdgeMode {
    type Value = super::opcode::FeConvolveMatrixEdgeMode;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeConvolveMatrixEdgeMode;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeConvolveMatrixEdgeMode::Duplicate),
                    1usize => Ok(super::opcode::FeConvolveMatrixEdgeMode::Wrap),
                    2usize => Ok(super::opcode::FeConvolveMatrixEdgeMode::None),
                    _ => Err(Error::UnknownVariantIndex(
                        "feConvolveMatrixEdgeMode".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "duplicate" => Ok(super::opcode::FeConvolveMatrixEdgeMode::Duplicate),
                    "wrap" => Ok(super::opcode::FeConvolveMatrixEdgeMode::Wrap),
                    "none" => Ok(super::opcode::FeConvolveMatrixEdgeMode::None),
                    _ => Err(Error::UnknownVariant(
                        "feConvolveMatrixEdgeMode".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(73usize, "feConvolveMatrixEdgeMode", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeMorphologyOperator {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Erode => {
                let serializer = serializer.serialize_enum(
                    74usize,
                    "feMorphologyOperator",
                    "erode",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Dilate => {
                let serializer = serializer.serialize_enum(
                    74usize,
                    "feMorphologyOperator",
                    "dilate",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeMorphologyOperator {
    type Value = super::opcode::FeMorphologyOperator;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeMorphologyOperator;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeMorphologyOperator::Erode),
                    1usize => Ok(super::opcode::FeMorphologyOperator::Dilate),
                    _ => Err(Error::UnknownVariantIndex(
                        "feMorphologyOperator".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "erode" => Ok(super::opcode::FeMorphologyOperator::Erode),
                    "dilate" => Ok(super::opcode::FeMorphologyOperator::Dilate),
                    _ => Err(Error::UnknownVariant(
                        "feMorphologyOperator".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(74usize, "feMorphologyOperator", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeStitchTiles {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::Stitch => {
                let serializer = serializer.serialize_enum(
                    75usize,
                    "feStitchTiles",
                    "stitch",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::NoStitch => {
                let serializer = serializer.serialize_enum(
                    75usize,
                    "feStitchTiles",
                    "noStitch",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeStitchTiles {
    type Value = super::opcode::FeStitchTiles;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeStitchTiles;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeStitchTiles::Stitch),
                    1usize => Ok(super::opcode::FeStitchTiles::NoStitch),
                    _ => Err(Error::UnknownVariantIndex(
                        "feStitchTiles".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "stitch" => Ok(super::opcode::FeStitchTiles::Stitch),
                    "noStitch" => Ok(super::opcode::FeStitchTiles::NoStitch),
                    _ => Err(Error::UnknownVariant(
                        "feStitchTiles".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(75usize, "feStitchTiles", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeTurbulenceType {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        match self {
            Self::FractalNoise => {
                let serializer = serializer.serialize_enum(
                    76usize,
                    "feTurbulenceType",
                    "fractalNoise",
                    0usize,
                    0usize,
                )?;
                serializer.finish()
            }
            Self::Turbulence => {
                let serializer = serializer.serialize_enum(
                    76usize,
                    "feTurbulenceType",
                    "turbulence",
                    1usize,
                    0usize,
                )?;
                serializer.finish()
            }
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeTurbulenceType {
    type Value = super::opcode::FeTurbulenceType;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeTurbulenceType;
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum<A>(
                self,
                variant_index: usize,
                mut node: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant_index {
                    0usize => Ok(super::opcode::FeTurbulenceType::FractalNoise),
                    1usize => Ok(super::opcode::FeTurbulenceType::Turbulence),
                    _ => Err(Error::UnknownVariantIndex(
                        "feTurbulenceType".to_string(),
                        variant_index,
                    )
                    .into()),
                }
            }
            #[doc = r" Visit enum field."]
            #[allow(unused_mut)]
            fn visit_enum_with<A>(self, variant: &str, mut node: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = node;
                match variant {
                    "fractalNoise" => Ok(super::opcode::FeTurbulenceType::FractalNoise),
                    "turbulence" => Ok(super::opcode::FeTurbulenceType::Turbulence),
                    _ => Err(Error::UnknownVariant(
                        "feTurbulenceType".to_string(),
                        variant.to_string(),
                    )
                    .into()),
                }
            }
        }
        deserializer.deserialize_enum(76usize, "feTurbulenceType", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeComponentTransfer {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(77usize, "feComponentTransfer", 1usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeComponentTransfer {
    type Value = super::opcode::FeComponentTransfer;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeComponentTransfer;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeComponentTransfer(data.deserialize_field::<Option<
                    mlang_rs::rt::opcode::Variable<FeIn>,
                >>(
                    "feComponentTransfer", 0usize, Some("in")
                )?);
                Ok(value)
            }
        }
        deserializer.deserialize_element(77usize, "feComponentTransfer", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFuncA {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(78usize, "feFuncA", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFuncA {
    type Value = super::opcode::FeFuncA;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFuncA;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeFuncA(data.deserialize_field::<FeFunc>("feFuncA", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(78usize, "feFuncA", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFuncR {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(79usize, "feFuncR", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFuncR {
    type Value = super::opcode::FeFuncR;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFuncR;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeFuncR(data.deserialize_field::<FeFunc>("feFuncR", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(79usize, "feFuncR", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFuncG {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(80usize, "feFuncG", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFuncG {
    type Value = super::opcode::FeFuncG;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFuncG;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeFuncG(data.deserialize_field::<FeFunc>("feFuncG", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(80usize, "feFuncG", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFuncB {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(81usize, "feFuncB", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFuncB {
    type Value = super::opcode::FeFuncB;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFuncB;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeFuncB(data.deserialize_field::<FeFunc>("feFuncB", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(81usize, "feFuncB", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeComposite {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(82usize, "feComposite", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("operator"), &self.operator)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeComposite {
    type Value = super::opcode::FeComposite;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeComposite;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    FeComposite {
                        r#in: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                                "feComposite",
                                0usize,
                                Some("in"),
                            )?,
                        in2: data.deserialize_field::<mlang_rs::rt::opcode::Variable<FeIn>>(
                            "feComposite",
                            1usize,
                            Some("in2"),
                        )?,
                        operator: data.deserialize_field::<Option<
                            mlang_rs::rt::opcode::Variable<FeCompositeOperator>,
                        >>(
                            "feComposite", 2usize, Some("operator")
                        )?,
                        x: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feComposite",
                                3usize,
                                Some("x"),
                            )?,
                        y: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feComposite",
                                4usize,
                                Some("y"),
                            )?,
                        width: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feComposite",
                                5usize,
                                Some("width"),
                            )?,
                        height: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feComposite",
                                6usize,
                                Some("height"),
                            )?,
                        result: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                                "feComposite",
                                7usize,
                                Some("result"),
                            )?,
                    };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(82usize, "feComposite", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeConvolveMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(83usize, "feConvolveMatrix", 15usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("order"), &self.order)?;
        serializer.serialize_field(2usize, Some("kernel"), &self.kernel)?;
        serializer.serialize_field(3usize, Some("divisor"), &self.divisor)?;
        serializer.serialize_field(4usize, Some("bias"), &self.bias)?;
        serializer.serialize_field(5usize, Some("targetX"), &self.target_x)?;
        serializer.serialize_field(6usize, Some("targetY"), &self.target_y)?;
        serializer.serialize_field(7usize, Some("edgeMode"), &self.edge_mode)?;
        serializer.serialize_field(8usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(9usize, Some("preserveAlpha"), &self.preserve_alpha)?;
        serializer.serialize_field(10usize, Some("x"), &self.x)?;
        serializer.serialize_field(11usize, Some("y"), &self.y)?;
        serializer.serialize_field(12usize, Some("width"), &self.width)?;
        serializer.serialize_field(13usize, Some("height"), &self.height)?;
        serializer.serialize_field(14usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeConvolveMatrix {
    type Value = super::opcode::FeConvolveMatrix;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeConvolveMatrix;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeConvolveMatrix { r#in : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FeIn > > > ("feConvolveMatrix" , 0usize , Some ("in")) ? , order : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < NumberOptNumber > > > ("feConvolveMatrix" , 1usize , Some ("order")) ? , kernel : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < Vec < f32 > > > ("feConvolveMatrix" , 2usize , Some ("kernel")) ? , divisor : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < f32 > > > ("feConvolveMatrix" , 3usize , Some ("divisor")) ? , bias : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < f32 > > > ("feConvolveMatrix" , 4usize , Some ("bias")) ? , target_x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < i32 > > > ("feConvolveMatrix" , 5usize , Some ("targetX")) ? , target_y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < i32 > > > ("feConvolveMatrix" , 6usize , Some ("targetY")) ? , edge_mode : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < FeConvolveMatrixEdgeMode > > ("feConvolveMatrix" , 7usize , Some ("edgeMode")) ? , kernel_unit_len : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < NumberOptNumber > > > ("feConvolveMatrix" , 8usize , Some ("kernelUnitLen")) ? , preserve_alpha : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < bool > > ("feConvolveMatrix" , 9usize , Some ("preserveAlpha")) ? , x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feConvolveMatrix" , 10usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feConvolveMatrix" , 11usize , Some ("y")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feConvolveMatrix" , 12usize , Some ("width")) ? , height : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feConvolveMatrix" , 13usize , Some ("height")) ? , result : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < String > > > ("feConvolveMatrix" , 14usize , Some ("result")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(83usize, "feConvolveMatrix", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeDiffuseLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(84usize, "feDiffuseLighting", 9usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surfaceScale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("diffuseConstant"), &self.diffuse_constant)?;
        serializer.serialize_field(3usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(4usize, Some("x"), &self.x)?;
        serializer.serialize_field(5usize, Some("y"), &self.y)?;
        serializer.serialize_field(6usize, Some("width"), &self.width)?;
        serializer.serialize_field(7usize, Some("height"), &self.height)?;
        serializer.serialize_field(8usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeDiffuseLighting {
    type Value = super::opcode::FeDiffuseLighting;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeDiffuseLighting;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    FeDiffuseLighting {
                        r#in: data.deserialize_field::<mlang_rs::rt::opcode::Variable<FeIn>>(
                            "feDiffuseLighting",
                            0usize,
                            Some("in"),
                        )?,
                        surface_scale: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                                "feDiffuseLighting",
                                1usize,
                                Some("surfaceScale"),
                            )?,
                        diffuse_constant: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                                "feDiffuseLighting",
                                2usize,
                                Some("diffuseConstant"),
                            )?,
                        kernel_unit_len: data.deserialize_field::<Option<
                            mlang_rs::rt::opcode::Variable<NumberOptNumber>,
                        >>(
                            "feDiffuseLighting",
                            3usize,
                            Some("kernelUnitLen"),
                        )?,
                        x: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feDiffuseLighting",
                                4usize,
                                Some("x"),
                            )?,
                        y: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feDiffuseLighting",
                                5usize,
                                Some("y"),
                            )?,
                        width: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feDiffuseLighting",
                                6usize,
                                Some("width"),
                            )?,
                        height: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feDiffuseLighting",
                                7usize,
                                Some("height"),
                            )?,
                        result: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                                "feDiffuseLighting",
                                8usize,
                                Some("result"),
                            )?,
                    };
                Ok(value)
            }
        }
        deserializer.deserialize_element(84usize, "feDiffuseLighting", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeDisplacementMap {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(85usize, "feDisplacementMap", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("in2"), &self.in2)?;
        serializer.serialize_field(2usize, Some("scale"), &self.scale)?;
        serializer.serialize_field(3usize, Some("xChannelSelector"), &self.x_channel_selector)?;
        serializer.serialize_field(4usize, Some("yChannelSelector"), &self.y_channel_selector)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeDisplacementMap {
    type Value = super::opcode::FeDisplacementMap;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeDisplacementMap;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeDisplacementMap {
                    r#in: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feDisplacementMap",
                        0usize,
                        Some("in"),
                    )?,
                    in2: data.deserialize_field::<mlang_rs::rt::opcode::Variable<FeIn>>(
                        "feDisplacementMap",
                        1usize,
                        Some("in2"),
                    )?,
                    scale: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feDisplacementMap",
                        2usize,
                        Some("scale"),
                    )?,
                    x_channel_selector: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Channel>>>(
                            "feDisplacementMap",
                            3usize,
                            Some("xChannelSelector"),
                        )?,
                    y_channel_selector: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Channel>>>(
                            "feDisplacementMap",
                            4usize,
                            Some("yChannelSelector"),
                        )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feDisplacementMap",
                        5usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feDisplacementMap",
                        6usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feDisplacementMap",
                            7usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feDisplacementMap",
                            8usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feDisplacementMap",
                            9usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(85usize, "feDisplacementMap", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeFlood {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(86usize, "feFlood", 7usize)?;
        serializer.serialize_field(0usize, Some("color"), &self.color)?;
        serializer.serialize_field(1usize, Some("opacity"), &self.opacity)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeFlood {
    type Value = super::opcode::FeFlood;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeFlood;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeFlood {
                    color: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Color>>>(
                            "feFlood",
                            0usize,
                            Some("color"),
                        )?,
                    opacity: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "feFlood",
                            1usize,
                            Some("opacity"),
                        )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feFlood",
                        2usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feFlood",
                        3usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feFlood",
                            4usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feFlood",
                            5usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feFlood",
                            6usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(86usize, "feFlood", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeGaussianBlur {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(87usize, "feGaussianBlur", 7usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("stdDeviation"), &self.std_deviation)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeGaussianBlur {
    type Value = super::opcode::FeGaussianBlur;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeGaussianBlur;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    FeGaussianBlur {
                        r#in: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                                "feGaussianBlur",
                                0usize,
                                Some("in"),
                            )?,
                        std_deviation: data.deserialize_field::<Option<
                            mlang_rs::rt::opcode::Variable<NumberOptNumber>,
                        >>(
                            "feGaussianBlur", 1usize, Some("stdDeviation")
                        )?,
                        x: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feGaussianBlur",
                                2usize,
                                Some("x"),
                            )?,
                        y: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feGaussianBlur",
                                3usize,
                                Some("y"),
                            )?,
                        width: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feGaussianBlur",
                                4usize,
                                Some("width"),
                            )?,
                        height: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feGaussianBlur",
                                5usize,
                                Some("height"),
                            )?,
                        result: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                                "feGaussianBlur",
                                6usize,
                                Some("result"),
                            )?,
                    };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(87usize, "feGaussianBlur", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeMerge {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(88usize, "feMerge", 5usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeMerge {
    type Value = super::opcode::FeMerge;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeMerge;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeMerge {
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feMerge",
                        0usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feMerge",
                        1usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feMerge",
                            2usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feMerge",
                            3usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feMerge",
                            4usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_element(88usize, "feMerge", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeMergeNode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(89usize, "feMergeNode", 1usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeMergeNode {
    type Value = super::opcode::FeMergeNode;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeMergeNode;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeMergeNode(
                    data.deserialize_field::<mlang_rs::rt::opcode::Variable<FeIn>>(
                        "feMergeNode",
                        0usize,
                        Some("in"),
                    )?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(89usize, "feMergeNode", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeImage {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(90usize, "feImage", 7usize)?;
        serializer.serialize_field(0usize, Some("href"), &self.href)?;
        serializer.serialize_field(1usize, Some("aspect"), &self.aspect)?;
        serializer.serialize_field(2usize, Some("x"), &self.x)?;
        serializer.serialize_field(3usize, Some("y"), &self.y)?;
        serializer.serialize_field(4usize, Some("width"), &self.width)?;
        serializer.serialize_field(5usize, Some("height"), &self.height)?;
        serializer.serialize_field(6usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeImage {
    type Value = super::opcode::FeImage;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeImage;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeImage { href : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < FuncIri > > ("feImage" , 0usize , Some ("href")) ? , aspect : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < PreserveAspectRatio > > > ("feImage" , 1usize , Some ("aspect")) ? , x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feImage" , 2usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feImage" , 3usize , Some ("y")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feImage" , 4usize , Some ("width")) ? , height : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feImage" , 5usize , Some ("height")) ? , result : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < String > > > ("feImage" , 6usize , Some ("result")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(90usize, "feImage", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeMorphology {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(91usize, "feMorphology", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("mode"), &self.mode)?;
        serializer.serialize_field(2usize, Some("radius"), &self.radius)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeMorphology {
    type Value = super::opcode::FeMorphology;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeMorphology;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeMorphology { r#in : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FeIn > > > ("feMorphology" , 0usize , Some ("in")) ? , mode : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FeMorphologyOperator > > > ("feMorphology" , 1usize , Some ("mode")) ? , radius : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < NumberOptNumber > > > ("feMorphology" , 2usize , Some ("radius")) ? , x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feMorphology" , 3usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feMorphology" , 4usize , Some ("y")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feMorphology" , 5usize , Some ("width")) ? , height : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feMorphology" , 6usize , Some ("height")) ? , result : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < String > > > ("feMorphology" , 7usize , Some ("result")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(91usize, "feMorphology", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeOffset {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(92usize, "feOffset", 8usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(2usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeOffset {
    type Value = super::opcode::FeOffset;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeOffset;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeOffset {
                    r#in: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feOffset",
                        0usize,
                        Some("in"),
                    )?,
                    dx: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feOffset",
                        1usize,
                        Some("dx"),
                    )?,
                    dy: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                        "feOffset",
                        2usize,
                        Some("dy"),
                    )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feOffset",
                        3usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feOffset",
                        4usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feOffset",
                            5usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feOffset",
                            6usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feOffset",
                            7usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(92usize, "feOffset", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeSpecularLighting {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(93usize, "feSpecularLighting", 10usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("surfaceScale"), &self.surface_scale)?;
        serializer.serialize_field(2usize, Some("specularConstant"), &self.specular_constant)?;
        serializer.serialize_field(3usize, Some("specularExponent"), &self.specular_exponent)?;
        serializer.serialize_field(4usize, Some("kernelUnitLen"), &self.kernel_unit_len)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeSpecularLighting {
    type Value = super::opcode::FeSpecularLighting;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeSpecularLighting;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    FeSpecularLighting {
                        r#in: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                                "feSpecularLighting",
                                0usize,
                                Some("in"),
                            )?,
                        surface_scale: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                                "feSpecularLighting",
                                1usize,
                                Some("surfaceScale"),
                            )?,
                        specular_constant: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                                "feSpecularLighting",
                                2usize,
                                Some("specularConstant"),
                            )?,
                        specular_exponent: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                                "feSpecularLighting",
                                3usize,
                                Some("specularExponent"),
                            )?,
                        kernel_unit_len: data.deserialize_field::<Option<
                            mlang_rs::rt::opcode::Variable<NumberOptNumber>,
                        >>(
                            "feSpecularLighting",
                            4usize,
                            Some("kernelUnitLen"),
                        )?,
                        x: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feSpecularLighting",
                                5usize,
                                Some("x"),
                            )?,
                        y: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feSpecularLighting",
                                6usize,
                                Some("y"),
                            )?,
                        width: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feSpecularLighting",
                                7usize,
                                Some("width"),
                            )?,
                        height: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                                "feSpecularLighting",
                                8usize,
                                Some("height"),
                            )?,
                        result: data
                            .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                                "feSpecularLighting",
                                9usize,
                                Some("result"),
                            )?,
                    };
                Ok(value)
            }
        }
        deserializer.deserialize_element(93usize, "feSpecularLighting", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeTile {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(94usize, "feTile", 6usize)?;
        serializer.serialize_field(0usize, Some("in"), &self.r#in)?;
        serializer.serialize_field(1usize, Some("x"), &self.x)?;
        serializer.serialize_field(2usize, Some("y"), &self.y)?;
        serializer.serialize_field(3usize, Some("width"), &self.width)?;
        serializer.serialize_field(4usize, Some("height"), &self.height)?;
        serializer.serialize_field(5usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeTile {
    type Value = super::opcode::FeTile;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeTile;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeTile {
                    r#in: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<FeIn>>>(
                        "feTile",
                        0usize,
                        Some("in"),
                    )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feTile",
                        1usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "feTile",
                        2usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feTile",
                            3usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "feTile",
                            4usize,
                            Some("height"),
                        )?,
                    result: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<String>>>(
                            "feTile",
                            5usize,
                            Some("result"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(94usize, "feTile", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::FeTurbulence {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(95usize, "feTurbulence", 10usize)?;
        serializer.serialize_field(0usize, Some("baseFrequency"), &self.base_frequency)?;
        serializer.serialize_field(1usize, Some("numOctaves"), &self.num_octaves)?;
        serializer.serialize_field(2usize, Some("seed"), &self.seed)?;
        serializer.serialize_field(3usize, Some("stitchTiles"), &self.stitch_tiles)?;
        serializer.serialize_field(4usize, Some("type"), &self.r#type)?;
        serializer.serialize_field(5usize, Some("x"), &self.x)?;
        serializer.serialize_field(6usize, Some("y"), &self.y)?;
        serializer.serialize_field(7usize, Some("width"), &self.width)?;
        serializer.serialize_field(8usize, Some("height"), &self.height)?;
        serializer.serialize_field(9usize, Some("result"), &self.result)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::FeTurbulence {
    type Value = super::opcode::FeTurbulence;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::FeTurbulence;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = FeTurbulence { base_frequency : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < NumberOptNumber > > > ("feTurbulence" , 0usize , Some ("baseFrequency")) ? , num_octaves : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < i32 > > > ("feTurbulence" , 1usize , Some ("numOctaves")) ? , seed : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < f32 > > > ("feTurbulence" , 2usize , Some ("seed")) ? , stitch_tiles : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FeStitchTiles > > > ("feTurbulence" , 3usize , Some ("stitchTiles")) ? , r#type : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < FeTurbulenceType > > > ("feTurbulence" , 4usize , Some ("type")) ? , x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feTurbulence" , 5usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feTurbulence" , 6usize , Some ("y")) ? , width : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feTurbulence" , 7usize , Some ("width")) ? , height : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("feTurbulence" , 8usize , Some ("height")) ? , result : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < String > > > ("feTurbulence" , 9usize , Some ("result")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(95usize, "feTurbulence", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::LinearGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(96usize, "linearGradient", 8usize)?;
        serializer.serialize_field(0usize, Some("gradientUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("gradientTransform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(3usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(4usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(5usize, Some("y2"), &self.y2)?;
        serializer.serialize_field(6usize, Some("spread"), &self.spread)?;
        serializer.serialize_field(7usize, Some("xlink:href"), &self.href)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::LinearGradient {
    type Value = super::opcode::LinearGradient;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::LinearGradient;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = LinearGradient { units : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Coords > > > ("linearGradient" , 0usize , Some ("gradientUnits")) ? , transform : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Transform > > > > ("linearGradient" , 1usize , Some ("gradientTransform")) ? , x1 : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("linearGradient" , 2usize , Some ("x1")) ? , y1 : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("linearGradient" , 3usize , Some ("y1")) ? , x2 : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("linearGradient" , 4usize , Some ("x2")) ? , y2 : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("linearGradient" , 5usize , Some ("y2")) ? , spread : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < SpreadMethod > > > ("linearGradient" , 6usize , Some ("spread")) ? , href : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Iri > > > ("linearGradient" , 7usize , Some ("xlink:href")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(96usize, "linearGradient", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::RadialGradient {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(97usize, "radialGradient", 9usize)?;
        serializer.serialize_field(0usize, Some("gradientUnits"), &self.unit)?;
        serializer.serialize_field(1usize, Some("gradientTransform"), &self.transform)?;
        serializer.serialize_field(2usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(3usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(4usize, Some("r"), &self.r)?;
        serializer.serialize_field(5usize, Some("fx"), &self.fx)?;
        serializer.serialize_field(6usize, Some("fy"), &self.fy)?;
        serializer.serialize_field(7usize, Some("spread"), &self.spread)?;
        serializer.serialize_field(8usize, Some("xlink:href"), &self.href)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::RadialGradient {
    type Value = super::opcode::RadialGradient;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::RadialGradient;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = RadialGradient { unit : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Coords > > > ("radialGradient" , 0usize , Some ("gradientUnits")) ? , transform : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Transform > > > > ("radialGradient" , 1usize , Some ("gradientTransform")) ? , cx : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("radialGradient" , 2usize , Some ("cx")) ? , cy : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("radialGradient" , 3usize , Some ("cy")) ? , r : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("radialGradient" , 4usize , Some ("r")) ? , fx : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("radialGradient" , 5usize , Some ("fx")) ? , fy : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("radialGradient" , 6usize , Some ("fy")) ? , spread : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < SpreadMethod > > > ("radialGradient" , 7usize , Some ("spread")) ? , href : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Iri > > > ("radialGradient" , 8usize , Some ("xlink:href")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(97usize, "radialGradient", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::GradientStop {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(98usize, "stop", 3usize)?;
        serializer.serialize_field(0usize, Some("offset"), &self.offset)?;
        serializer.serialize_field(1usize, Some("stop-color"), &self.color)?;
        serializer.serialize_field(2usize, Some("stop-opacity"), &self.opacity)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::GradientStop {
    type Value = super::opcode::GradientStop;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::GradientStop;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = GradientStop {
                    offset: data.deserialize_field::<mlang_rs::rt::opcode::Variable<f32>>(
                        "stop",
                        0usize,
                        Some("offset"),
                    )?,
                    color: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Color>>>(
                            "stop",
                            1usize,
                            Some("stop-color"),
                        )?,
                    opacity: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<f32>>>(
                            "stop",
                            2usize,
                            Some("stop-opacity"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(98usize, "stop", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Group {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let serializer = serializer.serialize_el(99usize, "g", 0usize)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Group {
    type Value = super::opcode::Group;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Group;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Group;
                Ok(value)
            }
        }
        deserializer.deserialize_element(99usize, "g", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Path {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(100usize, "path", 2usize)?;
        serializer.serialize_field(0usize, Some("d"), &self.events)?;
        serializer.serialize_field(1usize, Some("length"), &self.length)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Path {
    type Value = super::opcode::Path;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Path;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Path {
                    events: data
                        .deserialize_field::<mlang_rs::rt::opcode::Variable<Vec<PathEvent>>>(
                            "path",
                            0usize,
                            Some("d"),
                        )?,
                    length: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "path",
                            1usize,
                            Some("length"),
                        )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(100usize, "path", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(101usize, "pattern", 8usize)?;
        serializer.serialize_field(0usize, Some("patternUnits"), &self.units)?;
        serializer.serialize_field(1usize, Some("contentUnits"), &self.content_units)?;
        serializer.serialize_field(2usize, Some("transform"), &self.transform)?;
        serializer.serialize_field(3usize, Some("x"), &self.x)?;
        serializer.serialize_field(4usize, Some("y"), &self.y)?;
        serializer.serialize_field(5usize, Some("width"), &self.width)?;
        serializer.serialize_field(6usize, Some("height"), &self.height)?;
        serializer.serialize_field(7usize, Some("xlink:href"), &self.href)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Pattern {
    type Value = super::opcode::Pattern;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Pattern;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Pattern {
                    units: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Coords>>>(
                            "pattern",
                            0usize,
                            Some("patternUnits"),
                        )?,
                    content_units: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Coords>>>(
                            "pattern",
                            1usize,
                            Some("contentUnits"),
                        )?,
                    transform: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Transform>>>(
                            "pattern",
                            2usize,
                            Some("transform"),
                        )?,
                    x: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "pattern",
                        3usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "pattern",
                        4usize,
                        Some("y"),
                    )?,
                    width: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "pattern",
                            5usize,
                            Some("width"),
                        )?,
                    height: data
                        .deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                            "pattern",
                            6usize,
                            Some("height"),
                        )?,
                    href: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Iri>>>(
                        "pattern",
                        7usize,
                        Some("xlink:href"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_element(101usize, "pattern", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Use {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(102usize, "use", 1usize)?;
        serializer.serialize_field(0usize, Some("xlink:href"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Use {
    type Value = super::opcode::Use;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Use;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Use(
                    data.deserialize_field::<mlang_rs::rt::opcode::Variable<Iri>>(
                        "use",
                        0usize,
                        Some("xlink:href"),
                    )?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(102usize, "use", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Rect {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(103usize, "rect", 6usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("width"), &self.width)?;
        serializer.serialize_field(3usize, Some("height"), &self.height)?;
        serializer.serialize_field(4usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(5usize, Some("ry"), &self.ry)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Rect {
    type Value = super::opcode::Rect;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Rect;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Rect {
                    x: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "rect",
                        0usize,
                        Some("x"),
                    )?,
                    y: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "rect",
                        1usize,
                        Some("y"),
                    )?,
                    width: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "rect",
                        2usize,
                        Some("width"),
                    )?,
                    height: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "rect",
                        3usize,
                        Some("height"),
                    )?,
                    rx: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "rect",
                        4usize,
                        Some("rx"),
                    )?,
                    ry: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "rect",
                        5usize,
                        Some("ry"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(103usize, "rect", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Circle {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(104usize, "circle", 3usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("r"), &self.r)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Circle {
    type Value = super::opcode::Circle;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Circle;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Circle {
                    cx: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "circle",
                        0usize,
                        Some("cx"),
                    )?,
                    cy: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "circle",
                        1usize,
                        Some("cy"),
                    )?,
                    r: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "circle",
                        2usize,
                        Some("r"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(104usize, "circle", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Ellipse {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(105usize, "ellipse", 4usize)?;
        serializer.serialize_field(0usize, Some("cx"), &self.cx)?;
        serializer.serialize_field(1usize, Some("cy"), &self.cy)?;
        serializer.serialize_field(2usize, Some("rx"), &self.rx)?;
        serializer.serialize_field(3usize, Some("ry"), &self.ry)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Ellipse {
    type Value = super::opcode::Ellipse;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Ellipse;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Ellipse {
                    cx: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "ellipse",
                        0usize,
                        Some("cx"),
                    )?,
                    cy: data.deserialize_field::<Option<mlang_rs::rt::opcode::Variable<Length>>>(
                        "ellipse",
                        1usize,
                        Some("cy"),
                    )?,
                    rx: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "ellipse",
                        2usize,
                        Some("rx"),
                    )?,
                    ry: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "ellipse",
                        3usize,
                        Some("ry"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(105usize, "ellipse", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Line {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(106usize, "line", 4usize)?;
        serializer.serialize_field(0usize, Some("x1"), &self.x1)?;
        serializer.serialize_field(1usize, Some("y1"), &self.y1)?;
        serializer.serialize_field(2usize, Some("x2"), &self.x2)?;
        serializer.serialize_field(3usize, Some("y2"), &self.y2)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Line {
    type Value = super::opcode::Line;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Line;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Line {
                    x1: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "line",
                        0usize,
                        Some("x1"),
                    )?,
                    y1: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "line",
                        1usize,
                        Some("y1"),
                    )?,
                    x2: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "line",
                        2usize,
                        Some("x2"),
                    )?,
                    y2: data.deserialize_field::<mlang_rs::rt::opcode::Variable<Length>>(
                        "line",
                        3usize,
                        Some("y2"),
                    )?,
                };
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(106usize, "line", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Polyline {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(107usize, "polyline", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Polyline {
    type Value = super::opcode::Polyline;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Polyline;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Polyline(
                    data.deserialize_field::<mlang_rs::rt::opcode::Variable<Vec<Point>>>(
                        "polyline",
                        0usize,
                        Some("points"),
                    )?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(107usize, "polyline", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Polygon {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(108usize, "polygon", 1usize)?;
        serializer.serialize_field(0usize, Some("points"), &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Polygon {
    type Value = super::opcode::Polygon;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Polygon;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Polygon(
                    data.deserialize_field::<mlang_rs::rt::opcode::Variable<Vec<Point>>>(
                        "polygon",
                        0usize,
                        Some("points"),
                    )?,
                );
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(108usize, "polygon", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Text {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(110usize, "text", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("textLength"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("lengthAdjust"), &self.length_adjust)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Text {
    type Value = super::opcode::Text;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Text;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = Text { x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("text" , 0usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("text" , 1usize , Some ("y")) ? , dx : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("text" , 2usize , Some ("dx")) ? , dy : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("text" , 3usize , Some ("dy")) ? , rotate : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Angle > > > > ("text" , 4usize , Some ("rotate")) ? , text_length : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("text" , 5usize , Some ("textLength")) ? , length_adjust : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextLengthAdjust > > > ("text" , 6usize , Some ("lengthAdjust")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(110usize, "text", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextSpan {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(111usize, "tspan", 7usize)?;
        serializer.serialize_field(0usize, Some("x"), &self.x)?;
        serializer.serialize_field(1usize, Some("y"), &self.y)?;
        serializer.serialize_field(2usize, Some("dx"), &self.dx)?;
        serializer.serialize_field(3usize, Some("dy"), &self.dy)?;
        serializer.serialize_field(4usize, Some("rotate"), &self.rotate)?;
        serializer.serialize_field(5usize, Some("textLength"), &self.text_length)?;
        serializer.serialize_field(6usize, Some("lengthAdjust"), &self.length_adjust)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextSpan {
    type Value = super::opcode::TextSpan;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextSpan;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = TextSpan { x : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("tspan" , 0usize , Some ("x")) ? , y : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("tspan" , 1usize , Some ("y")) ? , dx : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("tspan" , 2usize , Some ("dx")) ? , dy : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("tspan" , 3usize , Some ("dy")) ? , rotate : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Angle > > > > ("tspan" , 4usize , Some ("rotate")) ? , text_length : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Vec < Length > > > > ("tspan" , 5usize , Some ("textLength")) ? , length_adjust : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextLengthAdjust > > > ("tspan" , 6usize , Some ("lengthAdjust")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(111usize, "tspan", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Characters {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_leaf(112usize, "characters", 1usize)?;
        serializer.serialize_field(0usize, None, &self.0)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Characters {
    type Value = super::opcode::Characters;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Characters;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value =
                    Characters(data.deserialize_field::<String>("characters", 0usize, None)?);
                Ok(value)
            }
        }
        deserializer.deserialize_leaf(112usize, "characters", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::TextPath {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        use mlang_rs::rt::serde::ser::SerializeNode;
        let mut serializer = serializer.serialize_el(113usize, "textPath", 4usize)?;
        serializer.serialize_field(0usize, Some("startOffset"), &self.start_offset)?;
        serializer.serialize_field(1usize, Some("method"), &self.method)?;
        serializer.serialize_field(2usize, Some("spacing"), &self.spacing)?;
        serializer.serialize_field(3usize, Some("href"), &self.href)?;
        serializer.finish()
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::TextPath {
    type Value = super::opcode::TextPath;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::TextPath;
            #[allow(unused_mut)]
            fn visit_node<A>(self, mut data: A) -> Result<Self::Value, A::Error>
            where
                A: NodeAccess,
            {
                let _ = data;
                use super::opcode::*;
                let value = TextPath { start_offset : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < Length > > > ("textPath" , 0usize , Some ("startOffset")) ? , method : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextPathMethod > > > ("textPath" , 1usize , Some ("method")) ? , spacing : data . deserialize_field :: < Option < mlang_rs :: rt :: opcode :: Variable < TextPathSpacing > > > ("textPath" , 2usize , Some ("spacing")) ? , href : data . deserialize_field :: < mlang_rs :: rt :: opcode :: Variable < Iri > > ("textPath" , 3usize , Some ("href")) ? } ;
                Ok(value)
            }
        }
        deserializer.deserialize_element(113usize, "textPath", V)
    }
}
impl mlang_rs::rt::serde::ser::Serialize for super::opcode::Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: mlang_rs::rt::serde::ser::Serializer,
    {
        match self {
            Self::Apply(v) => match v {
                super::opcode::Attr::TextLayout(value) => value.serialize(serializer),
                super::opcode::Attr::WithTransform(value) => value.serialize(serializer),
                super::opcode::Attr::Id(value) => value.serialize(serializer),
                super::opcode::Attr::Fill(value) => value.serialize(serializer),
                super::opcode::Attr::Stroke(value) => value.serialize(serializer),
                super::opcode::Attr::Font(value) => value.serialize(serializer),
                super::opcode::Attr::EnableBackground(value) => value.serialize(serializer),
                super::opcode::Attr::WithFilter(value) => value.serialize(serializer),
                super::opcode::Attr::WithClipPath(value) => value.serialize(serializer),
                super::opcode::Attr::WithMask(value) => value.serialize(serializer),
                super::opcode::Attr::Opacity(value) => value.serialize(serializer),
                super::opcode::Attr::ViewBox(value) => value.serialize(serializer),
            },
            Self::Element(v) => match v {
                super::opcode::Element::Canvas(value) => value.serialize(serializer),
                super::opcode::Element::Mask(value) => value.serialize(serializer),
                super::opcode::Element::ClipPath(value) => value.serialize(serializer),
                super::opcode::Element::Filter(value) => value.serialize(serializer),
                super::opcode::Element::FeComponentTransfer(value) => value.serialize(serializer),
                super::opcode::Element::FeDiffuseLighting(value) => value.serialize(serializer),
                super::opcode::Element::FeMerge(value) => value.serialize(serializer),
                super::opcode::Element::FeSpecularLighting(value) => value.serialize(serializer),
                super::opcode::Element::LinearGradient(value) => value.serialize(serializer),
                super::opcode::Element::RadialGradient(value) => value.serialize(serializer),
                super::opcode::Element::Group(value) => value.serialize(serializer),
                super::opcode::Element::Pattern(value) => value.serialize(serializer),
                super::opcode::Element::Text(value) => value.serialize(serializer),
                super::opcode::Element::TextSpan(value) => value.serialize(serializer),
                super::opcode::Element::TextPath(value) => value.serialize(serializer),
            },
            Self::Leaf(v) => match v {
                super::opcode::Leaf::FeDistantLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FePointLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FeSpotLight(value) => value.serialize(serializer),
                super::opcode::Leaf::FeBlend(value) => value.serialize(serializer),
                super::opcode::Leaf::FeColorMatrix(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncA(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncR(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncG(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFuncB(value) => value.serialize(serializer),
                super::opcode::Leaf::FeComposite(value) => value.serialize(serializer),
                super::opcode::Leaf::FeConvolveMatrix(value) => value.serialize(serializer),
                super::opcode::Leaf::FeDisplacementMap(value) => value.serialize(serializer),
                super::opcode::Leaf::FeFlood(value) => value.serialize(serializer),
                super::opcode::Leaf::FeGaussianBlur(value) => value.serialize(serializer),
                super::opcode::Leaf::FeMergeNode(value) => value.serialize(serializer),
                super::opcode::Leaf::FeImage(value) => value.serialize(serializer),
                super::opcode::Leaf::FeMorphology(value) => value.serialize(serializer),
                super::opcode::Leaf::FeOffset(value) => value.serialize(serializer),
                super::opcode::Leaf::FeTile(value) => value.serialize(serializer),
                super::opcode::Leaf::FeTurbulence(value) => value.serialize(serializer),
                super::opcode::Leaf::GradientStop(value) => value.serialize(serializer),
                super::opcode::Leaf::Path(value) => value.serialize(serializer),
                super::opcode::Leaf::Use(value) => value.serialize(serializer),
                super::opcode::Leaf::Rect(value) => value.serialize(serializer),
                super::opcode::Leaf::Circle(value) => value.serialize(serializer),
                super::opcode::Leaf::Ellipse(value) => value.serialize(serializer),
                super::opcode::Leaf::Line(value) => value.serialize(serializer),
                super::opcode::Leaf::Polyline(value) => value.serialize(serializer),
                super::opcode::Leaf::Polygon(value) => value.serialize(serializer),
                super::opcode::Leaf::Characters(value) => value.serialize(serializer),
            },
            Self::Pop => serializer.serialize_pop(),
        }
    }
}
impl mlang_rs::rt::serde::de::Deserialize for super::opcode::Opcode {
    type Value = Option<Vec<super::opcode::Opcode>>;
    fn deserialize<D>(deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: mlang_rs::rt::serde::de::Deserializer,
    {
        use mlang_rs::rt::serde::de::*;
        let _ = deserializer;
        struct V;
        impl Visitor for V {
            type Value = super::opcode::Opcode;
            fn is_element(&self, name: &str) -> bool {
                match name {
                    "svg"
                    | "mask"
                    | "clipPath"
                    | "filter"
                    | "feComponentTransfer"
                    | "feDiffuseLighting"
                    | "feMerge"
                    | "feSpecularLighting"
                    | "linearGradient"
                    | "radialGradient"
                    | "g"
                    | "pattern"
                    | "text"
                    | "tspan"
                    | "textPath" => true,
                    _ => false,
                }
            }
            fn is_leaf(&self, name: &str) -> bool {
                match name {
                    "feDistantLight" | "fePointLight" | "feSpotLight" | "feBlend"
                    | "feColorMatrix" | "feFuncA" | "feFuncR" | "feFuncG" | "feFuncB"
                    | "feComposite" | "feConvolveMatrix" | "feDisplacementMap" | "feFlood"
                    | "feGaussianBlur" | "feMergeNode" | "feImage" | "feMorphology"
                    | "feOffset" | "feTile" | "feTurbulence" | "stop" | "path" | "use" | "rect"
                    | "circle" | "ellipse" | "line" | "polyline" | "polygon" | "characters" => true,
                    _ => false,
                }
            }
            fn visit_opcode<D>(
                self,
                type_id: usize,
                deserializer: D,
            ) -> Result<Self::Value, D::Error>
            where
                D: Deserializer,
            {
                use crate::opcode::*;
                match type_id {
                    48usize => Ok(Opcode::from(Attr::from(TextLayout::deserialize(
                        deserializer,
                    )?))),
                    49usize => Ok(Opcode::from(Attr::from(WithTransform::deserialize(
                        deserializer,
                    )?))),
                    50usize => Ok(Opcode::from(Attr::from(Id::deserialize(deserializer)?))),
                    51usize => Ok(Opcode::from(Attr::from(Fill::deserialize(deserializer)?))),
                    52usize => Ok(Opcode::from(Attr::from(Stroke::deserialize(deserializer)?))),
                    53usize => Ok(Opcode::from(Attr::from(Font::deserialize(deserializer)?))),
                    54usize => Ok(Opcode::from(Attr::from(EnableBackground::deserialize(
                        deserializer,
                    )?))),
                    55usize => Ok(Opcode::from(Attr::from(WithFilter::deserialize(
                        deserializer,
                    )?))),
                    56usize => Ok(Opcode::from(Attr::from(WithClipPath::deserialize(
                        deserializer,
                    )?))),
                    57usize => Ok(Opcode::from(Attr::from(WithMask::deserialize(
                        deserializer,
                    )?))),
                    58usize => Ok(Opcode::from(Attr::from(Opacity::deserialize(
                        deserializer,
                    )?))),
                    59usize => Ok(Opcode::from(Attr::from(ViewBox::deserialize(
                        deserializer,
                    )?))),
                    61usize => Ok(Opcode::from(Element::from(Canvas::deserialize(
                        deserializer,
                    )?))),
                    62usize => Ok(Opcode::from(Element::from(Mask::deserialize(
                        deserializer,
                    )?))),
                    63usize => Ok(Opcode::from(Element::from(ClipPath::deserialize(
                        deserializer,
                    )?))),
                    64usize => Ok(Opcode::from(Element::from(Filter::deserialize(
                        deserializer,
                    )?))),
                    65usize => Ok(Opcode::from(Leaf::from(FeDistantLight::deserialize(
                        deserializer,
                    )?))),
                    66usize => Ok(Opcode::from(Leaf::from(FePointLight::deserialize(
                        deserializer,
                    )?))),
                    67usize => Ok(Opcode::from(Leaf::from(FeSpotLight::deserialize(
                        deserializer,
                    )?))),
                    68usize => Ok(Opcode::from(Leaf::from(FeBlend::deserialize(
                        deserializer,
                    )?))),
                    70usize => Ok(Opcode::from(Leaf::from(FeColorMatrix::deserialize(
                        deserializer,
                    )?))),
                    77usize => Ok(Opcode::from(Element::from(
                        FeComponentTransfer::deserialize(deserializer)?,
                    ))),
                    78usize => Ok(Opcode::from(Leaf::from(FeFuncA::deserialize(
                        deserializer,
                    )?))),
                    79usize => Ok(Opcode::from(Leaf::from(FeFuncR::deserialize(
                        deserializer,
                    )?))),
                    80usize => Ok(Opcode::from(Leaf::from(FeFuncG::deserialize(
                        deserializer,
                    )?))),
                    81usize => Ok(Opcode::from(Leaf::from(FeFuncB::deserialize(
                        deserializer,
                    )?))),
                    82usize => Ok(Opcode::from(Leaf::from(FeComposite::deserialize(
                        deserializer,
                    )?))),
                    83usize => Ok(Opcode::from(Leaf::from(FeConvolveMatrix::deserialize(
                        deserializer,
                    )?))),
                    84usize => Ok(Opcode::from(Element::from(FeDiffuseLighting::deserialize(
                        deserializer,
                    )?))),
                    85usize => Ok(Opcode::from(Leaf::from(FeDisplacementMap::deserialize(
                        deserializer,
                    )?))),
                    86usize => Ok(Opcode::from(Leaf::from(FeFlood::deserialize(
                        deserializer,
                    )?))),
                    87usize => Ok(Opcode::from(Leaf::from(FeGaussianBlur::deserialize(
                        deserializer,
                    )?))),
                    88usize => Ok(Opcode::from(Element::from(FeMerge::deserialize(
                        deserializer,
                    )?))),
                    89usize => Ok(Opcode::from(Leaf::from(FeMergeNode::deserialize(
                        deserializer,
                    )?))),
                    90usize => Ok(Opcode::from(Leaf::from(FeImage::deserialize(
                        deserializer,
                    )?))),
                    91usize => Ok(Opcode::from(Leaf::from(FeMorphology::deserialize(
                        deserializer,
                    )?))),
                    92usize => Ok(Opcode::from(Leaf::from(FeOffset::deserialize(
                        deserializer,
                    )?))),
                    93usize => Ok(Opcode::from(Element::from(
                        FeSpecularLighting::deserialize(deserializer)?,
                    ))),
                    94usize => Ok(Opcode::from(Leaf::from(FeTile::deserialize(deserializer)?))),
                    95usize => Ok(Opcode::from(Leaf::from(FeTurbulence::deserialize(
                        deserializer,
                    )?))),
                    96usize => Ok(Opcode::from(Element::from(LinearGradient::deserialize(
                        deserializer,
                    )?))),
                    97usize => Ok(Opcode::from(Element::from(RadialGradient::deserialize(
                        deserializer,
                    )?))),
                    98usize => Ok(Opcode::from(Leaf::from(GradientStop::deserialize(
                        deserializer,
                    )?))),
                    99usize => Ok(Opcode::from(Element::from(Group::deserialize(
                        deserializer,
                    )?))),
                    100usize => Ok(Opcode::from(Leaf::from(Path::deserialize(deserializer)?))),
                    101usize => Ok(Opcode::from(Element::from(Pattern::deserialize(
                        deserializer,
                    )?))),
                    102usize => Ok(Opcode::from(Leaf::from(Use::deserialize(deserializer)?))),
                    103usize => Ok(Opcode::from(Leaf::from(Rect::deserialize(deserializer)?))),
                    104usize => Ok(Opcode::from(Leaf::from(Circle::deserialize(deserializer)?))),
                    105usize => Ok(Opcode::from(Leaf::from(Ellipse::deserialize(
                        deserializer,
                    )?))),
                    106usize => Ok(Opcode::from(Leaf::from(Line::deserialize(deserializer)?))),
                    107usize => Ok(Opcode::from(Leaf::from(Polyline::deserialize(
                        deserializer,
                    )?))),
                    108usize => Ok(Opcode::from(Leaf::from(Polygon::deserialize(
                        deserializer,
                    )?))),
                    110usize => Ok(Opcode::from(Element::from(Text::deserialize(
                        deserializer,
                    )?))),
                    111usize => Ok(Opcode::from(Element::from(TextSpan::deserialize(
                        deserializer,
                    )?))),
                    112usize => Ok(Opcode::from(Leaf::from(Characters::deserialize(
                        deserializer,
                    )?))),
                    113usize => Ok(Opcode::from(Element::from(TextPath::deserialize(
                        deserializer,
                    )?))),
                    _ => {
                        return Err(mlang_rs::rt::serde::de::Error::UnknownType(type_id).into());
                    }
                }
            }
            fn visit_opcode_with<D>(
                self,
                name: &str,
                deserializer: D,
            ) -> Result<Self::Value, D::Error>
            where
                D: Deserializer,
            {
                use crate::opcode::*;
                match name {
                    "textLayout" => Ok(Opcode::from(Attr::from(TextLayout::deserialize(
                        deserializer,
                    )?))),
                    "withTransform" => Ok(Opcode::from(Attr::from(WithTransform::deserialize(
                        deserializer,
                    )?))),
                    "id" => Ok(Opcode::from(Attr::from(Id::deserialize(deserializer)?))),
                    "fill" => Ok(Opcode::from(Attr::from(Fill::deserialize(deserializer)?))),
                    "stroke" => Ok(Opcode::from(Attr::from(Stroke::deserialize(deserializer)?))),
                    "font" => Ok(Opcode::from(Attr::from(Font::deserialize(deserializer)?))),
                    "enableBackground" => Ok(Opcode::from(Attr::from(
                        EnableBackground::deserialize(deserializer)?,
                    ))),
                    "withFilter" => Ok(Opcode::from(Attr::from(WithFilter::deserialize(
                        deserializer,
                    )?))),
                    "withClipPath" => Ok(Opcode::from(Attr::from(WithClipPath::deserialize(
                        deserializer,
                    )?))),
                    "withMask" => Ok(Opcode::from(Attr::from(WithMask::deserialize(
                        deserializer,
                    )?))),
                    "opacity" => Ok(Opcode::from(Attr::from(Opacity::deserialize(
                        deserializer,
                    )?))),
                    "viewBox" => Ok(Opcode::from(Attr::from(ViewBox::deserialize(
                        deserializer,
                    )?))),
                    "svg" => Ok(Opcode::from(Element::from(Canvas::deserialize(
                        deserializer,
                    )?))),
                    "mask" => Ok(Opcode::from(Element::from(Mask::deserialize(
                        deserializer,
                    )?))),
                    "clipPath" => Ok(Opcode::from(Element::from(ClipPath::deserialize(
                        deserializer,
                    )?))),
                    "filter" => Ok(Opcode::from(Element::from(Filter::deserialize(
                        deserializer,
                    )?))),
                    "feDistantLight" => Ok(Opcode::from(Leaf::from(FeDistantLight::deserialize(
                        deserializer,
                    )?))),
                    "fePointLight" => Ok(Opcode::from(Leaf::from(FePointLight::deserialize(
                        deserializer,
                    )?))),
                    "feSpotLight" => Ok(Opcode::from(Leaf::from(FeSpotLight::deserialize(
                        deserializer,
                    )?))),
                    "feBlend" => Ok(Opcode::from(Leaf::from(FeBlend::deserialize(
                        deserializer,
                    )?))),
                    "feColorMatrix" => Ok(Opcode::from(Leaf::from(FeColorMatrix::deserialize(
                        deserializer,
                    )?))),
                    "feComponentTransfer" => Ok(Opcode::from(Element::from(
                        FeComponentTransfer::deserialize(deserializer)?,
                    ))),
                    "feFuncA" => Ok(Opcode::from(Leaf::from(FeFuncA::deserialize(
                        deserializer,
                    )?))),
                    "feFuncR" => Ok(Opcode::from(Leaf::from(FeFuncR::deserialize(
                        deserializer,
                    )?))),
                    "feFuncG" => Ok(Opcode::from(Leaf::from(FeFuncG::deserialize(
                        deserializer,
                    )?))),
                    "feFuncB" => Ok(Opcode::from(Leaf::from(FeFuncB::deserialize(
                        deserializer,
                    )?))),
                    "feComposite" => Ok(Opcode::from(Leaf::from(FeComposite::deserialize(
                        deserializer,
                    )?))),
                    "feConvolveMatrix" => Ok(Opcode::from(Leaf::from(
                        FeConvolveMatrix::deserialize(deserializer)?,
                    ))),
                    "feDiffuseLighting" => Ok(Opcode::from(Element::from(
                        FeDiffuseLighting::deserialize(deserializer)?,
                    ))),
                    "feDisplacementMap" => Ok(Opcode::from(Leaf::from(
                        FeDisplacementMap::deserialize(deserializer)?,
                    ))),
                    "feFlood" => Ok(Opcode::from(Leaf::from(FeFlood::deserialize(
                        deserializer,
                    )?))),
                    "feGaussianBlur" => Ok(Opcode::from(Leaf::from(FeGaussianBlur::deserialize(
                        deserializer,
                    )?))),
                    "feMerge" => Ok(Opcode::from(Element::from(FeMerge::deserialize(
                        deserializer,
                    )?))),
                    "feMergeNode" => Ok(Opcode::from(Leaf::from(FeMergeNode::deserialize(
                        deserializer,
                    )?))),
                    "feImage" => Ok(Opcode::from(Leaf::from(FeImage::deserialize(
                        deserializer,
                    )?))),
                    "feMorphology" => Ok(Opcode::from(Leaf::from(FeMorphology::deserialize(
                        deserializer,
                    )?))),
                    "feOffset" => Ok(Opcode::from(Leaf::from(FeOffset::deserialize(
                        deserializer,
                    )?))),
                    "feSpecularLighting" => Ok(Opcode::from(Element::from(
                        FeSpecularLighting::deserialize(deserializer)?,
                    ))),
                    "feTile" => Ok(Opcode::from(Leaf::from(FeTile::deserialize(deserializer)?))),
                    "feTurbulence" => Ok(Opcode::from(Leaf::from(FeTurbulence::deserialize(
                        deserializer,
                    )?))),
                    "linearGradient" => Ok(Opcode::from(Element::from(
                        LinearGradient::deserialize(deserializer)?,
                    ))),
                    "radialGradient" => Ok(Opcode::from(Element::from(
                        RadialGradient::deserialize(deserializer)?,
                    ))),
                    "stop" => Ok(Opcode::from(Leaf::from(GradientStop::deserialize(
                        deserializer,
                    )?))),
                    "g" => Ok(Opcode::from(Element::from(Group::deserialize(
                        deserializer,
                    )?))),
                    "path" => Ok(Opcode::from(Leaf::from(Path::deserialize(deserializer)?))),
                    "pattern" => Ok(Opcode::from(Element::from(Pattern::deserialize(
                        deserializer,
                    )?))),
                    "use" => Ok(Opcode::from(Leaf::from(Use::deserialize(deserializer)?))),
                    "rect" => Ok(Opcode::from(Leaf::from(Rect::deserialize(deserializer)?))),
                    "circle" => Ok(Opcode::from(Leaf::from(Circle::deserialize(deserializer)?))),
                    "ellipse" => Ok(Opcode::from(Leaf::from(Ellipse::deserialize(
                        deserializer,
                    )?))),
                    "line" => Ok(Opcode::from(Leaf::from(Line::deserialize(deserializer)?))),
                    "polyline" => Ok(Opcode::from(Leaf::from(Polyline::deserialize(
                        deserializer,
                    )?))),
                    "polygon" => Ok(Opcode::from(Leaf::from(Polygon::deserialize(
                        deserializer,
                    )?))),
                    "text" => Ok(Opcode::from(Element::from(Text::deserialize(
                        deserializer,
                    )?))),
                    "tspan" => Ok(Opcode::from(Element::from(TextSpan::deserialize(
                        deserializer,
                    )?))),
                    "characters" => Ok(Opcode::from(Leaf::from(Characters::deserialize(
                        deserializer,
                    )?))),
                    "textPath" => Ok(Opcode::from(Element::from(TextPath::deserialize(
                        deserializer,
                    )?))),
                    _ => {
                        return Err(mlang_rs::rt::serde::de::Error::UnknownTypeName(
                            name.to_string(),
                        )
                        .into());
                    }
                }
            }
            fn visit_opcode_with_attrs<D>(
                self,
                name: &str,
                mut deserializer: D,
            ) -> Result<Vec<Self::Value>, D::Error>
            where
                D: AttrsNodeAccess,
            {
                let _ = name;
                let _ = deserializer;
                let mut attrs = std::collections::HashSet::new();
                for attr_name in deserializer.attrs() {
                    match name {
                        "svg" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "viewBox" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "minx" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "miny" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "width" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "height" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "aspect" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "circle" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "ellipse" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "filter" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "g" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "line" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "linearGradient" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            _ => {}
                        },
                        "mask" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "path" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "pattern" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "viewBox" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "minx" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "miny" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "width" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "height" => {
                                attrs.insert("viewBox");
                            }
                            #[allow(unreachable_patterns)]
                            "aspect" => {
                                attrs.insert("viewBox");
                            }
                            _ => {}
                        },
                        "polygon" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "polyline" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "radialGradient" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            _ => {}
                        },
                        "rect" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        "text" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "textLayout" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "writeMode" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "direction" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "unicodeBidi" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "text-anchor" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "dominantBaseline" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "alignmentBaseline" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "baselineShift" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "text-decoration" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "letterSpacing" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "wordSpacing" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            _ => {}
                        },
                        "tspan" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "id" => {
                                attrs.insert("id");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "textLayout" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "writeMode" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "direction" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "unicodeBidi" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "text-anchor" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "dominantBaseline" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "alignmentBaseline" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "baselineShift" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "text-decoration" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "letterSpacing" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "wordSpacing" => {
                                attrs.insert("textLayout");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            _ => {}
                        },
                        "use" => match attr_name {
                            #[allow(unreachable_patterns)]
                            "enableBackground" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "enable-background" => {
                                attrs.insert("enableBackground");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-rule" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "fill-opacity" => {
                                attrs.insert("fill");
                            }
                            #[allow(unreachable_patterns)]
                            "font" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-family" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-style" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-variant" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-weight" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-size" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "font-stretch" => {
                                attrs.insert("font");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "opacity" => {
                                attrs.insert("opacity");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-width" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linecap" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-linejoin" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dasharray" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-dashoffset" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "stroke-opacity" => {
                                attrs.insert("stroke");
                            }
                            #[allow(unreachable_patterns)]
                            "withClipPath" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "clip-path" => {
                                attrs.insert("withClipPath");
                            }
                            #[allow(unreachable_patterns)]
                            "withFilter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "filter" => {
                                attrs.insert("withFilter");
                            }
                            #[allow(unreachable_patterns)]
                            "withMask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "mask" => {
                                attrs.insert("withMask");
                            }
                            #[allow(unreachable_patterns)]
                            "withTransform" => {
                                attrs.insert("withTransform");
                            }
                            #[allow(unreachable_patterns)]
                            "transform" => {
                                attrs.insert("withTransform");
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                let mut opcodes = vec![];
                for attr in attrs {
                    opcodes.push(deserializer.deserialize_attr(attr, Self)?);
                }
                opcodes.push(deserializer.deserialize_attr(name, Self)?);
                Ok(opcodes)
            }
            fn visit_pop<E>(self) -> Result<Self::Value, E>
            where
                E: From<Error>,
            {
                Ok(Self::Value::Pop)
            }
        }
        deserializer.deserialize_opcode(V)
    }
}
