use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, result::Error};

use crate::models::{Task, NewTask, PartialTask};

pub struct TaskCreationData {
    pub title: String,
    pub details: String,
    pub status_code: u8
}

pub struct TaskUpdateData {
    pub title: Option<String>,
    pub details: Option<String>,
    pub status_code: Option<i32>
}

pub async fn get_task(conn: &mut PgConnection, id_filter: &i32) -> Result<Task, Error> {
    use crate::schema::tasks::dsl::*;

    let result = tasks
        .filter(id.eq(id_filter))
        .load::<Task>(conn);
    
    match result {
        Ok(task_items) => {
            match task_items.first() {
                Some(task) => Ok(task.clone()),
                None =>  Err(Error::NotFound)
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch tasks from database: {:?}", err);
            Err(err)
        }
    }
}

pub async fn index_tasks(conn: &mut PgConnection) -> Result<Vec<Task>, Error> {
    use crate::schema::tasks::dsl::*;

    tasks
        .limit(10)
        .load::<Task>(conn)
}

pub async fn create_task(conn: &mut PgConnection, data: TaskCreationData) -> Result<Task, Error> {
    use crate::schema::tasks;

    let new_task = NewTask {
        title: &data.title,
        details: &data.details,
        status_code: &data.status_code.into()
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
}

pub async fn delete_task(conn: &mut PgConnection, id_filter: &i32) -> Result<usize, Error> {
    use crate::schema::tasks::dsl::*;

    diesel::delete(tasks.filter(id.eq(id_filter)))
        .execute(conn)
}

pub async fn update_task(conn: &mut PgConnection, id_filter: &i32, data: TaskUpdateData) -> Result<Task, Error> {
    use crate::schema::tasks::dsl::*;

    let task = PartialTask {
        id: Some(*id_filter),
        title: data.title,
        details: data.details,
        status_code: data.status_code
    };

    diesel::update(tasks)
        .set(task)
        .get_result(conn)
}