use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client, Database, bson::{doc, from_bson, to_bson}};
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
    let collection = db.collection("todos");

    // Convert Json<Todo> to Todo
    let mut todo = todo.into_inner();
    todo.id = Some(Uuid::new_v4()); // Generate a new UUID

    // Create a BSON document from the Todo struct
    let todo_doc = to_bson(&todo).unwrap().as_document().unwrap().clone();

    // Insert the document into the "todos" collection
    match collection.insert_one(todo_doc, None).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to insert todo: {}", e)),
    }
}

async fn get_todos(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection("todos");

    // Find all documents in the "todos" collection
    let mut cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to execute find query: {}", e)),
    };

    // Process the cursor asynchronously using StreamExt
    let mut todos = Vec::new();
    while let Some(result) = cursor.try_next().await.unwrap() {
        match bson::from_document::<Todo>(result) {
            Ok(todo) => todos.push(todo),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to parse todo: {}", e)),
        }
    }

    HttpResponse::Ok().json(todos)
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load MongoDB URI from environment variable
    let client_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set");
    let client = Client::with_uri_str(&client_uri).await?;

    // Get the database "todos"
    let db = client.database("todos");

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(index))
            .route("/todos", web::post().to(create_todo))
            .route("/todos", web::get().to(get_todos)) // GET endpoint for todos
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
