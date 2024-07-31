// Add modules
mod test;
mod csv_reader;

use std::collections::BTreeMap;
use axum::{
    extract::{Json, Extension, Query},
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
use crate::test::FindNovel;

type SharedState = Arc<Mutex<AppState>>;

#[derive(Debug, Deserialize, Clone)]
struct InputData {
    input: String,
    input2: String,
    checked: bool,
}

struct AppState {
    novels: Vec<Novel>,
    result: Vec<String>,
}



async fn handle_input(Json(data): Json<InputData>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
    println!("Received input: {}", data.input);
    println!("Received input: {}", data.input2);
    println!("Received input: {}", data.checked);

    let mut state = state.lock().unwrap();
    state.result.clear();
    state.result.push(data.input.clone());
    state.result.push(data.input2.clone());
    state.result.push(data.checked.to_string());

    "Input received"
    
}

#[tokio::main]
async fn main() {
    // Get a vector of ONLY SFW novels
    let novels = reading_csv();

    
    let mut num_data_points = 0;
    for novel in &novels {
        num_data_points += novel.tag_cont.len();
    }
    println!("Number of data points in sfw novels ONLY: {}", num_data_points);

    /* let test_novel1: usize = find_novel(&novels, 11).await; // Fate/Stay Night
    let test_novel2: usize = find_novel(&novels, 50).await; // Fate/Stay Night Ataraxia - Direct Sequel
    novels[test_novel1].print_novel();
    novels[test_novel2].print_novel();
    println!("{}", novels[test_novel1].comparing(&novels[test_novel2]));*/

    /* let test_novel3: usize = find_novel(&sfw_novels, 971).await;
    if test_novel3 < sfw_novels.len() {
        sfw_novels[test_novel3].print_novel();
    } */

    let novel_graph = get_weights(&novels).await;

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
    let shared_state = Arc::new(Mutex::new(AppState{/*vec![*/
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
                                           /*]*/,result: vec![],}));
    

    clearresult(&shared_state);
    addresult(&shared_state, "test".to_string());
    

    // Define routes and apply middleware
    let app = Router::new()
        .route("/", get(root))
        .route("/people", get(get_people))
        .route("/input", post(handle_input))
        .route("/result", get(get_result))
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
    "Hello World"
}


fn resultadd(state: &mut AppState, entry: String) {
    state.result.push(entry);
}

fn resultclear(state: &mut AppState) {
    state.result.clear();
}

fn clearresult(shared_state: &SharedState) {
    let mut state = shared_state.lock().unwrap();
    resultclear(&mut state);
}

fn addresult(shared_state: &SharedState, entry: String) {
    let mut state = shared_state.lock().unwrap();
    resultadd(&mut state, entry);
}



async fn get_result( 
    Extension(state): Extension<SharedState>,) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    // resultclear(&mut state);
    // resultadd(&mut state, String::from("test"));
    

    Json(state.result.clone())
}

async fn get_people(
    Extension(state): Extension<SharedState>,) -> impl IntoResponse {
    match state.lock() {
        Ok(novels) => {
            info!("Successfully fetched people data");
            Json(novels.novels.clone()).into_response()
        },
        Err(e) => {
            error!("Failed to acquire lock: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to acquire lock").into_response()
        }
    }
}

// Returns a TreeMap of <v_id, Vec<v_id, weight>>
async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u8)>> { // TODO: THIS WILL RETURN SOMETHING
    let mut graph= BTreeMap::new();
    for i in 0..novels.len() {
        let mut adj_list = Vec::new();
        println!("{}, {}", novels[i].title, novels[i].v_id);
        for j in i + 1..novels.len() {
            adj_list.push((novels[j].v_id, novels[i].comparing(&novels[j])));
        }
        graph.insert(novels[i].v_id, adj_list);
    }
    graph
}