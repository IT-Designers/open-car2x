use crate::result::CrResult;
use crate::util::display_to_extern_char_buffer;
use common_edge::common_message;
use common_edge::common_message_codec;
use common_message::MessageId;
use common_message_codec::EdgeMessageUperCodec;
use common_message_codec::StaticDecoder;
use common_message_codec::{MessageBody, RawParts};
use itertools::Itertools;
use log::error;
use std::convert::TryFrom;
use std::io::Write;
use std::mem::size_of;
use std::os::raw::c_char;
use strum::IntoEnumIterator;

pub const CR_MESSAGE_BODY_SIZE_LIMIT: usize = 8000;

#[repr(C)]
pub struct CrMessage {
    pub id: CrMessageIdOverloaded,
    pub size: usize,
    pub body: [u8; CR_MESSAGE_BODY_SIZE_LIMIT],
}

#[repr(C)]
pub struct CrMessageRef {
    pub id: CrMessageId,
    pub size: usize,
    pub body: *const u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, EnumIter, CStr, FromPrimitive)]
pub enum CrMessageId {
    // zero is reserved for 'no-filter'
    Denm = 0b1,
    Cpm = 0b10,
    Cam = 0b100,
    Vam = 0b1000,
    Mcm = 0b10000,

    #[allow(clippy::unusual_byte_groupings)]
    ApplicationInfo = 0b1___00000000_00000000,
    #[allow(clippy::unusual_byte_groupings)]
    DebugRequest = 0b10___00000000_00000000,
    #[allow(clippy::unusual_byte_groupings)]
    ComponentStatus = 0b100___00000000_00000000,
    // #[allow(clippy::unusual_byte_groupings)]
    // UulmRawBson = 0b1000___00000000_00000000,
}

macro_rules! from_to {
    (@parse $_from:ident, $_to:ident, $on:expr, {$($parsed:tt)*} $(,)*) => {
        match $on {
            $($parsed)*
        }
    };
    (@parse $from:ident, $to:ident, $on:expr, {$($parsed:tt)*}, $name:ident, $($remaining:tt)*) => {
        from_to! (
            @parse $from, $to, $on,
            {
                $($parsed)*
                $from::$name => $to::$name,
            },
            $($remaining)*
        )
    };
    (@parse $from:ident, $to:ident, $on:expr, {$($parsed:tt)*}, $name:ident => $other:ident, $($remaining:tt)*) => {
        from_to! (
            @parse $from, $to, $on,
            {
                $($parsed)*
                $from::$name => $to::$other,
            },
            $($remaining)*
        )
    };
    (@parse $from:ident, $to:ident, $on:expr, {$($parsed:tt)*}, $name:tt => $other:expr, $($remaining:tt)*) => {
        from_to! (
            @parse $from, $to, $on,
            {
                $($parsed)*
                $name => $other,
            },
            $($remaining)*
        )
    };
    ($from:ident, $to:ident, $on:expr, $($remaining:tt)*) => {
        from_to! (@parse $from, $to, $on, {}, $($remaining)*)
    };
}

impl TryFrom<MessageId> for CrMessageId {
    type Error = MessageId;

    fn try_from(mid: MessageId) -> Result<Self, Self::Error> {
        Ok(from_to!(
            MessageId,
            CrMessageId,
            mid,

            Denm,
            Cpm,
            Cam,
            Vam,
            Mcm,

            ApplicationInfo,
            DebugRequest,
            ComponentStatus,

            id => return Err(id),
        ))
    }
}

impl CrMessageId {
    pub fn opt_from(id: u64) -> Option<CrMessageId> {
        Self::iter().find(|i| *i as u64 == id)
    }

