use activitypub_federation::config::Data;
use actix_web::web::{Json, Query};
use lemmy_api_common::{
  context::LemmyContext,
  site::{ListRegistrationApplications, ListRegistrationApplicationsResponse},
  utils::is_admin,
};
use lemmy_db_views::{
  registration_applications::registration_application_view::RegistrationApplicationQuery,
  structs::{LocalUserView, SiteView},
};
use lemmy_utils::error::LemmyResult;

/// Lists registration applications, filterable by undenied only.
pub async fn list_registration_applications(
  data: Query<ListRegistrationApplications>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<ListRegistrationApplicationsResponse>> {
  let local_site = SiteView::read_local(&mut context.pool()).await?.local_site;

  // Make sure user is an admin
  is_admin(&local_user_view)?;

  let unread_only = data.unread_only.unwrap_or_default();
  let verified_email_only = local_site.require_email_verification;

  let page = data.page;
  let limit = data.limit;
  let registration_applications = RegistrationApplicationQuery {
    unread_only,
    verified_email_only,
    page,
    limit,
  }
  .list(&mut context.pool())
  .await?;

  Ok(Json(ListRegistrationApplicationsResponse {
    registration_applications,
  }))
}
