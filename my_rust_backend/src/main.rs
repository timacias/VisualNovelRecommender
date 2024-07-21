// Add modules
mod test;
mod csv_reader;

use axum::{
    extract::{Json, Extension},
    response::IntoResponse,
    routing::{get, post},
    Router,
    http::StatusCode,
};
use serde::{/*Serialize,*/ Deserialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};
use tracing::{info, error, Level};
// use tracing_subscriber::FmtSubscriber;

// Import from modules
use csv_reader::reading_csv;
use test::Novel;

type SharedState = Arc<Mutex<Vec<Novel>>>;

#[derive(Deserialize)]
struct InputData {
    input: String,
}

async fn handle_input(Json(data): Json<InputData>) -> impl IntoResponse {
    println!("Received input: {}", data.input);
    "Input received"
}

#[tokio::main]
async fn main() {
    // Get a vector of Novels from the vn database
    let novels = reading_csv();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // CORS configuration to allow requests from any origin
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    // TODO: Tim: I changed the attributes of the Novel struct
    //         try using novels : Vec<Novel> instead.
    // Shared state with initial data
    let shared_state = Arc::new(Mutex::new(/*vec![*/
        novels
        // Novel {
        //     title: String::from("Person X"),
        //     v_id: 36,
        //     favourite_food: Some(String::from("Pizza")),
        // },
        // Novel {
        //     title: String::from("Person B"),
        //     v_id: 5,
        //     favourite_food: Some(String::from("Broccoli")),
        // },
        // Novel {
        //     title: String::from("Person C"),
        //     v_id: 100,
        //     favourite_food: None,
        // }
    /*]*/));

    // Define routes and apply middleware
    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .route("/input", post(handle_input))
        .layer(cors)
        .layer(Extension(shared_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_people(
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    match state.lock() {
        Ok(novels) => {
            info!("Successfully fetched people data");
            Json(novels.clone()).into_response()
        },
        Err(e) => {
            error!("Failed to acquire lock: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to acquire lock").into_response()
        }
    }
}
