import React, { useEffect, useState } from 'react';
import axios from 'axios';
import VisualNovelSearch from './VNapi';
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
  const [searchQuery, setSearchQuery] = useState("");
  const [nameSearch, setNameSearch] = useState([]);

  const [searchTitle, setSearchTitle] = useState('');

  const handleSearchvn = (e) => {
    e.preventDefault();
    setSearchTitle(e.target.elements.title.value);
  };

  useEffect(() => {

    axios.get('http://localhost:3000/people')
      .then(response => {
        setNovel(response.data);
      })
      .catch(error => console.error('Error fetching data:', error));
  }, []);

  const handleInputChange = (event) => {
    setInput(event.target.value);
  };

  const handleSubmit = () => {
    axios.post('http://localhost:3000/input', { input })
      .then(response => alert('Input was successfully sent to the backend'))
      .catch(error => console.error('Error sending input:', error));
  };

  const handleSearch = (newSearchQuery) => {
    setSearchQuery(newSearchQuery);
    const filteredPeople = Novel.filter(Novel =>
      Novel.title.toLowerCase().includes(newSearchQuery.toLowerCase())
    );
    setNameSearch(filteredPeople);
  };
  const Row = ({ index, style }) => (
    <div style={style}>
      Name: {nameSearch[index].title}
    </div>
  );

  return (
    <div className="App">
      <h1>People from backend</h1>
      <div>
        <Search handleSearch={handleSearch} input={input} setInput={setInput} />
        <button onClick={handleSubmit}>Send Input</button>
      </div>
      <h2>Search Results:</h2>
      <List
        height={400}
        itemCount={nameSearch.length}
        itemSize={35}
        width={'100%'}
      >
        {Row}
      </List>
      <form onSubmit={handleSearchvn}>
        <input type="text" name="title" placeholder="Enter Visual Novel Title" />
        <button type="submit">Search</button>
      </form>
      {searchTitle && <VisualNovelSearch title={searchTitle} />}
    </div>
  );
}

export default App;