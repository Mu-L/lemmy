use crate::newtypes::CustomEmojiId;
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::custom_emoji_keyword;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
  feature = "full",
  derive(Queryable, Selectable, Associations, Identifiable)
)]
#[cfg_attr(feature = "full", diesel(table_name = custom_emoji_keyword))]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::custom_emoji::CustomEmoji))
)]
#[cfg_attr(feature = "full", diesel(primary_key(custom_emoji_id, keyword)))]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// A custom keyword for an emoji.
pub struct CustomEmojiKeyword {
  pub custom_emoji_id: CustomEmojiId,
  pub keyword: String,
}

#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(feature = "full", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = custom_emoji_keyword))]
pub struct CustomEmojiKeywordInsertForm {
  pub custom_emoji_id: CustomEmojiId,
  pub keyword: String,
}
