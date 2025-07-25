use crate::newtypes::LocalSiteId;
use chrono::{DateTime, Utc};
#[cfg(feature = "full")]
use lemmy_db_schema_file::schema::local_site_rate_limit;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "full", derive(Queryable, Selectable, Identifiable))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
#[cfg_attr(feature = "full", diesel(primary_key(local_site_id)))]
#[cfg_attr(
  feature = "full",
  diesel(belongs_to(crate::source::local_site::LocalSite))
)]
#[cfg_attr(feature = "full", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "ts-rs", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts-rs", ts(optional_fields, export))]
/// Rate limits for your site. Given in count / length of time.
pub struct LocalSiteRateLimit {
  pub local_site_id: LocalSiteId,
  pub message_max_requests: i32,
  pub message_interval_seconds: i32,
  pub post_max_requests: i32,
  pub post_interval_seconds: i32,
  pub register_max_requests: i32,
  pub register_interval_seconds: i32,
  pub image_max_requests: i32,
  pub image_interval_seconds: i32,
  pub comment_max_requests: i32,
  pub comment_interval_seconds: i32,
  pub search_max_requests: i32,
  pub search_interval_seconds: i32,
  pub published_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
  pub import_user_settings_max_requests: i32,
  pub import_user_settings_interval_seconds: i32,
}

#[derive(Clone, derive_new::new)]
#[cfg_attr(feature = "full", derive(Insertable))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
pub struct LocalSiteRateLimitInsertForm {
  pub local_site_id: LocalSiteId,
  #[new(default)]
  pub message_max_requests: Option<i32>,
  #[new(default)]
  pub message_interval_seconds: Option<i32>,
  #[new(default)]
  pub post_max_requests: Option<i32>,
  #[new(default)]
  pub post_interval_seconds: Option<i32>,
  #[new(default)]
  pub register_max_requests: Option<i32>,
  #[new(default)]
  pub register_interval_seconds: Option<i32>,
  #[new(default)]
  pub image_max_requests: Option<i32>,
  #[new(default)]
  pub image_interval_seconds: Option<i32>,
  #[new(default)]
  pub comment_max_requests: Option<i32>,
  #[new(default)]
  pub comment_interval_seconds: Option<i32>,
  #[new(default)]
  pub search_max_requests: Option<i32>,
  #[new(default)]
  pub search_interval_seconds: Option<i32>,
  #[new(default)]
  pub import_user_settings_max_requests: Option<i32>,
  #[new(default)]
  pub import_user_settings_interval_seconds: Option<i32>,
}

#[derive(Clone, Default)]
#[cfg_attr(feature = "full", derive(AsChangeset))]
#[cfg_attr(feature = "full", diesel(table_name = local_site_rate_limit))]
pub struct LocalSiteRateLimitUpdateForm {
  pub message_max_requests: Option<i32>,
  pub message_interval_seconds: Option<i32>,
  pub post_max_requests: Option<i32>,
  pub post_interval_seconds: Option<i32>,
  pub register_max_requests: Option<i32>,
  pub register_interval_seconds: Option<i32>,
  pub image_max_requests: Option<i32>,
  pub image_interval_seconds: Option<i32>,
  pub comment_max_requests: Option<i32>,
  pub comment_interval_seconds: Option<i32>,
  pub search_max_requests: Option<i32>,
  pub search_interval_seconds: Option<i32>,
  pub import_user_settings_max_requests: Option<i32>,
  pub import_user_settings_interval_seconds: Option<i32>,
  pub updated_at: Option<Option<DateTime<Utc>>>,
}
