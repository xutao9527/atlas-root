#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasWireKind {
    Request  = 0b0000_0001,
    Response = 0b0000_0010,
    Notify   = 0b0000_0100,
}

#[derive(Debug, Clone, Copy)]
pub struct AtlasWireHeader {
    pub id: u64,
    pub slot_index: u32,
    pub method: u32,
    pub kind: AtlasWireKind,
}

pub type AtlasRawRequest = AtlasWireRequest<bytes::Bytes>;
pub type AtlasRawResponse = AtlasWireResponse<bytes::Bytes>;

#[derive(Debug, Clone)]
pub struct AtlasWireRequest<T>
{
    pub header: AtlasWireHeader,
    pub payload: T,
}

#[derive(Debug, Clone)]
pub struct AtlasWireResponse<T>
{
    pub header: AtlasWireHeader,
    pub payload: T,
    pub error: Option<String>,
}
#[derive(Debug, Clone, Copy)]
pub struct AtlasWireNotify<T> {
    pub header: AtlasWireHeader,
    pub payload: T,
}

