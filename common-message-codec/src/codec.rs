pub trait StaticEncoder {
    type Target;
    type Item;
    type Error: From<std::io::Error>;

    fn guess_meta(item: &Self::Item) -> Self::Target;

    fn encode_into(item: &Self::Item) -> Result<Self::Target, Self::Error>;
}

pub trait StaticDecoder {
    type Source;
    type Item: Sized;
    type Error: From<std::io::Error>;

    fn decode_from(source: &Self::Source) -> Result<Self::Item, Self::Error>;
}
