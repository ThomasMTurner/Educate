import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import './App.css';
import Home from './pages/home.jsx';
import History from './pages/history.jsx'; 
import { SearchCache } from './SearchCache';

function App() {
    const [searches, setSearches] = useState([]);
    
    return (
        <SearchCache.Provider value={{ searches, setSearches }}>
            <Router>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/history" element={<History searches={searches} />} />
                </Routes>
            </Router>
        </SearchCache.Provider>
    );
}

export default App;
