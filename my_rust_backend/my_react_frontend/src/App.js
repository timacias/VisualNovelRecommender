import React, { useEffect, useState } from 'react';
import axios from 'axios';

function App() {
  const [people, setPeople] = useState([]);
  const [input, setInput] = useState("");
  
  useEffect(() => {
    axios.get('http://localhost:3000/people')
      .then(response => {
        setPeople(response.data);
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

  return (
    <div className="App">
      <h1>People from backend</h1>
      <ul>
        {people.map((person, index) => (
          <li key={index}>
          Name: {person.name}, Age: {person.age}
          {person.favourite_food && `, Favorite Food: ${person.favourite_food}`}
        </li>
        ))}
      </ul>
      <div>
        <input type="text" value={input} onChange={handleInputChange} />
        <button onClick={handleSubmit}>Send Input</button>
      </div>
    </div>
  );
}

export default App;


// useEffect(() => {
//   fetch('http://localhost:3000/')
//     .then(response => response.text())
//     .then(data => setMessage(data))
//     .catch(console.error);
// }, []);

// return <div>{message ? message : 'Loading...'}</div>;
// }

// export default App;
