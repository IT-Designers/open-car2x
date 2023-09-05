//! Accumulation of all FFI-Error codes
//!
//! CrResult info regarding assigned numbers.
//! The integer value should not exceed 32-bit
//!
//! ```text
//! ErrParameterConnectionPointerIsNull = 0x0010_0000,
//!    +++++++++                            +++       The group number
//!             +++++++++                      + ++   The sub-group number
//!                      ++++++++++++++            ++ Regular increasing counter
//!
//! ErrParameterAddressIsNotValidUtf8 = 0x0010_0200,
//!    +++++++++                          +++       The group number
//!             +++++++                      + ++   The sub-group number
//!                    ++++++++++++++            ++ Regular increasing counter
//! ```
#[cfg(doc)]
use crate::connection::CrConnection;
#[cfg(doc)]
use crate::pods::application::CrApplicationInfo;
#[cfg(doc)]
use crate::pods::connection::{CrConnectionInfo, CrConnectionStatus};
#[cfg(doc)]
use crate::pods::connector::CrConnectorInfo;
#[cfg(doc)]
use crate::pods::message::CrMessageRef;
#[cfg(doc)]
use crate::pods::message::{CrMessage, CrMessageId, CR_MESSAGE_BODY_SIZE_LIMIT};
#[cfg(doc)]
use crate::worker::conf::CrConnectionConfig;
#[cfg(doc)]
use crate::worker::rcv::CrDetailedMessage;
use num_traits::FromPrimitive;

/// There exist the following error groups
///
/// ```text
/// Ok                                  = 0x0000_0000,
/// ErrParameter..[Null,NotNull]        = 0x0010_00..,
/// ErrParameter..[IsNotValidUtf8]      = 0x0010_01..,
/// ErrParameter..                      = 0x0010_02..,
/// ErrWorker..                         = 0x002._....,
/// ErrConnection..                     = 0x003._....,
/// ErrUper..                           = 0x004._....,
/// ErrJson..                           = 0x005._....,
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, CStr, FromPrimitive)]
pub enum CrResult {
    Ok = 0,

    /// The parameter pointing to the [`CrConnection`] is `NULL` but mustn't be.
    ErrParameterConnectionPointerIsNull = 0x0010_0000,
    /// The pointer to the [`CrConnection`] the parameter dereferences to is `NULL` but mustn't be.
    ErrParameterConnectionPointerPointerIsNull,
    /// The pointer to the [`CrConnection`] the parameter dereferences to is *not* `NULL` but *must* be.
    ErrParameterConnectionPointerPointerIsNotNull,
    /// The parameter pointing to the [`CrApplicationInfo`] is `NULL` but mustn't be.
    ErrParameterApplicationInfoIsNull,
    /// The parameter pointing to the [`CrMessage`] is null but mustn't be.
    ErrParameterMessageIsNull,
    /// The parameter pointing to the body of the message is null but mustn't be.
    ErrParameterMessageBodyIsNull,
    /// The parameter pointing to a [`CrMessageId`] is `NULL` but mustn't be.
    ErrParameterMessageIdIsNull,
    /// The parameter pointing to a [`CrConnectionInfo`] is `NULL` but mustn't be.
    ErrParameterConnectionInfoIsNull,
    /// The parameter pointing to a [`CrConnectorInfo`] is `NULL` but mustn't be.
    ErrParameterConnectorInfoIsNull,
    /// The target buffer to write the chars into is `NULL` but mustn't be.
    ErrParameterCharTargetBufferIsNull,
    /// The parameter pointing to [`CrConnectionConfig`] is `NULL` but mustn't be.
    ErrParameterConnectionConfigPointerIsNull,
    /// The pointer to [`CrConnectionConfig`] dereferences *not* to `NULL` but must be.
    ErrParameterConnectionConfigPointerPointerIsNull,
    /// The pointer to [`CrConnectionConfig`] dereferences to `NULL` but mustn't be.
    ErrParameterConnectionConfigPointerPointerIsNotNull,
    /// The source buffer to read from is `NULL` but mustn't be.
    ErrParameterSourceBufferIsNull,
    /// The target buffer to write to is `NULL` but mustn't be.
    ErrParameterTargetBufferIsNull,
    /// The pointer to read and write the target len to is `NULL` but mustn't be.
    ErrParameterTargetBufferLenIsNull,
    /// The target buffer to write the to is lot large enough.
    ErrParameterTargetBufferInsufficientSize,
    /// The address parameter is not encoded as valid UTF-8 string.
    ErrParameterAddressIsNotValidUtf8 = 0x0010_0100,
    /// The name field within the [`CrApplicationInfo`] is not encoded as valid UTF-8 string.
    ErrParameterApplicationNameIsNotValidUtf8,
    /// The given parameter is no valid UTF-8 string.
    ErrParameterIsNotValidUtf8,
    /// The size of the message body exceeds the allowed limit of [`CR_MESSAGE_BODY_SIZE_LIMIT].
    ErrParameterMessageSizeExceedsLimit = 0x0010_0200,
    /// The parameter for the message-id is no valid single variant of [`CrMessageId`].
    ErrParameterMessageIdIsInvalid,
    /// The target buffer to write the chars into is lot large enough.
    ErrParameterCharTargetBufferInsufficientSize,
    /// The parameter for the format is no valid variant of [`crate::conversion::CrFormat`].
    ErrParameterFormatIsInvalid,

