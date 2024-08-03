mod test;
mod csv_reader;
use std::collections::{BTreeMap, HashMap};
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
    titletoid: HashMap<String, u16>,
    novel_graph: BTreeMap<u16, Vec<(u16, u16)>>
}

async fn handle_input(Json(data): Json<InputData>, Extension(state): Extension<SharedState>) -> impl IntoResponse {
    println!("Received input: {}", data.input);
    println!("Received input: {}", data.input2);
    println!("Received input: {}", data.checked);
    println!("Received input: {}", data.id1);
    println!("Received input: {}", data.id2);


    let mut state = state.lock().unwrap();

    println!("test");
    state.result.clear();
   
   //this was old method of getting the id, id was passed in from frontend through api, now we get it from backend 
    // state.result.push(data.input.clone());
    // state.result.push(data.input2.clone());
    // state.result.push(data.checked.to_string());

    // let novelid1 = &data.id1[1..].to_string();
    // let novelid2 = &data.id2[1..].to_string();
    // let intnovelid1: u16 = match novelid1.parse() {
    //     Ok(num) => num,
    //     Err(e) => {
    //         println!("Error parsing novelid1: {}", e);
    //         return (StatusCode::BAD_REQUEST, "Invalid id1 format").into_response();
    //     }
    // };
    // let intnovelid2: u16 = match novelid2.parse() {
    //     Ok(num) => num,
    //     Err(e) => {
    //         println!("Error parsing novelid2: {}", e);
    //         return (StatusCode::BAD_REQUEST, "Invalid id2 format").into_response();
    //     }
    // };
    // println!("{}", intnovelid1);
    // println!("{}", intnovelid2);
   
    state.result.clear();
    
    if !data.checked {
        let (dijkstra_path2,djistratime) = state.novel_graph.dijkstra(&(state.titletoid.get(&data.input).unwrap()), &( state.titletoid.get(&data.input2).unwrap()), state.novels.clone());
        state.result.push(djistratime.to_string());
        for vertices in dijkstra_path2 {
            println!("{}: {}", state.novels[state.novels.find_novel(&vertices)].v_id, state.novels[state.novels.find_novel(&vertices)].title);
            println!("{}", state.novels[state.novels.find_novel(&vertices)]);
            println!();
            let result = state.novels[state.novels.find_novel(&vertices)].title.to_string();
            println!("{}", result);
            state.result.push(result);
        }
    } else {
        let (bellmanford,bellmanfordtime) = state.novel_graph.bellman_ford(&(state.titletoid.get(&data.input).unwrap()), &( state.titletoid.get(&data.input2).unwrap()), state.novels.clone());
        state.result.push(bellmanfordtime.to_string());
        for vertices in bellmanford {
            println!("{}: {}", state.novels[state.novels.find_novel(&vertices)].v_id, state.novels[state.novels.find_novel(&vertices)].title);
            println!("{}", state.novels[state.novels.find_novel(&vertices)]);
            println!();
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

    let dijkstra_path1 = novel_graph.dijkstra(&(19119u16), &(18160u16), novels.clone());
    let bellman_path1 = novel_graph.bellman_ford(&(19119u16), &(18160u16), novels.clone());
    // Fate/EXTELLA (v19119) being compared to Collar x Malice (v18160) -> NO PATH FOUND

    let dijkstra_path2 = novel_graph.dijkstra(&(18160u16), &(14908u16), novels.clone());
    let bellman_path2 = novel_graph.bellman_ford(&(18160u16), &(14908u16), novels.clone());
    // Collar X Malice (v18160) being compared to Code:Realize (v14908) -> PATH FOUND

    let dijkstra_path3 = novel_graph.dijkstra(&(4602u16), &(30175u16), novels.clone());
    let bellman_path3 = novel_graph.bellman_ford(&(4602u16), &(30175u16), novels.clone());
    // Utano Prince Sama being compared to B Project Ryuusei Fantasia

    if dijkstra_path1 == bellman_path1 {
        println!("PATH 1 IS THE SAME!!!! YAY");
    }
    if dijkstra_path2 == bellman_path2 {
        println!("PATH 2 IS THE SAME!!!! YAY");
    }
    if dijkstra_path3 == bellman_path3 {
        println!("PATH 3 IS THE SAME!!!! YAY");
    }

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
    let shared_state = Arc::new(Mutex::new(AppState{novels,result: vec![],titletoid: titles_to_ids ,novel_graph,}));


    clearresult(&shared_state);
    addresult(&shared_state, "0".to_string());

                                          

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
async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u16)>> {
    let mut graph= BTreeMap::new();

    for from_index in 0..novels.len() {
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

//this was our old method to calculate weights it took longer to calulate

// async fn get_weights(novels: &Vec<Novel>) -> BTreeMap<u16, Vec<(u16, u16)>> {
//      let mut graph: BTreeMap<u16, Vec<(u16,u16)>> = BTreeMap::new();
//      let mut num_edges: u32 = 0;
//      for from in 0..novels.len(){ // Comparing 'from' novel to every other novel after it.
//          println!("{}, {}", novels[from].title, novels[from].v_id);
//          let curr_num_edge = num_edges;
//          let mut weights: Vec<(u16, u16)> = vec![];
//          for to in 0..novels.len(){
//              if to != from {
//                  let similarity: u16 = novels[from].comparing(&novels[to]);
//
//                  if similarity > 0  && similarity < 70 {
//                      weights.push((novels[to].v_id, similarity));
//                      num_edges += 1;
//                  }
//              }
//          }
//          if num_edges - curr_num_edge == 0 {
//              println!("NO EDGED ADDED")
//          }
//          graph.insert(novels[from].v_id, weights.clone());
//          println!();
//      }
//     println!("NUM EDGES: {}", num_edges);
//     graph
// }