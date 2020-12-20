use diesel::{dsl::*, result::Error, *};
use serde::{Deserialize, Serialize};

use crate::{naive_now, schema::post_report, source::post::Post, Reportable};

#[derive(
  Identifiable, Queryable, Associations, PartialEq, Serialize, Deserialize, Debug, Clone,
)]
#[belongs_to(Post)]
#[table_name = "post_report"]
pub struct PostReport {
  pub id: i32,
  pub creator_id: i32,
  pub post_id: i32,
  pub original_post_name: String,
  pub original_post_url: Option<String>,
  pub original_post_body: Option<String>,
  pub reason: String,
  pub resolved: bool,
  pub resolver_id: Option<i32>,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Clone)]
#[table_name = "post_report"]
pub struct PostReportForm {
  pub creator_id: i32,
  pub post_id: i32,
  pub original_post_name: String,
  pub original_post_url: Option<String>,
  pub original_post_body: Option<String>,
  pub reason: String,
}

impl Reportable<PostReportForm> for PostReport {
  /// creates a post report and returns it
  ///
  /// * `conn` - the postgres connection
  /// * `post_report_form` - the filled CommentReportForm to insert
  fn report(conn: &PgConnection, post_report_form: &PostReportForm) -> Result<Self, Error> {
    use crate::schema::post_report::dsl::*;
    insert_into(post_report)
      .values(post_report_form)
      .get_result::<Self>(conn)
  }

  /// resolve a post report
  ///
  /// * `conn` - the postgres connection
  /// * `report_id` - the id of the report to resolve
  /// * `by_resolver_id` - the id of the user resolving the report
  fn resolve(conn: &PgConnection, report_id: i32, by_resolver_id: i32) -> Result<usize, Error> {
    use crate::schema::post_report::dsl::*;
    update(post_report.find(report_id))
      .set((
        resolved.eq(true),
        resolver_id.eq(by_resolver_id),
        updated.eq(naive_now()),
      ))
      .execute(conn)
  }

  /// resolve a post report
  ///
  /// * `conn` - the postgres connection
  /// * `report_id` - the id of the report to unresolve
  /// * `by_resolver_id` - the id of the user unresolving the report
  fn unresolve(conn: &PgConnection, report_id: i32, by_resolver_id: i32) -> Result<usize, Error> {
    use crate::schema::post_report::dsl::*;
    update(post_report.find(report_id))
      .set((
        resolved.eq(false),
        resolver_id.eq(by_resolver_id),
        updated.eq(naive_now()),
      ))
      .execute(conn)
  }
}