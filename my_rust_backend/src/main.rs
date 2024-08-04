// Create modules
mod graph;
mod csv_reader;

// Import from crates
use std::collections::{BTreeMap, HashMap};
use axum::{
    extract::{Json, Extension},
    response::IntoResponse,
    routing::{get, post},
    Router,
    http::StatusCode,
};
use serde::{Deserialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};
use tracing::{info, error, Level};

// Import from modules
use csv_reader::reading_csv;
use graph::Novel;
use crate::graph::{FindNovel, Graph};

type SharedState = Arc<Mutex<AppState>>;

// This is a struct for the inputs received from the frontend,
// we get the inputs and put the values in here
#[derive(Debug, Deserialize, Clone)]
struct InputData {
    input: String,
    input2: String,
    checked: bool,
    id1: String,
    id2: String,
}

// This is a struct that holds everything, including:
// the novels, the result of shortest path, the graph which calls the functions, and a hashmap to get id by title
struct AppState {
    novels: Vec<Novel>,
    result: Vec<String>,
    titletoid: HashMap<String, u16>,
    novel_graph: BTreeMap<u16, Vec<(u16, u16)>>
}

// This function receives the input from the frontend,
// the AppState struct is passed into this, so it can have access to the graph algorithms and put the shortest path into the result vector
// it gets the data, clears the result vector,uses the algorithm depending on a boolean that the user checks and pushes the results and time into the result vector 
async fn handle_input(Json(data): Json<InputData>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
    println!("Received input: {}", data.input);
    println!("Received input: {}", data.input2);
    println!("Received input: {}", data.checked);
    println!("Received input: {}", data.id1);
    println!("Received input: {}", data.id2);
    
    let mut state = state.lock().unwrap();
    state.result.clear();

    if !data.checked {
        let (dijkstra_path2,djistratime) = state.novel_graph.dijkstra(&(state.titletoid.get(&data.input).unwrap()), &( state.titletoid.get(&data.input2).unwrap()), state.novels.clone());
        state.result.push(djistratime.to_string());
        for vertices in dijkstra_path2 {
            println!("{}: {}", state.novels[state.novels.find_novel(&vertices)].v_id, state.novels[state.novels.find_novel(&vertices)].title);
            println!("{}\n", state.novels[state.novels.find_novel(&vertices)]);
            let result = state.novels[state.novels.find_novel(&vertices)].title.to_string();
            println!("{}", result);
            state.result.push(result);
        }
    } else {
        let (bellmanford,bellmanfordtime) = state.novel_graph.bellman_ford(&(state.titletoid.get(&data.input).unwrap()), &( state.titletoid.get(&data.input2).unwrap()), state.novels.clone());
        state.result.push(bellmanfordtime.to_string());
        for vertices in bellmanford {
            println!("{}: {}", state.novels[state.novels.find_novel(&vertices)].v_id, state.novels[state.novels.find_novel(&vertices)].title);
            println!("{}\n", state.novels[state.novels.find_novel(&vertices)]);
            let result = state.novels[state.novels.find_novel(&vertices)].title.to_string();
            println!("{}", result);
            state.result.push(result);
        }
    }
    StatusCode::OK.into_response()
}

#[tokio::main]
async fn main() {
    // Get a vector of ONLY SFW novels and a map of novel titles to their respective v_ids
    let (novels, titles_to_ids) = reading_csv();

    let mut num_data_points = 0;
    for novel in &novels {
        num_data_points += novel.tag_cont.len() + novel.seiyuu.len() + novel.staff.len();
    }
    println!("Number of data points in sfw novels ONLY: {}", num_data_points);

    let novel_graph = get_weights(&novels).await;

    // setup for frontend and backend and setup corsrequest which allows for data to be sent from any origin any method
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());
    

    // Shared state with initial data it initializes and holds the novels, graphs, and results
    let shared_state = Arc::new(Mutex::new(AppState{novels, result: vec![], titletoid: titles_to_ids, novel_graph}));

    // Initialize the result vector to make sure its empty and default is 0 for time 
    clearresult(&shared_state);
    addresult(&shared_state, "0".to_string());

    // Define routes for the backend server and functions. get requests send data to frontend, post requests get data from frontend. they each call a function
    // for what to do. This sets up the backend rust server on localhost 3000
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

//add to result vector called from addresult
fn resultadd(state: &mut AppState, entry: String) {
    state.result.push(entry);
}

//clear result vector called from clearresult 
fn resultclear(state: &mut AppState) {
    state.result.clear();
}

//clear result vector
fn clearresult(shared_state: &SharedState) {
    let mut state = shared_state.lock().unwrap();
    resultclear(&mut state);
}

//add to result vector
fn addresult(shared_state: &SharedState, entry: String) {
    let mut state = shared_state.lock().unwrap();
    resultadd(&mut state, entry);
}

//send the results after calculations from the algorithms to the frontend, it has the state with all the vectors and sends it to the frontend as a response
async fn get_result(
    Extension(state): Extension<SharedState>,) -> impl IntoResponse {
    match state.lock() {
        Ok(state) => {
            info!("Successfully fetched result data");
            Json(state.result.clone()).into_response()
        },
        Err(poisoned) => {
            error!("Failed to acquire lock: {:?}", poisoned);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to acquire lock").into_response()
        }
    }
}

//send the initial vector with all the novels in our graph to the frontend, it is then displayed as the lists
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
async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u16)>> {
    let mut graph= BTreeMap::new();

    for from_index in 0..novels.len() {
        // Comment this out if you think this is causing the program to take longer to finish.
        println!("{}: {}", novels[from_index].v_id, novels[from_index].title);
        for to_index in from_index + 1..novels.len() {
            let from_id = novels[from_index].v_id;
            let to_id = novels[to_index].v_id;

            // Ensure that both the 'from' and 'to' novels are in the graph
            if !graph.contains_key(&from_id) {
                graph.insert(from_id, vec![]);
            }
            if !graph.contains_key(&to_id) {
                graph.insert(to_id, vec![]);
            }

            let weight = novels[from_index].comparing(&novels[to_index]);
            // Create an undirected edge, if the weight is within bounds
            if weight > 0 && weight < 70 {
                graph.get_mut(&from_id).unwrap().push((to_id, weight));
                graph.get_mut(&to_id).unwrap().push((from_id, weight));
            }
        }
    }
    graph
}
