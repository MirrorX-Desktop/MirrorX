use crate::error::CoreResult;
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use once_cell::sync::Lazy;

static SERIALIZER: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});

pub fn bincode_serialize<S>(t: &S) -> CoreResult<Vec<u8>>
where
    S: ?Sized + serde::Serialize,
{
    let buffer = SERIALIZER.serialize(t)?;
    Ok(buffer)
}

pub fn bincode_deserialize<'a, T>(bytes: &'a [u8]) -> CoreResult<T>
where
    T: serde::Deserialize<'a>,
{
    let ty = SERIALIZER.deserialize(bytes)?;
    Ok(ty)
}
