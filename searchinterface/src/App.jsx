import React, { useState } from 'react';
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import './App.css';
import Home from './pages/home';
import History from './pages/history'; 
import Settings from './pages/settings';
import Auth from './pages/authpage'
import AuthProvider from './AuthProvider'

function App() {
    return (
        <Router>
            <AuthProvider>
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/login" element={<Auth/>}/>
                    <Route path="/history" element={<History/>} />
                    <Route path="/settings" element={<Settings/>} />
                </Routes>
            </AuthProvider>
        </Router>
    );
}

export default App;
