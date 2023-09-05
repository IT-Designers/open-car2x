use bytes::Bytes;
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum MessageBody {
    Bytes(Bytes),
    Vec(Vec<u8>),
}

impl MessageBody {
    pub fn bytes(&self) -> &[u8] {
        &self[..]
    }

    pub fn len(&self) -> usize {
        match self {
            MessageBody::Bytes(bytes) => bytes.len(),
            MessageBody::Vec(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MessageBody::Bytes(bytes) => bytes.is_empty(),
            MessageBody::Vec(vec) => vec.is_empty(),
        }
    }
}

impl PartialEq for MessageBody {
    fn eq(&self, other: &Self) -> bool {
        let this = &self[..];
        let other = &other[..];
        this.eq(other)
    }
}

impl PartialOrd<Self> for MessageBody {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this = &self[..];
        let other = &other[..];
        this.partial_cmp(other)
    }
}

impl<I> std::ops::Index<I> for MessageBody
where
    I: std::slice::SliceIndex<[u8]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        match self {
            MessageBody::Bytes(bytes) => &bytes[index],
            MessageBody::Vec(vec) => &vec[index],
        }
    }
}
