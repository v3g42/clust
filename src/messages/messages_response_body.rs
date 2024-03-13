use crate::macros::{
    impl_display_for_serialize, impl_enum_string_serialization,
};
use crate::messages::{
    ClaudeModel, Content, Role, StopReason, StopSequence, Usage,
};
use std::fmt::{Display, Formatter};

/// The response body for the Messages API.
///
/// See also [the Messages API](https://docs.anthropic.com/claude/reference/messages_post).
#[derive(
    Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize,
)]
pub struct MessagesResponseBody {
    /// Unique object identifier.
    ///
    /// The format and length of IDs may change over time.
    pub id: String,
    /// Object type.
    ///
    /// For Messages, this is always "message".
    #[serde(rename = "type")]
    pub _type: MessageObjectType,
    /// Conversational role of the generated message.
    ///
    /// This will always be "assistant".
    pub role: Role,
    /// Content generated by the model.
    ///
    /// This is an array of content blocks, each of which has a type that determines its shape. Currently, the only type in responses is "text".
    pub content: Content,
    /// The model that handled the request.
    pub model: ClaudeModel,
    /// The reason that we stopped.
    ///
    /// This may be one of the following values:
    ///
    /// "end_turn": the model reached a natural stopping point
    /// "max_tokens": we exceeded the requested max_tokens or the model's maximum
    /// "stop_sequence": one of your provided custom stop_sequences was generated
    /// Note that these values are different from those in /v1/complete, where end_turn and stop_sequence were not differentiated.
    ///
    /// In non-streaming mode this value is always non-null. In streaming mode, it is null in the message_start event and non-null otherwise.
    pub stop_reason: Option<StopReason>,
    /// Which custom stop sequence was generated, if any.
    ///
    /// This value will be a non-null string if one of your custom stop sequences was generated.
    pub stop_sequence: Option<StopSequence>,
    /// Billing and rate-limit usage.
    ///
    /// Anthropic's API bills and rate-limits by token counts, as tokens represent the underlying cost to our systems.
    ///
    /// Under the hood, the API transforms requests into a format suitable for the model. The model's output then goes through a parsing stage before becoming an API response. As a result, the token counts in usage will not match one-to-one with the exact visible content of an API request or response.
    ///
    /// For example, output_tokens will be non-zero, even for an empty string response from Claude.
    pub usage: Usage,
}

impl_display_for_serialize!(MessagesResponseBody);

/// The object type for message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageObjectType {
    /// message
    Message,
}

impl Default for MessageObjectType {
    fn default() -> Self {
        Self::Message
    }
}

impl Display for MessageObjectType {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | MessageObjectType::Message => write!(f, "{}", "message"),
        }
    }
}

impl_enum_string_serialization!(
    MessageObjectType,
    Message => "message"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let response = MessagesResponseBody {
            id: "id".to_string(),
            _type: MessageObjectType::Message,
            role: Role::Assistant,
            content: "content".into(),
            model: ClaudeModel::Claude3Sonnet20240229,
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
            usage: Usage {
                input_tokens: 1,
                output_tokens: 2,
            },
        };
        assert_eq!(
            serde_json::to_string(&response).unwrap(),
            "{\"id\":\"id\",\"type\":\"message\",\"role\":\"assistant\",\"content\":\"content\",\"model\":\"claude-3-sonnet-20240229\",\"stop_reason\":\"end_turn\",\"stop_sequence\":\"stop_sequence\",\"usage\":{\"input_tokens\":1,\"output_tokens\":2}}"
        );
    }

    #[test]
    fn deserialize() {
        let response = MessagesResponseBody {
            id: "id".to_string(),
            _type: MessageObjectType::Message,
            role: Role::Assistant,
            content: "content".into(),
            model: ClaudeModel::Claude3Sonnet20240229,
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
            usage: Usage {
                input_tokens: 1,
                output_tokens: 2,
            },
        };
        assert_eq!(
            serde_json::from_str::<MessagesResponseBody>(
            "{\"id\":\"id\",\"type\":\"message\",\"role\":\"assistant\",\"content\":\"content\",\"model\":\"claude-3-sonnet-20240229\",\"stop_reason\":\"end_turn\",\"stop_sequence\":\"stop_sequence\",\"usage\":{\"input_tokens\":1,\"output_tokens\":2}}"
            ).unwrap(),
            response
        );
    }

    #[test]
    fn display() {
        let response = MessagesResponseBody {
            id: "id".to_string(),
            _type: MessageObjectType::Message,
            role: Role::Assistant,
            content: "content".into(),
            model: ClaudeModel::Claude3Sonnet20240229,
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: Some(StopSequence::new("stop_sequence")),
            usage: Usage {
                input_tokens: 1,
                output_tokens: 2,
            },
        };
        assert_eq!(
            response.to_string(),
            "{\n  \"id\": \"id\",\n  \"type\": \"message\",\n  \"role\": \"assistant\",\n  \"content\": \"content\",\n  \"model\": \"claude-3-sonnet-20240229\",\n  \"stop_reason\": \"end_turn\",\n  \"stop_sequence\": \"stop_sequence\",\n  \"usage\": {\n    \"input_tokens\": 1,\n    \"output_tokens\": 2\n  }\n}"
        );
    }

    #[test]
    fn default_message_object_type() {
        assert_eq!(
            MessageObjectType::default(),
            MessageObjectType::Message
        );
    }

    #[test]
    fn message_object_type_display() {
        assert_eq!(
            MessageObjectType::Message.to_string(),
            "message"
        );
    }

    #[test]
    fn message_object_type_serialize() {
        assert_eq!(
            serde_json::to_string(&MessageObjectType::Message).unwrap(),
            "\"message\""
        );
    }

    #[test]
    fn message_object_type_deserialize() {
        assert_eq!(
            serde_json::from_str::<MessageObjectType>("\"message\"").unwrap(),
            MessageObjectType::Message
        );
    }
}
