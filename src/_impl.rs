use crate::{PortMode, SioConfig};

pub enum TagOrContentField {
    Tag,
    Content,
}

/// Not public API.
pub struct TagOrContentFieldVisitor {
    pub tag: &'static str,
    pub content: &'static str,
}

impl<'de> serde::de::DeserializeSeed<'de> for TagOrContentFieldVisitor {
    type Value = TagOrContentField;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de> serde::de::Visitor<'de> for TagOrContentFieldVisitor {
    type Value = TagOrContentField;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "{:?} or {:?}", self.tag, self.content)
    }

    fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match field_index {
            0 => Ok(TagOrContentField::Tag),
            1 => Ok(TagOrContentField::Content),
            _ => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(field_index),
                &self,
            )),
        }
    }

    fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if field == self.tag {
            Ok(TagOrContentField::Tag)
        } else if field == self.content {
            Ok(TagOrContentField::Content)
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(field),
                &self,
            ))
        }
    }

    fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if field == self.tag.as_bytes() {
            Ok(TagOrContentField::Tag)
        } else if field == self.content.as_bytes() {
            Ok(TagOrContentField::Content)
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(field),
                &self,
            ))
        }
    }
}

pub enum TagContentOtherField {
    Tag,
    Content,
    Other,
}

/// Not public API.
pub struct TagContentOtherFieldVisitor {
    pub tag: &'static str,
    pub content: &'static str,
}

impl<'de> serde::de::DeserializeSeed<'de> for TagContentOtherFieldVisitor {
    type Value = TagContentOtherField;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(self)
    }
}

impl<'de> serde::de::Visitor<'de> for TagContentOtherFieldVisitor {
    type Value = TagContentOtherField;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            formatter,
            "{:?}, {:?}, or other ignored fields",
            self.tag, self.content
        )
    }

    fn visit_u64<E>(self, field_index: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match field_index {
            0 => Ok(TagContentOtherField::Tag),
            1 => Ok(TagContentOtherField::Content),
            _ => Ok(TagContentOtherField::Other),
        }
    }

    fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_bytes(field.as_bytes())
    }

    fn visit_bytes<E>(self, field: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if field == self.tag.as_bytes() {
            Ok(TagContentOtherField::Tag)
        } else if field == self.content.as_bytes() {
            Ok(TagContentOtherField::Content)
        } else {
            Ok(TagContentOtherField::Other)
        }
    }
}

