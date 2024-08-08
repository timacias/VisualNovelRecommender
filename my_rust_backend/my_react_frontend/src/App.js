/*
Copyright © 2024 Sarah Tran, Justin Ming, Timothy Macias

This file is part of VisualNovelRecommender.

VisualNovelRecommender is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

VisualNovelRecommender is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with VisualNovelRecommender. If not, see <https://www.gnu.org/licenses/>.
*/

import React, { useEffect, useState } from 'react';
import axios from 'axios';
import VisualNovelSearch from './VNapi';
import './App.css';
import { FixedSizeList as List } from 'react-window';
//this is the frontend, it is using react and connects to the rust backend using axios which is user to call the rust backend local server and call the vndb api
// we also use the library react-window to help render the list, we have thousands of novels in our list and rendering them is very slow
//react-window allows us to lazy loadd it so only renders a portion of the list at a time on screen

//to connect rust and react using axum we used this guide https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e
// as a base to connect the two

//this is the search function, it allows for the searchbar to change characters are typed in by setting the input to e and the value (searchbar value) to input
//we used this guide as a reference to build the search components https://dev.to/alais29dev/building-a-real-time-search-filter-in-react-a-step-by-step-guide-3lmm
function Search({ handleSearch, input, setInput}) {
  const onChange = (e) => {
    handleSearch(e.target.value);
    setInput(e.target.value);
  };

  return (
    <input
      type="text"
      placeholder="Search by name"
      value={input}
      onChange={onChange}
    />
  );
}

