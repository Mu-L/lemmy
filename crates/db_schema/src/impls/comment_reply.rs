use crate::{
  diesel::OptionalExtension,
  newtypes::{CommentId, CommentReplyId, PersonId},
  source::comment_reply::{CommentReply, CommentReplyInsertForm, CommentReplyUpdateForm},
  traits::Crud,
  utils::{get_conn, DbPool},
};
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use lemmy_db_schema_file::schema::comment_reply;
use lemmy_utils::error::{LemmyErrorExt, LemmyErrorType, LemmyResult};

impl Crud for CommentReply {
  type InsertForm = CommentReplyInsertForm;
  type UpdateForm = CommentReplyUpdateForm;
  type IdType = CommentReplyId;

  async fn create(
    pool: &mut DbPool<'_>,
    comment_reply_form: &Self::InsertForm,
  ) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;

    // since the return here isnt utilized, we dont need to do an update
    // but get_result doesn't return the existing row here
    insert_into(comment_reply::table)
      .values(comment_reply_form)
      .on_conflict((comment_reply::recipient_id, comment_reply::comment_id))
      .do_update()
      .set(comment_reply_form)
      .get_result::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntCreateCommentReply)
  }

  async fn update(
    pool: &mut DbPool<'_>,
    comment_reply_id: CommentReplyId,
    comment_reply_form: &Self::UpdateForm,
  ) -> LemmyResult<Self> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(comment_reply::table.find(comment_reply_id))
      .set(comment_reply_form)
      .get_result::<Self>(conn)
      .await
      .with_lemmy_type(LemmyErrorType::CouldntUpdateCommentReply)
  }
}

impl CommentReply {
  pub async fn mark_all_as_read(
    pool: &mut DbPool<'_>,
    for_recipient_id: PersonId,
  ) -> LemmyResult<Vec<CommentReply>> {
    let conn = &mut get_conn(pool).await?;
    diesel::update(
      comment_reply::table
        .filter(comment_reply::recipient_id.eq(for_recipient_id))
        .filter(comment_reply::read.eq(false)),
    )
    .set(comment_reply::read.eq(true))
    .get_results::<Self>(conn)
    .await
    .with_lemmy_type(LemmyErrorType::CouldntMarkCommentReplyAsRead)
  }

  pub async fn read_by_comment(
    pool: &mut DbPool<'_>,
    for_comment_id: CommentId,
  ) -> LemmyResult<Option<Self>> {
    let conn = &mut get_conn(pool).await?;
    comment_reply::table
      .filter(comment_reply::comment_id.eq(for_comment_id))
      .first(conn)
      .await
      .optional()
      .with_lemmy_type(LemmyErrorType::NotFound)
  }

  pub async fn read_by_comment_and_person(
    pool: &mut DbPool<'_>,
    for_comment_id: CommentId,
    for_recipient_id: PersonId,
  ) -> LemmyResult<Option<Self>> {
    let conn = &mut get_conn(pool).await?;
    comment_reply::table
      .filter(comment_reply::comment_id.eq(for_comment_id))
      .filter(comment_reply::recipient_id.eq(for_recipient_id))
      .first(conn)
      .await
      .optional()
      .with_lemmy_type(LemmyErrorType::NotFound)
  }
}
