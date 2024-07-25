use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client, Database, Collection, bson::{doc, Bson, from_bson, to_bson}};
use std::{env, error::Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use futures_util::stream::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Todo API!")
}

async fn create_todo(
    todo: web::Json<Todo>,
    db: web::Data<Database>
) -> impl Responder {
    let collection: Collection<bson::Document> = db.collection("todos");
    let mut todo = todo.into_inner();
    todo.id = Some(Uuid::new_v4());
    todo.created_at = Utc::now();
    todo.updated_at = Utc::now();
    let todo_doc = match to_bson(&todo) {
        Ok(bson) => match bson.as_document() {
            Some(doc) => doc.clone(),
            None => return HttpResponse::InternalServerError().body("Failed to convert to BSON document"),
        },
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to serialize todo: {}", e)),
    };

    match collection.insert_one(todo_doc, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to insert todo: {}", e)),
    }
}

async fn get_todos(db: web::Data<Database>) -> impl Responder {
    let collection: Collection<bson::Document> = db.collection("todos");
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to execute find query: {}", e)),
    };

    let mut todos = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        match from_bson::<Todo>(Bson::Document(result)) {
            Ok(todo) => todos.push(todo),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse todo: {}", e)),
        }
    }

    HttpResponse::Ok().json(todos)
}

async fn update_todo(
    id: web::Path<Uuid>,
    updated_todo: web::Json<Todo>,
    db: web::Data<Database>
) -> impl Responder {
    let collection: Collection<bson::Document> = db.collection("todos");
    let filter = doc! { "_id": id.to_string() };
    let mut updated_todo = updated_todo.into_inner();
    updated_todo.updated_at = Utc::now();
    let todo_doc = match to_bson(&updated_todo) {
        Ok(bson) => match bson.as_document() {
            Some(doc) => doc.clone(),
            None => return HttpResponse::InternalServerError().body("Failed to convert to BSON document"),
        },
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to serialize todo: {}", e)),
    };
    let update = doc! { "$set": todo_doc };

    match collection.update_one(filter, update, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to update todo: {}", e)),
    }
}

async fn delete_todo(
    id: web::Path<Uuid>,
    db: web::Data<Database>
) -> impl Responder {
    let collection: Collection<bson::Document> = db.collection("todos");
    let filter = doc! { "_id": id.to_string() };

    match collection.delete_one(filter, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to delete todo: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
    let client = Client::with_uri_str(&client_uri).await?;
    let db = client.database("todos");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(index))
            .route("/todos", web::post().to(create_todo))
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
