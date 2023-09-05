use crate::pods::message::{CrMessageId, CrMessageIdOverloaded, CrMessageRef};
use crate::result::CrResult;
use common_edge::ReceivedMessage;
use std::ffi::c_void;

#[repr(C)]
pub struct CrDetailedMessage {
    pub content: CrMessageRef,
    pub details: *mut c_void,
}

impl CrDetailedMessage {
    pub fn id(&self) -> CrMessageIdOverloaded {
        CrMessageIdOverloaded::from(self.content.id)
    }

    /// Returns a reference to the inner [`ReceivedMessage`].
    ///
    /// # Safety
    ///
    /// The inner `details` pointer must be valid and not be tampered with.
    pub unsafe fn received_message_ref(&self) -> Option<&ReceivedMessage> {
        if self.details.is_null() {
            None
        } else {
            Some(&*(self.details as *const ReceivedMessage))
        }
    }

    /// Returns the inner values of `self` in a new instance and writes empty values to `self`
    fn take(&mut self) -> Self {
        Self {
            content: CrMessageRef {
                id: self.content.id,
                size: core::mem::take(&mut self.content.size),
                body: core::mem::replace(&mut self.content.body, core::ptr::null()),
            },
            details: core::mem::replace(&mut self.details, core::ptr::null_mut()),
        }
    }

    /// Frees the inner value of `self`
    ///
    /// # Safety
    ///
    /// The inner values must not be freed before and point to a valid memory location.
    pub unsafe fn free(self) {
        let inner: Box<ReceivedMessage> = Box::from_raw(self.details as *mut ReceivedMessage);
        drop(inner);
    }

    /// Wraps the given values into a new instance of `Self`
    ///
    /// # Safety
    ///
    /// The memory must be freed manually by calling [`Self::free`]
    pub unsafe fn from(id: CrMessageId, message: Box<ReceivedMessage>) -> Self {
        Self {
            content: CrMessageRef {
                id,
                size: message.body().len(),
                body: message.body().as_ptr(),
            },
            details: Box::into_raw(message) as _,
        }
    }
}

/// Retrieves the time the message was created in milliseconds since unix epoch. Retrieves `0` if
/// unknown.
///
/// This field is normally set by the sender of the message right before submitting the message for
/// transmission. An comparison of `creation_time` and `reception_time` could lead to an estimation
/// of the transmission time for the received message.
///
/// # Safety
///
/// The given pointers must not be `NULL` and point to a valid `CrDetailedMessage` instance.
#[no_mangle]
pub unsafe extern "C" fn cr_detailed_message_creation_time(
    message: *const CrDetailedMessage,
    creation_time: *mut u64,
) -> CrResult {
    let message = valid_ptr_or!(message, CrResult::ErrParameterDetailedMessagePointerIsNull);
    let inner = some_or_return!(
        message.received_message_ref(),
        CrResult::ErrParameterDetailedMessageInnerPointerIsNull
    );

    let creation_time = valid_mut_ptr_or!(
        creation_time,
        CrResult::ErrParameterDetailedMessageCreationTimePointerIsNull
    );

    *creation_time = inner.creation_time_millis().unwrap_or_default();
    CrResult::Ok
}

/// Retrieves the time the message was received in milliseconds since unix epoch.
///
/// This field is normally set by the receiver of the message immediately after transmission. An
/// comparison of `creation_time` and `reception_time` could lead to an estimation of the
/// transmission time for the received message.
///
/// # Safety
///
/// The given pointers must not be `NULL` and point to a valid `CrDetailedMessage` instance.
#[no_mangle]
pub unsafe extern "C" fn cr_detailed_message_reception_time(
    message: *const CrDetailedMessage,
    reception_time: *mut u64,
) -> CrResult {
    let message = valid_ptr_or!(message, CrResult::ErrParameterDetailedMessagePointerIsNull);
    let inner = some_or_return!(
        message.received_message_ref(),
        CrResult::ErrParameterDetailedMessageInnerPointerIsNull
    );

    let reception_time = valid_mut_ptr_or!(
        reception_time,
        CrResult::ErrParameterDetailedMessageReceptionTimePointerIsNull
    );

    *reception_time = inner.reception_time_millis();
    CrResult::Ok
}

/// Frees the [`CrDetailedMessage`] instance and zeroes it on success.
///
/// # Safety
///
/// The given pointer must not be `NULL` and point to a valid `CrDetailedMessage` instance.
#[no_mangle]
pub unsafe extern "C" fn cr_detailed_message_free(message: *mut CrDetailedMessage) -> CrResult {
    valid_mut_ptr_or!(message, CrResult::ErrParameterMessageIsNull)
        .take()
        .free();
    CrResult::Ok
}