    pub(crate) fn to_common_id(self) -> MessageId {
        from_to! {
            CrMessageId,
            MessageId,
            self,

            Denm,
            Cpm,
            Cam,
            Vam,
            Mcm,

            ApplicationInfo,
            ComponentStatus,
            DebugRequest,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct CrMessageIdOverloaded(u64);

impl CrMessageIdOverloaded {
    pub fn empty() -> Self {
        Self(0u64)
    }

    pub fn add(&mut self, id: CrMessageId) {
        self.0 |= id as u64;
    }

    pub fn separate(self) -> Vec<CrMessageId> {
        self.iter_set().collect()
    }

    pub fn into_single(self) -> Option<CrMessageId> {
        let mut iter = self.iter_set();
        let first = iter.next();
        if iter.next().is_none() {
            first
        } else {
            None
        }
    }

    pub fn iter_set(self) -> impl Iterator<Item = CrMessageId> {
        CrMessageId::iter().filter(move |v| self.is_set(*v))
    }

    pub fn is_overloaded(self) -> bool {
        self.iter_set().count() > 1
    }

    fn is_set(self, variant: CrMessageId) -> bool {
        debug_assert!(size_of::<CrMessageIdOverloaded>() >= size_of::<CrMessageId>());
        self.0 & variant as u64 == variant as u64
    }
}

impl std::fmt::Debug for CrMessageIdOverloaded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple(stringify!(CrMessageIdOverloaded));
        if self.iter_set().count() == 0 {
            f.field(&format_args!("_"));
        }
        for v in self.iter_set() {
            f.field(&v);
        }
        f.finish()
    }
}

impl From<CrMessageId> for CrMessageIdOverloaded {
    fn from(id: CrMessageId) -> Self {
        let mut result = Self::empty();
        result.add(id);
        result
    }
}

/// Writes the string representation of the given [`CrMessageIdOverloaded`] value to the given
/// buffer. The string representation will be truncated so that it and the zero byte `\0` fit into
/// it.
///
/// # Safety
///
/// The given `target_buffer` pointer must point to a writable memory location that is at least
/// `buffer_len`-bytes large.
#[no_mangle]
pub unsafe extern "C" fn cr_message_ids_to_string(
    target_buffer: *mut c_char,
    buffer_len: usize,
    status: CrMessageIdOverloaded,
) -> CrResult {
    let string = format!(
        "[{}]",
        status
            .iter_set()
            .map(|s| s.as_cstr().to_string_lossy())
            .join(", ")
    );
    display_to_extern_char_buffer(target_buffer, buffer_len, string)
}

/// Returns the string representation of the given value. The returned pointer is references a
/// null-terminated byte string. If the given value is an empty [`CrMessageIdOverloaded`], the
/// string "\<Empty>" is returned and for overloaded values "\<Overloaded>" is  returned.
/// The returned pointer is never `NULL`.
#[no_mangle]
pub extern "C" fn cr_message_id_str(result: CrMessageIdOverloaded) -> *const c_char {
    if let Some(message_id) = result.into_single() {
        message_id.as_cstr().as_ptr()
    } else if result.iter_set().count() == 0 {
        cstr::cstr!("<Empty>").as_ptr()
    } else {
        cstr::cstr!("<Overloaded>").as_ptr()
    }
}

/// Decodes the given message and writes the content as JSON text into the given buffer.
///
/// # Safety
///
/// The target buffer must point to a valid memory region of at least the size of `buffer_len`.
/// The body pointer of the given message must point to a valid memory region of at least the size
/// of the `size` property of [`CrMessageRef`].  
#[no_mangle]
pub unsafe extern "C" fn cr_message_uper_to_json_pretty(
    message: CrMessageRef,
    target_buffer: *mut c_char,
    buffer_len: usize,
) -> CrResult {
    if message.body.is_null() {
        return CrResult::ErrParameterMessageBodyIsNull;
    }

    if target_buffer.is_null() {
        return CrResult::ErrParameterCharTargetBufferIsNull;
    }

    let body = core::slice::from_raw_parts(message.body, message.size);
    let mut target = std::slice::from_raw_parts_mut(target_buffer as *mut u8, buffer_len);
    let mut writer = &mut target;

    let raw = RawParts {
        protocol: EdgeMessageUperCodec::PROTOCOL_LUKAS_CURRENT,
        message_id: message.id.to_common_id(),
        body: MessageBody::Vec(body.to_vec()),
    };

    match EdgeMessageUperCodec::decode_from(&raw) {
        Err(e) => {
            eprintln!("{e}");
            error!("{e}");
            CrResult::ErrUperDecodingFailed
        }
        Ok(message) => match message.inner_to_json_writer_pretty(&mut writer) {
            Err(e) if e.is_io() => CrResult::ErrParameterCharTargetBufferInsufficientSize,
            Err(e) => {
                eprintln!("{e}");
                error!("{e}");
                CrResult::ErrJsonEncodingFailed
            }
            Ok(_) => match writer.write(&[0x00]) {
                Err(_) => CrResult::ErrParameterCharTargetBufferInsufficientSize,
                Ok(_) => CrResult::Ok,
            },
        },
    }
}
