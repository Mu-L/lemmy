use super::check_community_content_fetchable;
use crate::http::{create_apub_response, create_apub_tombstone_response, redirect_remote_object};
use activitypub_federation::{config::Data, traits::Object};
use actix_web::{web::Path, HttpRequest, HttpResponse};
use lemmy_api_utils::context::LemmyContext;
use lemmy_apub_objects::objects::comment::ApubComment;
use lemmy_db_schema::{
  newtypes::CommentId,
  source::{comment::Comment, community::Community, post::Post},
  traits::Crud,
};
use lemmy_utils::error::LemmyResult;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CommentQuery {
  comment_id: String,
}

/// Return the ActivityPub json representation of a local comment over HTTP.
pub(crate) async fn get_apub_comment(
  info: Path<CommentQuery>,
  context: Data<LemmyContext>,
  request: HttpRequest,
) -> LemmyResult<HttpResponse> {
  let id = CommentId(info.comment_id.parse::<i32>()?);
  // Can't use CommentView here because it excludes deleted/removed/local-only items
  let comment: ApubComment = Comment::read(&mut context.pool(), id).await?.into();
  let post = Post::read(&mut context.pool(), comment.post_id).await?;
  let community = Community::read(&mut context.pool(), post.community_id).await?;
  check_community_content_fetchable(&community, &request, &context).await?;

  if !comment.local {
    Ok(redirect_remote_object(&comment.ap_id))
  } else if !comment.deleted && !comment.removed {
    create_apub_response(&comment.into_json(&context).await?)
  } else {
    create_apub_tombstone_response(comment.ap_id.clone())
  }
}
