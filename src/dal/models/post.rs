use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use chrono::prelude::*;

use util::time::get_now;
use dal::schema::post;
use dal::schema::post::dsl::post as all_posts;

#[derive(Serialize, Deserialize,Queryable, Debug, Clone,AsChangeset,Identifiable)]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub raw_content: String,
    pub rendered_content: String,
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
    pub published: bool,
}
impl Post {
    pub fn query_all(conn: &PgConnection) -> Vec<Post> {
        all_posts.order(post::id.desc()).load::<Post>(conn).unwrap()
    }
    pub fn query_all_published(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .filter(post::published.eq(true))
            .order(post::create_time.desc())
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn query_latest_five(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .filter(post::published.eq(true))
            .order(post::create_time.desc())
            .limit(5)
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn pagination_query(conn: &PgConnection) -> Vec<Post> {
        all_posts
            .filter(post::published.eq(true))
            .order(post::create_time.desc())
            .offset(0)
            .limit(15)
            .load::<Post>(conn)
            .expect("Error loading posts")
    }
    pub fn query_by_id(conn: &PgConnection, id: i32) -> Vec<Post> {
        all_posts
            .find(id)
            .load::<Post>(conn)
            .expect("Error finding post")

    }
    pub fn delete_with_id(conn: &PgConnection, id: i32) -> bool {
        diesel::delete(all_posts.find(id)).execute(conn).is_ok()
    }
    pub fn update_post(conn: &PgConnection, post: &Post) -> bool {
        diesel::update(post).set(post).execute(conn).is_ok()
    }
}
#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "post"]
pub struct NewPost {
    pub title: String,
    pub subtitle: String,
    pub raw_content: String,
    pub rendered_content: String,
    pub published: bool,
    #[serde(default = "get_now")]
    pub create_time: NaiveDateTime,
    #[serde(default = "get_now")]
    pub modify_time: NaiveDateTime,
}
impl NewPost {
    pub fn insert(new_post: &NewPost, conn: &PgConnection) -> bool {
        // use dal::schema::post;
        diesel::insert(new_post)
            .into(post::table)
            .execute(conn)
            .is_ok()
    }
}
// fn get_now() -> NaiveDateTime {
//     let dt = Local::now();
//     let d = NaiveDate::from_ymd(dt.year(), dt.month(), dt.day());
//     let t = NaiveTime::from_hms(dt.hour(), dt.minute(), dt.second());
//     NaiveDateTime::new(d, t)
// }

#[derive( Deserialize, Serialize)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub create_time: NaiveDateTime,
}
impl PostView {
    pub fn model_convert_to_postview(post: &Post) -> PostView {
        PostView {
            id: post.id,
            title: post.title.to_string(),
            subtitle: post.subtitle.to_string(),
            create_time: post.create_time,
        }
    }
}
// and in the structure