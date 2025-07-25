use activitypub_federation::config::Data;
use actix_web::web::Json;
use lemmy_api_utils::{
  build_response::build_post_response,
  context::LemmyContext,
  send_activity::{ActivityChannel, SendActivityData},
  utils::check_community_mod_action,
};
use lemmy_db_schema::{
  source::{
    mod_log::moderator::{ModLockPost, ModLockPostForm},
    post::{Post, PostUpdateForm},
  },
  traits::Crud,
};
use lemmy_db_views_local_user::LocalUserView;
use lemmy_db_views_post::{
  api::{LockPost, PostResponse},
  PostView,
};
use lemmy_utils::error::LemmyResult;

pub async fn lock_post(
  data: Json<LockPost>,
  context: Data<LemmyContext>,
  local_user_view: LocalUserView,
) -> LemmyResult<Json<PostResponse>> {
  let post_id = data.post_id;
  let local_instance_id = local_user_view.person.instance_id;

  let orig_post =
    PostView::read(&mut context.pool(), post_id, None, local_instance_id, false).await?;

  check_community_mod_action(
    &local_user_view,
    &orig_post.community,
    false,
    &mut context.pool(),
  )
  .await?;

  // Update the post
  let post_id = data.post_id;
  let locked = data.locked;
  let post = Post::update(
    &mut context.pool(),
    post_id,
    &PostUpdateForm {
      locked: Some(locked),
      ..Default::default()
    },
  )
  .await?;

  // Mod tables
  let form = ModLockPostForm {
    mod_person_id: local_user_view.person.id,
    post_id: data.post_id,
    locked: Some(locked),
    reason: data.reason.clone(),
  };
  ModLockPost::create(&mut context.pool(), &form).await?;

  ActivityChannel::submit_activity(
    SendActivityData::LockPost(
      post,
      local_user_view.person.clone(),
      data.locked,
      data.reason.clone(),
    ),
    &context,
  )?;

  build_post_response(&context, orig_post.community.id, local_user_view, post_id).await
}
