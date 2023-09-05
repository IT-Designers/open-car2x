use common_message::MessageId;
use dove::container::Message;
use dove::options::{ReceiverFilter, ReceiverOptions};
use dove::types::Timestamp;
use std::time::UNIX_EPOCH;

pub const MESSAGE_ID_IDENTIFIER: &str = "mid";
pub const SENDER_ID_IDENTIFIER: &str = "sender";
pub const STATION_ID_IDENTIFIER: &str = "station_id";

pub fn message_id_from_message_headers(message: &Message) -> Option<MessageId> {
    message_id_with_origin_from_message_headers(message).map(|mid| match mid {
        MessageIdOrigin::ApplicationProperties(mid) => mid,
        MessageIdOrigin::SubjectMessageId(mid) => {
            debug!(
                "Missing message-id field in application-properties map of Message: {:?}",
                message
            );
            mid
        }
    })
}

enum MessageIdOrigin {
    ApplicationProperties(MessageId),
    SubjectMessageId(MessageId),
}

fn message_id_with_origin_from_message_headers(message: &Message) -> Option<MessageIdOrigin> {
    message_id_from_application_properties(message)
        .map(MessageIdOrigin::ApplicationProperties)
        .or_else(|| {
            message_id_from_subject_message_id(message).map(MessageIdOrigin::SubjectMessageId)
        })
}

fn message_id_from_application_properties(message: &Message) -> Option<MessageId> {
    message.application_properties.as_ref().and_then(|p| {
        p.iter().find_map(|(key, value)| {
            if matches!(key, dove::container::Value::String(s) if s == MESSAGE_ID_IDENTIFIER) {
                value
                    .as_any_integer()
                    .and_then(|mid| {
                        if mid.is_positive() && mid < i64::from(u16::MAX) {
                            Some(mid as u16)
                        } else {
                            None
                        }
                    })
                    .map(MessageId::from)
            } else {
                None
            }
        })
    })
}

fn message_id_from_subject_message_id(message: &Message) -> Option<MessageId> {
    let subject_message_id = message
        .properties
        .as_ref()
        .and_then(|p| p.subject.as_deref());

    match subject_message_id {
        None => {
            error!(
                "Missing subject field in properties map of Message: {:?}",
                message
            );
            None
        }
        Some(mid) => {
            use common_message::strum::IntoEnumIterator;
            MessageId::iter().find(|m| mid.eq_ignore_ascii_case(m.as_ref()))
        }
    }
}

pub fn wrap_message(
    sender_id: u32,
    reply_to: Option<String>,
    mid: MessageId,
    body: impl Into<Vec<u8>>,
    station_id: Option<u32>,
) -> Message {
    let message_id = {
        let mut id = format!("{:?}", mid);
        id.make_ascii_lowercase();
        id
    };

    Message {
        header: None,
        delivery_annotations: None,
        message_annotations: None,
        properties: Some(dove::message::MessageProperties {
            message_id: None,
            user_id: None,
            to: None,
            subject: Some(message_id),
            reply_to,
            correlation_id: None,
            content_type: None,
            content_encoding: None,
            absolute_expiry_time: None,
            creation_time: Some(Timestamp(
                UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64,
            )),
            group_id: None,
            group_sequence: None,
            reply_to_group_id: None,
        }),
        application_properties: Some(
            [
                Some((
                    dove::container::Value::Str(MESSAGE_ID_IDENTIFIER),
                    dove::container::Value::Ushort(mid.into()),
                )),
                Some((
                    dove::container::Value::Str(SENDER_ID_IDENTIFIER),
                    dove::container::Value::Uint(sender_id),
                )),
                station_id.map(|station_id| {
                    (
                        dove::container::Value::Str(STATION_ID_IDENTIFIER),
                        dove::container::Value::Uint(station_id),
                    )
                }),
            ]
            .into_iter()
            .flatten()
            .collect(),
        ),
        body: dove::message::MessageBody::Data(body.into()),
        footer: None,
    }
}

