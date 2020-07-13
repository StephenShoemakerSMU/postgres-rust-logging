use std::time::SystemTime;

#[derive(Queryable)]
pub struct Log{
    pub log_id: i32,
    pub status_code: Option<i32>,
    pub message: Option<String>,
    pub time: Option<SystemTime>,
    pub resolved: Option<bool>
}