use actix_web::{
    get, post, patch, delete,
    Responder,
    HttpResponse, web::{Data, Json, Query}
};
use serde::{Deserialize};
use serde_json::json;

use crate::{db::{self, TaskCreationData, TaskUpdateData}, DbPool};

#[derive(Deserialize)]
pub struct CreateTaskData {
    title: String,
    details: String,
    status: String
}

#[derive(Deserialize)]
pub struct TaskActionQuery {
    id: i32
}

#[derive(Deserialize)]
pub struct UpdateTaskData {
    title: Option<String>,
    details: Option<String>,
    status: Option<String>
}

#[get("/task")]
async fn index_tasks(pool: Data<DbPool>) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("Failed to get connection.");
    
    let result = db::index_tasks(&mut conn).await;

    match result {
        Ok(tasks) => {
            HttpResponse::Ok()
                .json(json!({
                    "tasks": tasks
                }))
        },
        Err(err) => {
            eprintln!("Error indexing tasks: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({
                    "message": "Failed to index tasks, couldn't fetch database."
                }))
        }
    }
}

#[post("/task")]
async fn create_task(pool: Data<DbPool>, data: Json<CreateTaskData>) -> impl Responder {
    let mut conn = pool
        .get()
        .expect("Failed to get connection.");

    if !vec!["open", "closed", "paused"].contains(&data.status.as_str()) {
        return HttpResponse::BadRequest()
            .json(json!({
                "message": "status field must be 'open', 'closed' or 'paused."
            }))
    };

    let status_code = match data.status.as_str() {
        "open" => 0,
        "closed" => 1,
        "paused" => 2,
        _ => unreachable!()
    };

    let data = TaskCreationData {
        title: data.title.clone(),
        details: data.details.clone(),
        status_code
    };

    let result = db::create_task(&mut conn, data).await;

    match result {
        Ok(task) => {
            HttpResponse::Created()
                .json(json!({
                    "task": task
                }))
        },
        Err(err) => {
            eprintln!("Failed to create task: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({
                    "message": "Failed to create task in database."
                }))
        }
    }
}

#[patch("/task")]
async fn update_task(pool: Data<DbPool>, query: Query<TaskActionQuery>, data: Json<UpdateTaskData>) -> impl Responder {
    let mut conn = pool.get()
        .expect("Failed  to get connection.");
    
    if query.id < 1 {
        return HttpResponse::BadRequest()
            .json(json!({
                "message": "'id' query must be a positive integer."
            }))
    }

    let status_code = match &data.status {
        Some(status) => {
            match status.as_str() {
                "open" => Some(0),
                "closed" => Some(1),
                "paused" => Some(2),
                _ => unreachable!()
            }


        },
        None => None
    };

    let data = TaskUpdateData {
        title: data.title.clone(),
        details: data.details.clone(),
        status_code
    };

    let result = db::update_task(&mut conn, &query.id, data).await;

    match result {
        Ok(task) => {
            HttpResponse::Ok()
                .json(json!({
                    "task": task
                }))
        },
        Err(err) => {
            eprintln!("Failed to update task: {:?}", err);
            HttpResponse::NotModified()
                .json(json!({
                    "message": "Failed to update task, couldn't update database item."
            }))
        }
    }
}

#[delete("/task")]
async fn delete_task(pool: Data<DbPool>, query: Query<TaskActionQuery>) -> impl Responder {
    if query.id < 1 {
        return HttpResponse::BadRequest()
            .json(json!({
                "message": "'id' query must be a positive integer."
            }))
    }

    let mut conn = pool.get()
        .expect("Failed to get database connection.");

    let result = db::delete_task(&mut conn, &query.id).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "message": format!("Task successfully deleted. (id: {})", &query.id)
                }))
        },
        Err(err) => {
            eprint!("Failed to delete task from database: {:?}", err);
            HttpResponse::InternalServerError()
                .json(json!({
                    "message": "Failed to delete task."
                }))
        }
    }
}