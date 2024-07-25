# Overview

This project demonstrates the creation of a RESTful API for managing Todo items using Rust, Actix Web, and MongoDB. The purpose of this software is to deepen my understanding of Rust, particularly in building web services and interacting with databases.

The software allows users to create, read, update, and delete Todo items. Each Todo item includes a unique identifier, title, description, completion status, and timestamps for creation and last update. This project showcases the power of Rust in building efficient and safe web applications.

# Development Environment

- **Operating System:** Your OS (e.g., Ubuntu 20.04, Windows 10)
- **IDE:** Visual Studio Code with Rust Analyzer extension
- **Rust Version:** 1.56.0 (or your version)
- **Database:** MongoDB
- **Tools:** Cargo for Rust package management and build tool

# Useful Websites

- [Rust Programming Language](https://www.rust-lang.org/)
- [Actix Web Framework](https://actix.rs/)
- [MongoDB Rust Driver](https://docs.rs/mongodb/latest/mongodb/)
- [Serde for Serialization](https://serde.rs/)
- [UUID Library](https://docs.rs/uuid/latest/uuid/)
- [Chrono Library](https://docs.rs/chrono/latest/chrono/)

# Future Work

- Implement update and delete operations for Todo items.
- Add error handling for edge cases.
- Implement user authentication and authorization.
- Improve the API documentation and add more test cases.
- Explore deploying the application using Docker.

# Overview - Cloud Database Update

In this project, I aimed to build a robust web API using Rust and Actix Web, integrated with a cloud database to handle CRUD operations for a Todo application. The application showcases my ability to interact with a cloud database and implement essential features such as creating, updating, deleting, and retrieving data.

The software allows users to manage their tasks effectively by interacting with a MongoDB cloud database. Through this project, I explored Rust's concurrency features, Actix Web framework for building asynchronous web services, and MongoDB for storing and managing data.

The purpose of this software is to demonstrate practical experience with Rust and cloud databases, enhancing my skills in web development, asynchronous programming, and database management.

# Cloud Database

I used MongoDB Atlas for the cloud database, providing a scalable and flexible NoSQL database service. The database setup includes a collection named `todos` to store task data.

The `todos` collection in MongoDB contains the following fields:

- `_id`: A unique identifier for each task, generated as a UUID.
- `title`: The title of the task.
- `description`: A detailed description of the task.
- `completed`: A boolean indicating whether the task is completed.
- `created_at`: The timestamp when the task was created.
- `updated_at`: The timestamp when the task was last updated.

# Development Environment

- **Programming Language**: Rust
- **Web Framework**: Actix Web
- **Database**: MongoDB
- **Libraries**: `mongodb`, `uuid`, `chrono`, `serde`, `serde_json`, `actix-web`, `futures-util`

The development environment consists of the Rust programming language for building the backend, Actix Web for handling HTTP requests, and MongoDB for data storage. The project leverages various libraries to handle asynchronous operations, data serialization, and UUID generation.

# Useful Websites

- [MongoDB Atlas Documentation](https://www.mongodb.com/cloud/atlas/docs)
- [Actix Web Documentation](https://actix.rs/docs/)
- [Rust Programming Language](https://www.rust-lang.org/)
- [UUID Crate Documentation](https://docs.rs/uuid/latest/uuid/)
- [Serde Documentation](https://serde.rs/)

# Future Work

- Implement user authentication to secure access to the API.
- Add notifications for changes in the cloud database.
- Enhance error handling and validation for API endpoints.
- Optimize performance and scalability of the application.
- Expand the API with additional features such as task prioritization and deadlines.
