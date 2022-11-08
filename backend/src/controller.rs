use actix_web::{
    get, post, patch, delete,
    Responder,
    HttpResponse, web::{Data, Json}
};
use serde::{Deserialize};
use serde_json::json;

use crate::{db::{self, TaskCreationData}, DbPool};

#[derive(Deserialize)]
pub struct CreateTaskData {
    title: String,
    details: String,
    status: String
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
async fn update_task() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[delete("/task")]
async fn delete_task() -> impl Responder {
    HttpResponse::NotImplemented()
}