impl<'de> serde::Deserialize<'de> for PortMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        enum Field {
            inactive,
            sio,
        }
        #[doc(hidden)]
        struct FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = Field;
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "variant identifier")
            }
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    0u64 => Ok(Field::inactive),
                    1u64 => Ok(Field::sio),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value),
                        &"variant index 0 <= i < 2",
                    )),
                }
            }
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "inactive" => Ok(Field::inactive),
                    "sio" => Ok(Field::sio),
                    _ => Err(serde::de::Error::unknown_variant(value, VARIANTS)),
                }
            }
            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    b"inactive" => Ok(Field::inactive),
                    b"sio" => Ok(Field::sio),
                    _ => {
                        let value = core::str::from_utf8(value).unwrap_or_default();
                        Err(serde::de::Error::unknown_variant(value, VARIANTS))
                    }
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }
        #[doc(hidden)]
        const VARIANTS: &[&str] = &["inactive", "sio"];
        #[doc(hidden)]
        struct Seed<'de> {
            field: Field,
            marker: core::marker::PhantomData<PortMode>,
            lifetime: core::marker::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::DeserializeSeed<'de> for Seed<'de> {
            type Value = PortMode;
            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match self.field {
                    Field::inactive => Ok(PortMode::Inactive),
                    Field::sio => Result::map(
                        <SioConfig as serde::Deserialize>::deserialize(deserializer),
                        PortMode::Sio,
                    ),
                }
            }
        }

        #[doc(hidden)]
        struct Visitor<'de> {
            marker: core::marker::PhantomData<PortMode>,
            lifetime: core::marker::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for Visitor<'de> {
            type Value = PortMode;
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "adjacently tagged enum PortMode")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                match {
                    let mut rk: Option<TagOrContentField> = None;
                    while let Some(key) = map.next_key_seed(TagContentOtherFieldVisitor {
                        tag: "mode",
                        content: "config",
                    })? {
                        match key {
                            TagContentOtherField::Other => {
                                let _: serde::de::IgnoredAny = map.next_value()?;
                                continue;
                            }
                            TagContentOtherField::Tag => {
                                rk = Some(TagOrContentField::Tag);
                                break;
                            }
                            TagContentOtherField::Content => {
                                rk = Some(TagOrContentField::Content);
                                break;
                            }
                        }
                    }
                    rk
                } {
                    Some(TagOrContentField::Tag) => {
                        let field = map.next_value()?;
                        match {
                            let mut rk: Option<TagOrContentField> = None;
                            while let Some(k) = map.next_key_seed(TagContentOtherFieldVisitor {
                                tag: "mode",
                                content: "config",
                            })? {
                                match k {
                                    TagContentOtherField::Other => {
                                        let _: serde::de::IgnoredAny = map.next_value()?;
                                        continue;
                                    }
                                    TagContentOtherField::Tag => {
                                        rk = Some(TagOrContentField::Tag);
                                        break;
                                    }
                                    TagContentOtherField::Content => {
                                        rk = Some(TagOrContentField::Content);
                                        break;
                                    }
                                }
                            }
                            rk
                        } {
                            Some(TagOrContentField::Tag) => {
                                Err(<A::Error as serde::de::Error>::duplicate_field("mode"))
                            }
                            Some(TagOrContentField::Content) => {
                                let ret = map.next_value_seed(Seed {
                                    field,
                                    marker: core::marker::PhantomData,
                                    lifetime: core::marker::PhantomData,
                                })?;
                                match {
                                    let mut rk: Option<TagOrContentField> = None;
                                    while let Some(key) =
                                        map.next_key_seed(TagContentOtherFieldVisitor {
                                            tag: "mode",
                                            content: "config",
                                        })?
                                    {
                                        match key {
                                            TagContentOtherField::Other => {
                                                let _: serde::de::IgnoredAny = map.next_value()?;
                                                continue;
                                            }
                                            TagContentOtherField::Tag => {
                                                rk = Some(TagOrContentField::Tag);
                                                break;
                                            }
                                            TagContentOtherField::Content => {
                                                rk = Some(TagOrContentField::Content);
                                                break;
                                            }
                                        }
                                    }
                                    rk
                                } {
                                    Some(TagOrContentField::Tag) => {
                                        Err(<A::Error as serde::de::Error>::duplicate_field("mode"))
                                    }
                                    Some(TagOrContentField::Content) => Err(
                                        <A::Error as serde::de::Error>::duplicate_field("config"),
                                    ),
                                    None => Ok(ret),
                                }
                            }
                            None => match field {
                                Field::inactive => Ok(PortMode::Inactive),
                                Field::sio => {
                                    Err(<A::Error as serde::de::Error>::missing_field("config"))
                                }
                            },
                        }
                    }
                    Some(TagOrContentField::Content) => Err(
                        <A::Error as serde::de::Error>::custom("expected mode before config"),
                    ),
                    None => Err(<A::Error as serde::de::Error>::missing_field("mode")),
                }
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                match seq.next_element()? {
                    Some(field) => {
                        match seq.next_element_seed(Seed {
                            field,
                            marker: core::marker::PhantomData,
                            lifetime: core::marker::PhantomData,
                        })? {
                            Some(ret) => Ok(ret),
                            None => Err(serde::de::Error::invalid_length(1, &self)),
                        }
                    }
                    None => Err(serde::de::Error::invalid_length(0, &self)),
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &[&str] = &["mode", "config"];
        serde::Deserializer::deserialize_struct(
            deserializer,
            "PortMode",
            FIELDS,
            Visitor {
                marker: core::marker::PhantomData::<PortMode>,
                lifetime: core::marker::PhantomData,
            },
        )
    }
}
