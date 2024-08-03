import React, { useEffect, useState } from 'react';
import './VNAPI.css';
import axios from 'axios';

//this is the code for calling the vndb api, we used axios to call it we used this documentation from the vndb website to code it https://api.vndb.org/kana
// and we also used these guides for api calls using axios https://medium.com/@olivier.trinh/bringing-your-react-app-to-life-mastering-api-calls-a4adbc08f7df, 
//https://blog.logrocket.com/how-to-make-http-requests-like-a-pro-with-axios/, https://builtin.com/software-engineering-perspectives/react-api, 


//this is the url we call for the information and our apitoken which gives us access, we also create our axios item to be able to use it later
const API_TOKEN = 'jmhy-biikn-f8oe3-6rq8-mub8w-9am88-fjcy';
const api = axios.create({
  baseURL: 'https://api.vndb.org/kana',
  headers: {
    'Authorization': `Token ${API_TOKEN}`,
    'Content-Type': 'application/json',
  },
});

//this is the function that calls the actual vndb api, it takes in a title and fields argument, the fields argument is what we want to get and title is what we give it
//we send a post request to get the data
//it uses the title to search for vn with the title through the filters and returns the data
export const queryVisualNovelByTitle = async (title, fields) => {
    try {
      const response = await api.post('/vn', {
        filters: ["search", "=", title],
        fields
      });
      return response.data;
    } catch (error) {
      console.error('Error getting VN: ', error);
      throw error;
    }
  };
  

  //this is the function that we call from our app.js it calls the function above to get the info
  const VisualNovelSearch = ({ title, check, id, setId }) => {
    //these are variables, we use them to set the visualnovel after we get it
    const [visualNovel, setVisualNovel] = useState(null);
    
    useEffect(() => {
      //this funcion is called whenever VisualNovelSearch is called and it calls the function to get the api data
      const fetchData = async () => {
        try {
          //these fields indicate our request to the API.
          const fields = "title,image.url,released,description,rating,image.thumbnail,id"; 
          const data = await queryVisualNovelByTitle(title, fields);
          //the api sends back an array of novels that match the title, so we check if the novel exists and we set it to the first in the array
          if (data.results.length > 0) {
            setVisualNovel(data.results[0]);
            //then we set our id to the id of the novel, this is the variable sent in by app.js to send back to the rust server
            setId(data.results[0].id.toString());
          } else {
            //these are errors if the post request fails
            console.error('Visual novel not found');
            setError('Visual novel not found');
          }
        } catch (err) {
          console.error("error");
        }
      };
      
      fetchData();
    }, [title]);


    if(check){
      return (
        //render on screen the title, releasedate, rating, image and description
        <div>
          <h1>{visualNovel.title}</h1>
          <h2>date: {visualNovel.released}</h2>
          <h3>rating: {visualNovel.rating}</h3>
          {visualNovel.image && <img src={visualNovel.image.thumbnail} alt={visualNovel.title} />}
          <h3>description:</h3>
          <h5>{visualNovel.description}</h5>
        </div>
      );
    }
    else{
      return(
           //render on screen the title, releasedate, rating, image and description just in a different format
        <div>
          <h5>{visualNovel.title}</h5>
          <h5>date: {visualNovel.released}</h5>
          <div className='resultmodule'>
            {visualNovel.image && <img src={visualNovel.image.thumbnail} alt={visualNovel.title} />}
            <h5>{visualNovel.description}</h5>
          </div>
        
      </div>
      );
    }
    
  };
  
  export default VisualNovelSearch;