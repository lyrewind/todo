use diesel::prelude::*;
use serde::Serialize;

use crate::schema::tasks;

#[derive(Queryable, Serialize, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub details: String,
    pub status_code: i32
}

#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct PartialTask {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub details: Option<String>,
    pub status_code: Option<i32>
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub details: &'a str,
    pub status_code: &'a i32
}