    /// The parameter pointing to the [`CrDetailedMessage`] is `NULL`.
    ErrParameterDetailedMessagePointerIsNull = 0x0010_0300,
    /// The inner pointer of [`CrDetailedMessage`] is `NULL`.
    ErrParameterDetailedMessageInnerPointerIsNull,
    /// The pointer for storing the creation-time is `NULL`.
    ErrParameterDetailedMessageCreationTimePointerIsNull,
    /// The pointer for storing the reception-time is `NULL`.
    ErrParameterDetailedMessageReceptionTimePointerIsNull,
    /// The pointer for storing the [`CrMessageRef`] is `NULL`.
    ErrParameterDetailedMessageMessageRefPointerIsNull,

    /// The connection worker detached itself unexpectedly and can no longer be reached.
    ErrWorkerDetached = 0x0020_0000,
    /// The connection worker caused a panic. Please see stderr for details.
    ErrWorkerPanic,
    /// For some reason the request has been ignored. See stderr for more details.
    ErrWorkerRequestDropped,
    /// The request reached its time limit before it could be fulfilled successfully.
    ErrWorkerRequestTimeoutReached,

    /// The [`CrConnectionStatus`] is currently unavailable.
    ErrConnectionStatusUnavailable = 0x0030_0000,

    /// Failed to decode the uPER message. Either a wrong [`CrMessageId`] was specified, an
    /// incompatible version is used or the data is corrupt. See stderr for more details.
    ErrUperDecodingFailed = 0x0040_0000,
    /// Failed to encode the data as uPER message. See stderr for more details.
    ErrUperEncodingFailed,

    /// Failed to decode the JSON message. Either a wrong [`CrMessageId`] was specified, an
    /// incompatible version is used or the data is corrupt.
    ErrJsonDecodingFailed = 0x0050_0000,
    /// Failed to encode the message as JSON. See stderr for more details.
    ErrJsonEncodingFailed,

    /// Failed to encode the data as human readable text.
    ErrTextEncodingFailed = 0x0060_0000,

    /// At least one constraint violation found
    ErrConstraintCheckViolationFound = 0x0070_0000,

    /// Failed to decode the BSON message. Either a wrong [`CrMessageId`] was specified, an
    /// incompatible version is used or the data is corrupt.
    ErrBsonDecodingFailed = 0x0080_0000,
    /// Failed to encode the message as BSON. See stderr for more details
    ErrBsonEncodingFailed,

    /// Failed to decode the protobuf message. Either a wrong [`CrMessageId`] was specified, an
    /// incompatible version is used or the data is corrupt.
    ErrProtobufDecodingFailed = 0x0090_0000,
    /// Failed to encode the message as protobuf. See stderr for more details
    ErrProtobufEncodingFailed,

    /// The logger is already configured and does not support a reconfiguration.
    ErrLoggerAlreadyConfigured = 0x00A0_0000,
}

/// Returns the string representation of the given value. The returned pointer is references a
/// null-terminated string. For an invalid variant of [`CrResult`] the returned pointer is
/// "\<Invalid>", but never `NULL`.
#[no_mangle]
pub extern "C" fn cr_result_str(result: CrResult) -> *const std::os::raw::c_char {
    // C can pass an invalid value so treat it as isize value (which it is for C)
    match CrResult::from_isize(result as isize) {
        Some(result) => result.as_cstr().as_ptr(),
        None => cstr::cstr!("<Invalid>").as_ptr(),
    }
}
