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
use std::mem::swap;
// use tracing_subscriber::FmtSubscriber;
use lazy_static::lazy_static;
// Import from modules
use csv_reader::reading_csv;
use test::Novel;
use crate::test::{FindNovel, Graph};

type SharedState = Arc<Mutex<AppState>>;


#[derive(Debug, Deserialize, Clone)]

struct InputData {
    input: String,
    input2: String,
    checked: bool,
    id1: String,
    id2: String,
}

struct AppState {
    novels: Vec<Novel>,
    result: Vec<String>,
    novel_graph: BTreeMap<u16, Vec<(u16, u8)>>
}


async fn handle_input(Json(data): Json<InputData>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
    println!("Received input: {}", data.input);
    println!("Received input: {}", data.input2);
    println!("Received input: {}", data.checked);
    println!("Received input: {}", data.id1);
    println!("Received input: {}", data.id2);

    let novelid1 = &data.id1[1..].to_string();
    let novelid2 = &data.id2[1..].to_string();
    let intnovelid1: u16 = match novelid1.parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing novelid1: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid id1 format").into_response();
        }
    };
    let intnovelid2: u16 = match novelid2.parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing novelid2: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid id2 format").into_response();
        }
    };
    println!("{}", intnovelid1);
    println!("{}", intnovelid2);

    
    let mut state = match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(), 
    };

    state.result.clear();
    if !data.checked {
        let dijkstra_path2 = state.novel_graph.dijkstra(&(intnovelid1), &(intnovelid2), state.novels.clone());
        for vertices in dijkstra_path2 {
            println!("{}: {}", state.novels[state.novels.find_novel(&vertices)].v_id, state.novels[state.novels.find_novel(&vertices)].title);
            println!("{}", state.novels[state.novels.find_novel(&vertices)]);
            println!();
            let result = state.novels[state.novels.find_novel(&vertices)].title.to_string();
            println!("{}", result);
            state.result.push(result);
        }
    } else {
       
    }

    StatusCode::OK.into_response()

}

#[tokio::main]
async fn main() {
    // Get a vector of ONLY SFW novels
    let novels = reading_csv();


    let mut num_data_points = 0;
    for novel in &novels {
        num_data_points += novel.tag_cont.len() + novel.seiyuu.len();
    }
    println!("Number of data points in sfw novels ONLY: {}", num_data_points);

    /* let test_novel1: usize = find_novel(&novels, 11).await; // Fate/Stay Night
    let test_novel2: usize = find_novel(&novels, 50).await; // Fate/Stay Night Ataraxia - Direct Sequel
    novels[test_novel1].print_novel();
    novels[test_novel2].print_novel();
    println!("{}", novels[test_novel1].comparing(&novels[test_novel2]));*/
    let novel_graph = get_weights(&novels).await;

    /* let test_novel3: usize = find_novel(&sfw_novels, 971).await;
    if test_novel3 < sfw_novels.len() {
        sfw_novels[test_novel3].print_novel();
    } */
    //let dijkstra_path1 = novel_graph.dijkstra(&(19119u16), &(14908u16), novels.clone());
    // Fate/EXTELLA being compared to Code:Realize -> NO PATH FOUND

    //let dijkstra_path2 = novel_graph.dijkstra(&(18160u16), &(14908u16), novels.clone());
    // Collar X Malice being compared to Code:Realized -> PATH FOUND BUT THERE ALSO MIGHT BE AN EDGE BETWEEN THE TWO IF WEIGHT > 112

    // println!();
    // println!("DIJKSTRA_PATH1");
    // if dijkstra_path1.len() == 1{
    //     println!("No path found!!!!");
    //     println!();
    // }
    // else {
    //     for vertices in dijkstra_path1{
    //         println!("{}: {}", novels[novels.find_novel(&vertices)].v_id, novels[novels.find_novel(&vertices)].title);
    //         println!("{}", novels[novels.find_novel(&vertices)]);
    //         println!();
    //     }
    // }


    // println!("DIJKSTRA_PATH2");
    // if dijkstra_path2.len() == 1{
    //     println!("No path found!!!!");
    // }
    // else{
    //     for vertices in dijkstra_path2{
    //         println!("{}: {}", novels[novels.find_novel(&vertices)].v_id, novels[novels.find_novel(&vertices)].title);
    //         println!("{}", novels[novels.find_novel(&vertices)]);
    //         println!();
    //     }
    // }


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
    let shared_state = Arc::new(Mutex::new(AppState{novels,result: vec![],novel_graph,}));


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
 // TODO: THIS WILL RETURN SOMETHING
// async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u8)>> { // TODO: THIS WILL RETURN SOMETHING
//     let mut graph= BTreeMap::new();
//     for from in 0..novels.len() {
//         println!("{}, {}", novels[from].title, novels[from].v_id);
//         for to in from + 1..novels.len() {
//             // If the current node does not exist in the graph, add it
//             if !graph.contains_key(&novels[from].v_id) {
//                 graph.insert(novels[from].v_id, vec![]);
//             }
//             let weight = novels[from].comparing(&novels[to]);
//             // Only add an edge to the graph if two novels have at least one similarity
//             if weight < 126 {
//                 // Push the edge to the current node's adjList
//                 graph.get_mut(&novels[from].v_id).unwrap().push((novels[to].v_id, weight));
//                 // Ensure that the graph is undirected (edges are symmetric for both nodes)
//                 // If the other node is not already in the graph, add it
//                 if !graph.contains_key(&novels[to].v_id) {
//                     graph.insert(novels[to].v_id, vec![]);
//                 }
//                 // Now that the other node is in the graph add the current node to its adjList
//                 graph.get_mut(&novels[to].v_id).unwrap().push((novels[to].v_id, weight));
//             }
//             // End if
//         }
        
//     }
//     graph}


async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u8)>> {     // TODO: THIS WILL RETURN SOMETHING
    let mut graph: BTreeMap<u16, Vec<(u16,u8)>> = BTreeMap::new();
    for from in 0..novels.len(){ // Comparing 'from' novel to every other novel after it.
        // println!("{}, {}", novels[from].title, novels[from].v_id);
        let mut weights: Vec<(u16, u8)> = vec![];
        for to in 0..novels.len(){
            if to != from {
                let similarity: u8 = novels[from].comparing(&novels[to]);

                if similarity < 115{
                    weights.push((novels[to].v_id, similarity));
                }
            }
        }
        graph.insert(novels[from].v_id, weights.clone());
    }
    graph
}