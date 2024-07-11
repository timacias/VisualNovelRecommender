import React, { useEffect, useState } from 'react';

import axios from 'axios';

const API_BASE_URL = 'https://api.vndb.org/kana';
const API_TOKEN = 'jmhy-biikn-f8oe3-6rq8-mub8w-9am88-fjcy';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Authorization': `Token ${API_TOKEN}`,
    'Content-Type': 'application/json',
  },
});

export const queryVisualNovelByTitle = async (title, fields) => {
    try {
      const response = await api.post('/vn', {
        filters: ["search", "=", title],
        fields
      });
      return response.data;
    } catch (error) {
      console.error('Error querying visual novel:', error);
      throw error;
    }
  };
  const VisualNovelSearch = ({ title }) => {
    const [visualNovel, setVisualNovel] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
  
    useEffect(() => {
      const fetchData = async () => {
        try {
          const fields = "title,image.url";
          const data = await queryVisualNovelByTitle(title, fields);
          if (data.results.length > 0) {
            setVisualNovel(data.results[0]); //use first result
          } else {
            setError('Visual novel not found');
          }
          setLoading(false);
        } catch (err) {
          setError(err.message);
          setLoading(false);
        }
      };
  
      fetchData();
    }, [title]);
  
    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error: {error}</p>;
  
    return (
      <div>
        <h1>{visualNovel.title}</h1>
        {visualNovel.image && <img src={visualNovel.image.url} alt={visualNovel.title} />}
      </div>
    );
  };
  
  export default VisualNovelSearch;