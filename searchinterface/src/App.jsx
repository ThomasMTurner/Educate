import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import './App.css';
import Home from '../pages/Home.jsx';
import History from './pages/History.jsx'; 
import { SearchCache } from './SearchCache';

function App() {
 const [searches, setSearches] = useState([]);

 return (
    <SearchCache.Provider value={{ searches, setSearches }}>
      <Router>
        <Switch>
          <Route exact path="/" component={Home} />
          <Route path="/history" element={<History searches={searches} />} />
        </Switch>
      </Router>
    </SearchCache.Provider>
 );
}

export default App;
