#![allow(dead_code, non_upper_case_globals)]
/// pub trait + proc derive `Deserialize`
pub const Deserialize: crate_paths::Path = crate_paths::Path::new("serde::Deserialize");

/// pub trait `Deserializer`
pub const Deserializer: crate_paths::Path = crate_paths::Path::new("serde::Deserializer");

/// pub trait + proc derive `Serialize`
pub const Serialize: crate_paths::Path = crate_paths::Path::new("serde::Serialize");

/// pub trait `Serializer`
pub const Serializer: crate_paths::Path = crate_paths::Path::new("serde::Serializer");

/// pub macro `forward_to_deserialize_any`
pub const forward_to_deserialize_any: crate_paths::Path =
    crate_paths::Path::new("serde::forward_to_deserialize_any");
pub mod de {
    /// pub trait `Deserialize`
    pub const Deserialize: crate_paths::Path = crate_paths::Path::new("serde::de::Deserialize");

    /// pub trait `DeserializeOwned`
    pub const DeserializeOwned: crate_paths::Path =
        crate_paths::Path::new("serde::de::DeserializeOwned");

    /// pub trait `DeserializeSeed`
    pub const DeserializeSeed: crate_paths::Path =
        crate_paths::Path::new("serde::de::DeserializeSeed");

    /// pub trait `Deserializer`
    pub const Deserializer: crate_paths::Path = crate_paths::Path::new("serde::de::Deserializer");

    /// pub trait `EnumAccess`
    pub const EnumAccess: crate_paths::Path = crate_paths::Path::new("serde::de::EnumAccess");

    /// pub trait `Error`
    pub const Error: crate_paths::Path = crate_paths::Path::new("serde::de::Error");

    /// pub trait `Expected`
    pub const Expected: crate_paths::Path = crate_paths::Path::new("serde::de::Expected");

    /// pub struct `IgnoredAny`
    pub const IgnoredAny: crate_paths::Path = crate_paths::Path::new("serde::de::IgnoredAny");

    /// pub trait `IntoDeserializer`
    pub const IntoDeserializer: crate_paths::Path =
        crate_paths::Path::new("serde::de::IntoDeserializer");

    /// pub trait `MapAccess`
    pub const MapAccess: crate_paths::Path = crate_paths::Path::new("serde::de::MapAccess");

    /// pub trait `SeqAccess`
    pub const SeqAccess: crate_paths::Path = crate_paths::Path::new("serde::de::SeqAccess");

    /// pub enum `Unexpected`
    pub const Unexpected: crate_paths::Path = crate_paths::Path::new("serde::de::Unexpected");

    /// pub trait `VariantAccess`
    pub const VariantAccess: crate_paths::Path = crate_paths::Path::new("serde::de::VariantAccess");

    /// pub trait `Visitor`
    pub const Visitor: crate_paths::Path = crate_paths::Path::new("serde::de::Visitor");
    pub mod value {
        /// pub struct `BoolDeserializer`
        pub const BoolDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::BoolDeserializer");

        /// pub struct `BorrowedBytesDeserializer`
        pub const BorrowedBytesDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::BorrowedBytesDeserializer");

        /// pub struct `BorrowedStrDeserializer`
        pub const BorrowedStrDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::BorrowedStrDeserializer");

        /// pub struct `BytesDeserializer`
        pub const BytesDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::BytesDeserializer");

        /// pub struct `CharDeserializer`
        pub const CharDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::CharDeserializer");

        /// pub struct `CowStrDeserializer`
        pub const CowStrDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::CowStrDeserializer");

        /// pub struct `EnumAccessDeserializer`
        pub const EnumAccessDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::EnumAccessDeserializer");

        /// pub struct `Error`
        pub const Error: crate_paths::Path = crate_paths::Path::new("serde::de::value::Error");

        /// pub struct `F32Deserializer`
        pub const F32Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::F32Deserializer");

        /// pub struct `F64Deserializer`
        pub const F64Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::F64Deserializer");

        /// pub struct `I128Deserializer`
        pub const I128Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::I128Deserializer");

        /// pub struct `I16Deserializer`
        pub const I16Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::I16Deserializer");

        /// pub struct `I32Deserializer`
        pub const I32Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::I32Deserializer");

        /// pub struct `I64Deserializer`
        pub const I64Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::I64Deserializer");

        /// pub struct `I8Deserializer`
        pub const I8Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::I8Deserializer");

        /// pub struct `IsizeDeserializer`
        pub const IsizeDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::IsizeDeserializer");

        /// pub struct `MapAccessDeserializer`
        pub const MapAccessDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::MapAccessDeserializer");

        /// pub struct `MapDeserializer`
        pub const MapDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::MapDeserializer");

        /// pub struct `SeqAccessDeserializer`
        pub const SeqAccessDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::SeqAccessDeserializer");

        /// pub struct `SeqDeserializer`
        pub const SeqDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::SeqDeserializer");

        /// pub struct `StrDeserializer`
        pub const StrDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::StrDeserializer");

        /// pub struct `StringDeserializer`
        pub const StringDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::StringDeserializer");

        /// pub struct `U128Deserializer`
        pub const U128Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::U128Deserializer");

        /// pub struct `U16Deserializer`
        pub const U16Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::U16Deserializer");

        /// pub struct `U32Deserializer`
        pub const U32Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::U32Deserializer");

        /// pub struct `U64Deserializer`
        pub const U64Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::U64Deserializer");

        /// pub struct `U8Deserializer`
        pub const U8Deserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::U8Deserializer");

        /// pub struct `UnitDeserializer`
        pub const UnitDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::UnitDeserializer");

        /// pub struct `UsizeDeserializer`
        pub const UsizeDeserializer: crate_paths::Path =
            crate_paths::Path::new("serde::de::value::UsizeDeserializer");
    }
}
pub mod ser {
    /// pub trait `Error`
    pub const Error: crate_paths::Path = crate_paths::Path::new("serde::ser::Error");

    /// pub struct `Impossible`
    pub const Impossible: crate_paths::Path = crate_paths::Path::new("serde::ser::Impossible");

    /// pub trait `Serialize`
    pub const Serialize: crate_paths::Path = crate_paths::Path::new("serde::ser::Serialize");

    /// pub trait `SerializeMap`
    pub const SerializeMap: crate_paths::Path = crate_paths::Path::new("serde::ser::SerializeMap");

    /// pub trait `SerializeSeq`
    pub const SerializeSeq: crate_paths::Path = crate_paths::Path::new("serde::ser::SerializeSeq");

    /// pub trait `SerializeStruct`
    pub const SerializeStruct: crate_paths::Path =
        crate_paths::Path::new("serde::ser::SerializeStruct");

    /// pub trait `SerializeStructVariant`
    pub const SerializeStructVariant: crate_paths::Path =
        crate_paths::Path::new("serde::ser::SerializeStructVariant");

    /// pub trait `SerializeTuple`
    pub const SerializeTuple: crate_paths::Path =
        crate_paths::Path::new("serde::ser::SerializeTuple");

    /// pub trait `SerializeTupleStruct`
    pub const SerializeTupleStruct: crate_paths::Path =
        crate_paths::Path::new("serde::ser::SerializeTupleStruct");

    /// pub trait `SerializeTupleVariant`
    pub const SerializeTupleVariant: crate_paths::Path =
        crate_paths::Path::new("serde::ser::SerializeTupleVariant");

    /// pub trait `Serializer`
    pub const Serializer: crate_paths::Path = crate_paths::Path::new("serde::ser::Serializer");
}
