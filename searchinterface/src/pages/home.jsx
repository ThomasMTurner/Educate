import styles from '../styles/page.module.css';
import SearchBar from '../components/SearchBar';
import { IoIosSettings } from "react-icons/io";
import { CgProfile } from "react-icons/cg";
import { FaHistory } from "react-icons/fa";
import { useState, useEffect } from 'react';
import { useAuth } from '../AuthProvider';
import { useNavigate } from 'react-router-dom';
import { motion } from 'framer-motion';
import ClipLoader from 'react-spinners/ClipLoader';
import axios from 'axios';
import SearchResult from '../components/SearchResult';
import SearchHistory from '../components/SearchHistory';

// Overriding the default timeout to 5 minutes while we speedup the search request.
axios.defaults.timeout = 500000;

const Home = () => {
  const [iconColours, setIconColours] = useState({"Settings": "gray", "History": "gray", "Profile": "gray"})
  const [resultsScreen, setResultsScreen] = useState(false)
  const [searchQuery, setSearchQuery] = useState("")
  const [search, setSearch] = useState(false)
  const [searchResults, setSearchResults] = useState([])
  // const [summaries, setSummaries] = useState({})
  const [loadingResults, setLoadingResults] = useState(false)
  const [searchBarOffset, setSearchBarOffset] = useState(12);
  const [historyVisible, setHistoryVisible] = useState(false);
  const [performance, setPerformance] = useState({"Indexed": null, "Ranked": null, "Time": null})
  
  const { user, logOut, history } = useAuth();
  
  const navigate = useNavigate();
 
  const setIconColour = (iconName, colour) => {
    setIconColours(prev => ({...prev, [iconName]: colour}));
  }

  const handleAuthNavigate = () => {
    navigate('/login');
  }

  const handleHistoryNavigate = () => {
    navigate('/history');
  }

  const handleSettingsNavigate = () => {
    navigate('/settings');
  }

  useEffect(() => {
        if (search) {
            const handleSearch = async () => {

                // Trigger search bar animation.
                setResultsScreen(true)

                // Trigger loading animation - reset to true.
                setLoadingResults(true)

                const data = {
                    query: searchQuery
                }
                
                // Call to fill indices with new documents.
                await axios.get("http://localhost:9797/search/fill")
                    .then((response) => {
                        console.log(response.data)
                    })
                    .catch((error) => {
                        console.error(error)
                    })

                let results = []
               
                try {

                    // Call to rank existing indices based on search query.
                    const response = await axios.post('http://localhost:9797/search/get-results', data)

                    setSearchResults(response.data.results)
                    results.push(...response.data.results)

                    // TO DO: set performance indicators (number indexed, number ranked, time taken).
                    setPerformance({"Indexed": response.data.indexed, "Ranked": response.data.results.length, "Time": "10 minutes"}) 

                    setLoadingResults(false)
                    setSearchBarOffset(0)

                    } catch (error) {
                        console.error(error)
                        setLoadingResults(false)
                    }
                
                    if (user != null) {
                        try {
                            const result = results.map(({ url, title }) => ({ 
                            url, 
                            title, 
                            date: new Date().toLocaleString(),
                            query: searchQuery  
                        }));
        
                    await axios.post('http://localhost:9797/auth/add-history', {
                        history: result, 
                        username: user, 
                        password: ""
                    })
                } catch (error) {
                    console.error(error)
                }
            }

                    // Call to summarise each search result
                    /* 
                    Promise.all(searchResults.map(async (result) => {
                        try {
                            const response = await axios.post('http://localhost:11434/api/generate', {
                                model: 'llama2-uncensored', 
                                prompt: "Summarise the following text: " + result.content, 
                            });
                            setSummaries(prev => ({...prev, [result.id]: response.data}))
                            console.log(summaries)
                        } catch (error) {
                            console.error("There was an error: ", error);
                        }
                    }));
                    */
                }
            
                handleSearch()
            }

    }, [search])

    return (
    (!resultsScreen) ? (
        <div className={styles.main}>
            <motion.div 
                style={{display: 'flex', alignItems: 'center', position: 'relative', gap:'2rem', bottom:'2.5rem'}}
                initial={{opacity: 0, x: -50}}
                animate={{opacity: 1, x: 0}}
                transition={{duration: 0.75}}
            >
                <h1 style={{fontFamily: 'helvetica', fontWeight: '100'}}> Welcome to <b style={{fontWeight: 'bold'}}>Educate Search.</b></h1> 
                <div style={{display: 'flex', position: "relative", gap: '1rem'}}>
                    <IoIosSettings size={25} onClick={handleSettingsNavigate}  color={iconColours.Settings} onMouseEnter={() => setIconColour("Settings", "black")} onMouseLeave={() => setIconColour("Settings", "gray")} />
                    <div>
                    <FaHistory size={25} onClick={handleHistoryNavigate} color={iconColours.History} onMouseEnter={() => setIconColour("History", "black")} onMouseLeave={() => setIconColour("History", "gray")}/>
                    </div>
                    <div style={{display: 'flex', flexDirection: 'column', gap: '0.05rem', alignItems: 'center', justifyContent: 'center'}}>
                        <CgProfile size={25} />
                        <p style={{fontWeight: '200', fontSize: '0.8rem', position:'relative', bottom:'0.2rem'}}> { 
                        user != null ? (
                            <button onClick={logOut} className={styles.defaultButton}> Logout </button>
                        )
                        : (
                            <button onClick={handleAuthNavigate} className={styles.defaultButton}> Login or Register </button>
                        )
                            } </p>
                    </div>
                </div>
            </motion.div>
            <motion.div 
                style={{position: 'relative', bottom: '7rem', width:'50rem', fontWeight: '200'}}
                initial={{opacity: 0, y: 50, scale: 0.5}}
                animate={{opacity: 1, y: 50, scale: 1}}
                transition={{duration: 0.75}}
            >
                <p>Providing search results for educational content, carefully selected from research- and academic-oriented domains.</p>
            </motion.div>
            <div>
                <SearchBar searchBarOffset={searchBarOffset} setSearchQuery={setSearchQuery} setSearch={setSearch}/>
            </div>
        </div>
        ) : (
        <motion.div 
            style={{display: 'flex', flexDirection: 'column', position:'relative', justifyContent: 'center', alignItems: 'center', gap: '5rem'}}
            initial={{opacity: 0, y: 0}}
            animate={{opacity: 1, y: -10}}
            transition={{duration: 0.5}}
        > 
        <SearchBar setSearchQuery={setSearchQuery} setSearch={setSearch}/>
            <div style={{display:'flex', position:'relative', alignItems:'left', justifyContent:'left', textAlign:'left', flexDirection:'column'}}>
            {(!loadingResults) && <p style={{fontFamily:'arial', color:'darkgray', fontWeight:'200'}}> (Indexed {performance["Indexed"]} results & ranked {performance["Ranked"]} results in {performance["Time"]})  </p>}
            {!(loadingResults) ? (
                searchResults.map((document, index) => (
                    <div onClick={() => window.open(document.url, '_blank')} key={index}>
                        <SearchResult document={document} />
                    </div>
            ))
            ) : (
            <div style={{ position: 'relative', right: '1rem' }}>
                <ClipLoader color="#52bfd9" size={40} />
            </div>
            )}
            </div>
        </motion.div>
    )
  );
};
  


export default Home;
