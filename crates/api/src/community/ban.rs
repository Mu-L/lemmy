use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_common::{
  community::{BanFromCommunity, BanFromCommunityResponse},
  context::LemmyContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::{
    check_community_mod_action,
    check_expire_time,
    remove_or_restore_user_data_in_community,
  },
};
use lemmy_db_schema::{
  source::{
    community::{
      Community,
      CommunityFollower,
      CommunityFollowerForm,
      CommunityPersonBan,
      CommunityPersonBanForm,
    },
    local_user::LocalUser,
    mod_log::moderator::{ModBanFromCommunity, ModBanFromCommunityForm},
  },
  traits::{Bannable, Crud, Followable},
};
use lemmy_db_views::structs::{LocalUserView, PersonView};
use lemmy_utils::{
  error::{LemmyErrorExt, LemmyErrorType, LemmyResult},
  utils::validation::is_valid_body_field,
};

pub async fn ban_from_community(
  data: Json<BanFromCommunity>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<BanFromCommunityResponse>> {
  let banned_person_id = data.person_id;
  let expires = check_expire_time(data.expires)?;
  let community = Community::read(&mut context.pool(), data.community_id).await?;

  // Verify that only mods or admins can ban
  check_community_mod_action(
    &local_user_view.person,
    &community,
    false,
    &mut context.pool(),
  )
  .await?;

  LocalUser::is_higher_mod_or_admin_check(
    &mut context.pool(),
    data.community_id,
    local_user_view.person.id,
    vec![data.person_id],
  )
  .await?;

  if let Some(reason) = &data.reason {
    is_valid_body_field(reason, false)?;
  }

  let community_user_ban_form = CommunityPersonBanForm {
    community_id: data.community_id,
    person_id: data.person_id,
    expires: Some(expires),
  };

  if data.ban {
    CommunityPersonBan::ban(&mut context.pool(), &community_user_ban_form)
      .await
      .with_lemmy_type(LemmyErrorType::CommunityUserAlreadyBanned)?;

    // Also unsubscribe them from the community, if they are subscribed
    let community_follower_form = CommunityFollowerForm::new(data.community_id, banned_person_id);
    CommunityFollower::unfollow(&mut context.pool(), &community_follower_form)
      .await
      .ok();
  } else {
    CommunityPersonBan::unban(&mut context.pool(), &community_user_ban_form)
      .await
      .with_lemmy_type(LemmyErrorType::CommunityUserAlreadyBanned)?;
  }

  // Remove/Restore their data if that's desired
  if data.remove_or_restore_data.unwrap_or(false) {
    let remove_data = data.ban;
    remove_or_restore_user_data_in_community(
      data.community_id,
      local_user_view.person.id,
      banned_person_id,
      remove_data,
      &data.reason,
      &mut context.pool(),
    )
    .await?;
  };

  // Mod tables
  let form = ModBanFromCommunityForm {
    mod_person_id: local_user_view.person.id,
    other_person_id: data.person_id,
    community_id: data.community_id,
    reason: data.reason.clone(),
    banned: Some(data.ban),
    expires,
  };

  ModBanFromCommunity::create(&mut context.pool(), &form).await?;

  let person_view = PersonView::read(&mut context.pool(), data.person_id, false).await?;

  ActivityChannel::submit_activity(
    SendActivityData::BanFromCommunity {
      moderator: local_user_view.person,
      community_id: data.community_id,
      target: person_view.person.clone(),
      data: data.0.clone(),
    },
    &context,
  )?;

  Ok(Json(BanFromCommunityResponse {
    person_view,
    banned: data.ban,
  }))
}