pub fn receiver_filter(
    filter_sender_id: Option<u32>,
    filter_messages: impl Iterator<Item = u16>,
    filter_station_id: Option<u32>,
) -> ReceiverOptions {
    let query = [
        filter_sender_id.map(|sender_id| {
            format!(
                "(NOT({} = {}) OR {} IS NULL)",
                SENDER_ID_IDENTIFIER, sender_id, SENDER_ID_IDENTIFIER
            )
        }),
        {
            let accepted_messages = filter_messages
                .map(|mid| format!("{} = {}", MESSAGE_ID_IDENTIFIER, mid))
                .collect::<Vec<String>>();
            let multiple = accepted_messages.len() > 1;
            let accepted_messages = accepted_messages.join(" OR ");
            if accepted_messages.is_empty() {
                None
            } else if multiple {
                Some(format!("({})", accepted_messages))
            } else {
                Some(accepted_messages)
            }
        },
        filter_station_id.map(|station_id| format!("station_id = {station_id}")),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" AND ");

    if cfg!(debug_assertions) {
        info!("Receiver filter-query: {}", query);
    }

    if query.is_empty() {
        ReceiverOptions::default()
    } else {
        ReceiverFilter::apache_selector(query).into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use dove::options::apache_selector::Selector;
    use std::iter::empty;

    #[inline]
    fn assert_eq_apache_selector<'a>(
        expected: impl Into<Option<&'a str>> + 'a,
        actual: Option<ReceiverFilter>,
    ) {
        if let Some(expected) = expected.into() {
            if let Some(ReceiverFilter::ApacheSelector(Selector { query })) = actual {
                assert_eq!(expected, query.as_str());
            } else {
                panic!("Unexpected ReceiveFilter: {actual:?}");
            }
        } else if actual.is_some() {
            panic!("Expected no filter but got instead: {actual:?}");
        }
    }

    #[test]
    fn receive_filter_empty() {
        assert_eq_apache_selector(None, receiver_filter(None, empty(), None).filter);
    }

    #[test]
    fn receive_filter_sender_id() {
        assert_eq_apache_selector(
            "(NOT(sender = 1234) OR sender IS NULL)",
            receiver_filter(Some(1234), empty(), None).filter,
        );
    }

    #[test]
    fn receive_filter_sender_id_and_one_filter_message() {
        assert_eq_apache_selector(
            "(NOT(sender = 1234) OR sender IS NULL) AND mid = 2049",
            receiver_filter(
                Some(1234),
                [MessageId::Cpm].into_iter().map(u16::from),
                None,
            )
            .filter,
        );
    }

    #[test]
    fn receive_filter_sender_id_and_two_filter_messages() {
        assert_eq_apache_selector(
            "(NOT(sender = 1234) OR sender IS NULL) AND (mid = 2049 OR mid = 2052)",
            receiver_filter(
                Some(1234),
                [MessageId::Cpm, MessageId::Mcm].into_iter().map(u16::from),
                None,
            )
            .filter,
        );
    }

    #[test]
    fn receive_filter_sender_id_and_two_filter_messages_and_station_id() {
        assert_eq_apache_selector(
            "(NOT(sender = 1234) OR sender IS NULL) AND (mid = 2049 OR mid = 2052) AND station_id = 42",
            receiver_filter(
                Some(1234),
                [MessageId::Cpm, MessageId::Mcm].into_iter().map(u16::from),
                Some(42),
            )
            .filter,
        );
    }

    #[test]
    fn receive_filter_two_filter_messages_and_station_id() {
        assert_eq_apache_selector(
            "(mid = 2049 OR mid = 2052) AND station_id = 42",
            receiver_filter(
                None,
                [MessageId::Cpm, MessageId::Mcm].into_iter().map(u16::from),
                Some(42),
            )
            .filter,
        );
    }

    #[test]
    fn receive_filter_station_id() {
        assert_eq_apache_selector(
            "station_id = 42",
            receiver_filter(None, empty(), Some(42)).filter,
        );
    }
}
