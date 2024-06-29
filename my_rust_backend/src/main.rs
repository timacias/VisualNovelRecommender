use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders, AllowMethods};


mod test;
use test::Person;

#[derive(Deserialize)]
struct InputData {
    input: String,
}

async fn handle_input(Json(data): Json<InputData>) -> &'static str {
    println!("Received input: {}", data.input);
    "Input received"
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::any())
    .allow_methods(AllowMethods::any())
    .allow_headers(AllowHeaders::any()); 

    let app = Router::new()
        .route("/", get(get_people))
        .route("/people", get(get_people))
        .route("/input", post(handle_input))
        .layer(cors);

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

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person {
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        },
    ];

    Json(people)
}


//import React, { useEffect, useState } from 'react';
// function App() {
//     const [message, setMessage] = useState('');
//     const [input, setInput] = useState('');
  
    
//     const handleSubmit = async () => {
//       const response = await fetch('http://localhost:3000/submit', {
//         method: 'POST',
//         headers: {
//           'Content-Type': 'application/json',
//         },
//         body: JSON.stringify({ input }),
//       });
//       const data = await response.json();
//       console.log(data);
//     };
  
//     useEffect(() => {
//       fetch('http://localhost:3000/')
//         .then(response => response.text())
//         .then(data => setMessage(data))
//         .catch(console.error);
//     }, []);
  
//     return (
//       <div>
//         <input
//           type="text"
//           value={input}
//           onChange={(e) => setInput(e.target.value)}
//         />
//         <button onClick={handleSubmit}>Submit</button>
//       </div>
//     );
//   }
  
//   export default App;
  