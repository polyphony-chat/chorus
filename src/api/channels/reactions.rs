use crate::types;

/**
Extends the [`types::Reaction`] struct with useful metadata.
 */
pub struct ReactionMeta {
    pub message_id: types::Snowflake,
    pub reaction: types::Reaction,
}