function App() {
//these are just variables we initialize to use, novel is the vector of novels that is passed in from the rust backend
//input is the user input for the novels they choose
//current novel, is the current novel chosen by the user
//search query is used to help the search bar to find an item in the novels array
//namesearch is another vector of the novels used to render them onscreen
  const [Novel, setNovel] = useState([]);
  const [input, setInput] = useState("");
  const [currentnovel1, setCurrentnovel1] = useState("Fate/zero");
  const [searchQuery, setSearchQuery] = useState("");
  const [nameSearch, setNameSearch] = useState([]);

  //these are secondary variables because the user chooses a 2 novels.
  const [Novel2, setNovel2] = useState([]);
  const [input2, setInput2] = useState("");
  const [currentnovel2, setCurrentnovel2] = useState("Fate/zero");
  const [searchQuery2, setSearchQuery2] = useState("");
  const [nameSearch2, setNameSearch2] = useState([]);
  //this is the ids of the novels 1 and 2, this is updated after submit is called and is sent to the backend to use in processing
  const [id1, setId1] = useState("");
  const [id2, setId2] = useState("");
  //this is a result vector that gets an array of novels that form the shortest path from the backens
  const [Result, setResult] = useState([]);
//this checked is a boolean that signifies whether to use djikstras or bellmanford
//time variable is time of algorithm
  const [checked, setchecked] = useState(false);
  const [loading, setloading] = useState(false);
  const [time, settime] = useState("");
 
//this is the function to get the shortest path then set it to the result vector this function will be called after the get results button and submit input button is pressed
//get first entry in array which is time taken for algorithm and from second onwards is the shortest path
  const getresults = () => {
    axios.get('http://localhost:3000/result')
    .then(response => {
      settime(response.data[0]);
      setResult(response.data.slice(1));
    })
    .catch(error => console.error('Error fetching data:', error));
}

//this is the useffect, this is called whenever something needs to render, so this is basically called at the start of launching the application and initializes
//the backend information to the frontend. It gets the graph and puts it into the novel vectors on startup and calls getresults to initialize it blank.

  useEffect(() => {

    axios.get('http://localhost:3000/people')
      .then(response => {
        setNovel(response.data);
        setNovel2(response.data);
      })
      .catch(error => console.error('Error fetching data:', error));

      
    getresults();

  }, []);
  const loadingcheck = (event) => {
    setloading(event.target.loading);
  };
  //this function calls the getresults function to render onscreen
  function refreshPage() {
    getresults();
  }
//this function just toggles the boolean for which algorithm to use
  const handlecheck = (event) => {
    setchecked(event.target.checked);
  };
  //this is the check database button for the first novel, it checks if the user input is a valid novel title in our graph and if so sets the currentnovel1 to it
  const Novelsearch1 = () => {
    const novelExists = Novel.some(novel => novel.title === input);

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    setCurrentnovel1(input);
  };
//this is the check database button for the second novel, it checks if the user input is a valid novel title in our graph and if so sets the currentnovel2 to it
  const Novelsearch2 = () => {
    const novelExists = Novel2.some(novel => novel.title === input2);

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    setCurrentnovel2(input2);
  };

  //this is the submit input button, it does another round of checking to see if the inputs are valid novels then does a post request which sends the input to the backend
  
  const handleSubmit = () => {
    const novelExists = Novel.some(novel => novel.title === input);

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    const novelExists2 = Novel2.some(novel => novel.title === input2);

    if (!novelExists2) {
      alert('Input is not a valid novel in our database');
      return;
    }
    setCurrentnovel1(input);
    setCurrentnovel2(input2);
    setloading(true);
    //https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e
    axios.post('http://localhost:3000/input', {
      input: input,
      input2: input2,
      id1: id1,
      id2: id2,
      checked: checked
    })
      .then(
        response => alert('Input was successfully sent to the backend'))
      .catch(error => console.error('Error sending input:', error));
  };

  //this function is for the first novel search bar, it filters the array to only include current user input to narrow down the search. 
  //so when you type in fate into the input bar, it shows novels including the word fate
  //we used this to help build the search bar https://dev.to/alais29dev/building-a-real-time-search-filter-in-react-a-step-by-step-guide-3lmm
  const handleSearch = (newSearchQuery) => {
    setSearchQuery(newSearchQuery);
    const Novelfound = Novel.filter(Novel =>
      Novel.title.toLowerCase().includes(newSearchQuery.toLowerCase())
    );
    setNameSearch(Novelfound);
  };


  //this function is for the second novel search bar, it filters the array to only include current user input to narrow down the search. 
  //so when you type in fate into the input bar, it shows novels including the word fate
  //we used this to help build the search bar https://dev.to/alais29dev/building-a-real-time-search-filter-in-react-a-step-by-step-guide-3lmm
  const handleSearch2 = (newSearchQuery) => {
    setSearchQuery2(newSearchQuery);
    const Novelfound = Novel2.filter(Novel2 =>
      Novel2.title.toLowerCase().includes(newSearchQuery.toLowerCase())
    );
    setNameSearch2(Novelfound);
  };

  //this is the function to render the first list, we used this as a template https://react-window.vercel.app/#/examples/list/fixed-size
  //we made it so that it alternates in color between each novel and displays the name and first 3 tags
  const Column1 = ({ index, style }) => (
    <div style={style}>
       <div className={index % 2 ? 'list1' : 'list2'}>
       <button onClick={() => setInput(nameSearch[index].title)}>
       <b> Name: </b>{nameSearch[index].title + ' '}
        <b>Tags:</b> {Array.from(nameSearch[index].tag_cont).slice(0, 3).join(', ')}
        </button>

      </div>
    </div>
  );
    //this is the function to render the second list, we used this as a template https://react-window.vercel.app/#/examples/list/fixed-size
  //we made it so that it alternates in color between each novel and displays the name and first 3 tags
  const Column2 = ({ index, style }) => (
    <div style={style}>
      <div className={index % 2 ? 'list1' : 'list2'}>
      <button onClick={() => setInput2(nameSearch2[index].title)}>
        <b>Name:</b> {nameSearch2[index].title + ' '}
        <b>Tags:</b> {Array.from(nameSearch2[index].tag_cont).slice(0, 3).join(', ')}
        </button>

      </div>
    </div>
  );



  return (
    <div className="App">
    <h3>Instructions:</h3>
    <h5>1. Search for a novel in our database using the bars under 'Choose a starting/ending novel!'</h5>
    <h5>2. Click the button in the list for the novel you want. This is important due to case sensitivity.</h5>
    <h5>3. Click the 'Check Database' buttons for both novels.</h5>
    <h5>4. Click 'Send Input', wait for a popup saying that input has been sent.</h5>
    <h5>5. Click `Get Results!'</h5>
    <h5>Note: Use the checkbox above the 'Send Input' button to toggle pathfinding algorithms.</h5>
    <h5>A copy of this program's source code can be found at https://github.com/timacias/VisualNovelRecommender</h5>
      <div class="flexbox-container">
        <div class="module">
          <h1>Choose a starting novel!</h1>
          <div>
          {/* call the first search bar and render it */}
          <Search handleSearch={handleSearch} input={input} setInput={setInput} />
          {/* render the button for the searchbar */}
          <button onClick={Novelsearch1}>Check Database</button>
          </div>
          <h2>Database Results:</h2>
          <div class= "holder">
            {/* render the list */}
          <List
            height={250}
            itemCount={nameSearch.length}
            itemSize={37}
            width={'100%'}
          >
            {/* render the array in the reac-window component */}
            {Column1}
          </List>
          </div>

        </div>

        <div class="module">
          {/* render the second search bar */}
          <h1>Choose an ending novel!</h1>
          <div>
          <Search handleSearch={handleSearch2} input={input2} setInput={setInput2} />
          {/* render the second button */}
          <button onClick={Novelsearch2}>Check Database</button>
          </div>
          <h2>Database Results:</h2>
          <div class= "holder">
            {/* render the second array of novels */}
          <List
            height={250}
            itemCount={nameSearch2.length}
            itemSize={37}
            width={'100%'}
          >
          {/* render the array in the reac-window component  */}
            {Column2}
          </List>
          </div>
         
        </div>
        
      </div>
      {/* render the checkbox  */}
      <label>
        <input
          type="checkbox"
          checked={checked}
          onChange={handlecheck}
        />
      </label>
      {/* if checked boolean is false display djikstras, if true bellmanford */}
      <p>{checked ? "Bellman-Ford Pathfinding" : "Djikstra's Pathfinding"}</p>
      {/* render the send input and get results button */}
      <button onClick={handleSubmit}>Send Input</button>
      <button onClick={refreshPage}>Get Results!</button>

      <div class="flexbox-container">
        <div class="module">
          {/* call the component which calls the api for vndb and enter in the novel and a boolean, 
          the boolean just says how we want the information to be displayed, we also give it 
          a variable for id and a function to change id so we can get the id of the novel and send it to the backend */}
          <VisualNovelSearch title={currentnovel1} check={true} id = {id1} setId={setId1}/>
        </div>
      
        <div class="module">
           {/* call the component which calls the api for vndb and enter in the novel and a boolean, 
          the boolean just says how we want the information to be displayed, we also give it 
          a variable for id and a function to change id so we can get the id of the novel and send it to the backend */}
          <VisualNovelSearch title={currentnovel2} check={true} id = {id2} setId={setId2}/>
        </div>
      </div>
            {/* render the shortest path results by calling the api component again, but with boolean = to false for a different display format */}
    <h2><b>Results!</b></h2>
    <h1>Time taken: {time} seconds!!!!!</h1>
      <ul>
        {/* only display it if the length is 2 or more, if less it means there was no path */}
      {Result.length < 2 ? (
        <p>No path</p>
      ) : (
        Result.map((str, index) => (
          <VisualNovelSearch title={str} check={false} id = {id1} setId={setId1}/>
        ))
      )}
      
          </ul>

    </div>
  );
}

export default App;
//    PORT=3001 npm start
