use diesel::{prelude::*, insert_into};
use diesel::{PgConnection, RunQueryDsl, result::Error};

use crate::models::{Task, NewTask};

pub struct TaskCreationData {
    pub title: String,
    pub details: String,
    pub status_code: u8
}

pub async fn get_task(conn: &mut PgConnection, id_filter: i32) -> Result<Task, Error> {
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

    insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
}