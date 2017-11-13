use rocket_contrib::Json;
use chrono::NaiveDateTime;

use dal::diesel_pool::DB;
use dal::models::visitor_log::*;

#[get("/admin/log/daily_pv")]
pub fn count_daily_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_daily_page_view(db.conn());
    Json(results)
}

#[get("/admin/log/monthly_pv")]
pub fn count_monthly_page_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_monthly_page_view(db.conn());
    Json(results)
}


#[get("/admin/log/daily_uv")]
pub fn count_daily_user_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_daily_user_view(db.conn());
    Json(results)
}

#[get("/admin/log/monthly_uv")]
pub fn count_monthly_user_view(db: DB) -> Json<Vec<(NaiveDateTime, i64)>> {
    let results = VisitorLog::count_monthly_user_view(db.conn());
    Json(results)
}
