use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
/// Like [Option], but with **different deserialization approach**:
///
/// - [Nullable::Null] deserialized as `"null"`.
/// - [Nullable::Some] deserialized transparently as `T`.
pub enum Nullable<T> {
    /// Same as [None].
    #[serde(rename = "null")]
    Null,

    #[serde(untagged)]
    Some(T),
}
