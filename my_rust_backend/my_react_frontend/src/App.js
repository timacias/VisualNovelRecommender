import React, { useEffect, useState } from 'react';
import axios from 'axios';
import VisualNovelSearch from './VNapi';
import './App.css';
import { FixedSizeList as List } from 'react-window';

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

  const [Novel, setNovel] = useState([]);
  const [input, setInput] = useState("");
  const [currentnovel1, setCurrentnovel1] = useState("Fate");
  const [searchQuery, setSearchQuery] = useState("");
  const [nameSearch, setNameSearch] = useState([]);

  const [Novel2, setNovel2] = useState([]);
  const [input2, setInput2] = useState("");
  const [currentnovel2, setCurrentnovel2] = useState("Fate");
  const [searchQuery2, setSearchQuery2] = useState("");
  const [nameSearch2, setNameSearch2] = useState([]);
  const [id1, setId1] = useState("");
  const [id2, setId2] = useState("");
  const [Result, setResult] = useState([]);
 

  const [checked, setchecked] = useState(false);

 

  const getresults = () => {
    axios.get('http://localhost:3000/result')
    .then(response => {
      setResult(response.data);
    })
    .catch(error => console.error('Error fetching data:', error));
}

  useEffect(() => {

    axios.get('http://localhost:3000/people')
      .then(response => {
        setNovel(response.data);
        setNovel2(response.data);
      })
      .catch(error => console.error('Error fetching data:', error));

      
    getresults();

  }, []);
  
  function refreshPage() {
    getresults();
  }

  const handlecheck = (event) => {
    setchecked(event.target.checked);
  };
  const Novelsearch1 = () => {
    const novelExists = Novel.some(novel => novel.title.toLowerCase() === input.toLowerCase());

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    setCurrentnovel1(input);
  };

  const Novelsearch2 = () => {
    const novelExists = Novel2.some(novel => novel.title.toLowerCase() === input2.toLowerCase());

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    setCurrentnovel2(input2);
  };

  const handleSubmit = () => {
    const novelExists = Novel.some(novel => novel.title.toLowerCase() === input.toLowerCase());

    if (!novelExists) {
      alert('Input is not a valid novel in our database');
      return;
    }

    const novelExists2 = Novel2.some(novel => novel.title.toLowerCase() === input2.toLowerCase());

    if (!novelExists2) {
      alert('Input is not a valid novel in our database');
      return;
    }
    setCurrentnovel1(input);
    setCurrentnovel2(input2);
  
    axios.post('http://localhost:3000/input', {
      input: input,
      input2: input2,
      id1: id1,
      id2: id2,
      checked: checked
    })
      .then(response => alert('Input was successfully sent to the backend'))
      .catch(error => console.error('Error sending input:', error));
  };

  const handleSearch = (newSearchQuery) => {
    setSearchQuery(newSearchQuery);
    const Novelfound = Novel.filter(Novel =>
      Novel.title.toLowerCase().includes(newSearchQuery.toLowerCase())
    );
    setNameSearch(Novelfound);
  };

  const handleSearch2 = (newSearchQuery) => {
    setSearchQuery2(newSearchQuery);
    const Novelfound = Novel2.filter(Novel2 =>
      Novel2.title.toLowerCase().includes(newSearchQuery.toLowerCase())
    );
    setNameSearch2(Novelfound);
  };

  const Column1 = ({ index, style }) => (
    <div style={style}>
       <div className={index % 2 ? 'list1' : 'list2'}>
       <b> Name: </b>{nameSearch[index].title + ' '}
        <b>Tags:</b> {Array.from(nameSearch[index].tag_cont).slice(0, 3).join(', ')}
      </div>
    </div>
  );
  const Column2 = ({ index, style }) => (
    <div style={style}>
      <div className={index % 2 ? 'list1' : 'list2'}>
        <b>Name:</b> {nameSearch2[index].title + ' '}
        <b>Tags:</b> {Array.from(nameSearch2[index].tag_cont).slice(0, 3).join(', ')}
      </div>
    </div>
  );


  return (
    <div className="App">
      <h2>Note: our database for Novels is not including NSFW and r18 tags, our database is from the June 24, 2024 VNDB</h2>
      <h3>Because the VNDB is an ongoing database that includes realy old novels and novels that are in progress, some information such as date, rating, image, or description may not be available</h3>
      <h3>Some NSFW novels may slip through due to lack of proper nsfw tag documentation from the vndb</h3>
      <h3>our lists only display the top 3 tags in a novel for visual clarity, but we use all of the tags in calculation</h3>
      <h3>Graph edges is based on similarity score</h3>

      <div class="flexbox-container">
        <div class="module">
          <h1>Choose a starting novel!</h1>
          <div>
          <Search handleSearch={handleSearch} input={input} setInput={setInput} />
          <button onClick={Novelsearch1}>Check Database</button>
          </div>
          <h2>Database Results:</h2>
          <div class= "holder">
          <List
            height={250}
            itemCount={nameSearch.length}
            itemSize={37}
            width={'100%'}
          >
            {Column1}
          </List>
          </div>
          
          {/*<ul>
            {nameSearch.map((Novel, index) => (
              <li key={index}>
                Name: {Novel.name}, Age: {Novel.age}
                {Novel.favourite_food && `, Favorite Food: ${Novel.favourite_food}`}
              </li>
            ))}
          </ul>
          */}

        </div>

        <div class="module">
          <h1>Choose an ending novel!</h1>
          <div>
          <Search handleSearch={handleSearch2} input={input2} setInput={setInput2} />
          <button onClick={Novelsearch2}>Check Database</button>
          </div>
          <h2>Database Results:</h2>
          <div class= "holder">
          <List
            height={250}
            itemCount={nameSearch2.length}
            itemSize={37}
            width={'100%'}
          >
          
            {Column2}
          </List>
          </div>
          {/*
          <ul>
            {nameSearch2.map((Novel2, index) => (
              <li key={index}>
                Name: {Novel2.name}, Age: {Novel2.age}
                {Novel2.favourite_food && `, Favorite Food: ${Novel2.favourite_food}`}
              </li>
            ))}
          </ul>
          */}
        </div>
        
      </div>

      <label>
        <input
          type="checkbox"
          checked={checked}
          onChange={handlecheck}
        />
      </label>
      <p>{checked ? "BFS Search" : "Djikstra's Search"}</p>
      <button onClick={handleSubmit}>Send Input</button>
      <button onClick={refreshPage}>Get Results!</button>
      <div class="flexbox-container">
        <div class="module">
          <VisualNovelSearch title={currentnovel1} check={true} id = {id1} setId={setId1}/>
        </div>
      
        <div class="module">
          <VisualNovelSearch title={currentnovel2} check={true} id = {id2} setId={setId2}/>
        </div>
      </div>
      <h1>{id1}</h1>
      <h1>{id2}</h1>

    <h2><b>Results!</b></h2>
      <ul>
            {Result.map((str, index) => (
            //   <li key={index}>
            //   {str}
            // </li>
              <VisualNovelSearch title={str} check={false} id = {id1} setId={setId1}/>
              
              
            ))}
          </ul>

    </div>
  );
}

export default App;
//    PORT=3001 npm start