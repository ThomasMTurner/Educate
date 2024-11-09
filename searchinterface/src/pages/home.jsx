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
import Summary from '../components/Summary';
import { closest } from 'fastest-levenshtein';
import { AutoSuggestions } from '../components/AutoSuggestion';

//import TrieSearch from 'trie-search';
//import SearchHistory from '../components/SearchHistory';

// Overriding the default timeout to 5 minutes while we speedup the search request.
axios.defaults.timeout = 500000;

function convertMsToTime(ms) {
    let seconds = Math.floor((ms / 1000) % 60);
    let minutes = Math.floor(ms / 60000);
    return `${minutes} minutes ${seconds} seconds`
} 

const Home = () => {
  const [iconColours, setIconColours] = useState({"Settings": "gray", "History": "gray", "Profile": "gray"})
  const [resultsScreen, setResultsScreen] = useState(false)
  const [searchQuery, setSearchQuery] = useState("")
  const [singleSuggestions, setSingleSuggestions] = useState([])
  const [fullSuggestions, setFullSuggestions] = useState([])
  const [completion, setCompletion] = useState("")
  const [search, setSearch] = useState(false)
  const [searchResults, setSearchResults] = useState([])
  const [summaries, setSummaries] = useState({})
  const [loadingResults, setLoadingResults] = useState(false)
  const [searchBarOffset, setSearchBarOffset] = useState(12);
  //const [historyVisible, setHistoryVisible] = useState(false);
  const [performance, setPerformance] = useState({"Indexed": null, "Ranked": null, "Time": null})
  
  const { user, logOut, config, trie, dataArray, relevanceTrie  } = useAuth();
  
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
    if (config.autosuggest) {
        let terms = searchQuery.split(" ");
        let auto_rel = relevanceTrie.search(terms[terms.length - 1])
        let auto = trie.search(terms[terms.length - 1])
        auto = [...auto_rel, ...auto]
        const suggestions = auto.slice(0, 4)
        if (!(auto === 'undefined') && auto.length > 0) {
            setCompletion(auto[0].key);
            setSingleSuggestions(suggestions);
        }
    }
  }, [searchQuery])

  useEffect(() => {
    let words = searchQuery.split(" ");
    words[words.length - 1] = completion
    setSearchQuery(words.join(" "))
  }, [completion])
    
  useEffect(() => {
        if (search && config) {
            const handleSearch = async () => {
                // Begin performance timer.
                const start = window.performance.now()

                // Trigger search bar animation.
                setResultsScreen(true)

                // Trigger loading animation - reset to true.
                setLoadingResults(true)

                let updatedConfig;

                if (config.query_correction) {
                    let i = 0;
                    let pre = searchQuery.split(" ")
                    for (const term of pre) {
                        let match = closest(term, dataArray);
                        if (!(match === term)) {
                            pre[i] = match
                        }
                        i += 1;
                    }

                    setSearchQuery(pre.join(" "))
                    updatedConfig = {
                        ...config,
                        search_params: {
                            ...config.search_params,
                            q: pre.join(" ")
                        }
                    }

                }
                
                else {
                    updatedConfig = {
                        ...config,
                        search_params: {
                            ...config.search_params,
                            q: searchQuery
                        }
                    }
                }

                await axios.post("http://localhost:9797/search/fill", updatedConfig)
                    .then((response) => {
                        console.log(response.data)
                    })
                    .catch((error) => {
                        console.error(error)
                    })

                try {
                    const response = await axios.post('http://localhost:9797/search/get-results', updatedConfig)
                    
                    const duration = window.performance.now() - start
                    var searchResults = []
                    var ranked = 0
                    var indexed = 0;

                    let idCounter = 1;

                    for (const item of response.data) {
                        if ('MetaSearch' in item) {
                            let result = item.MetaSearch;
                            result.type = 'meta';
                            result.id = idCounter++;
                            searchResults.push(item.MetaSearch);
                            
                        } else if ('Search' in item) {
                            indexed = item.Search.indexed;
                            ranked = item.Search.results.length;
                            for (const result of item.Search.results) {
                                result.type = 'local';
                            }
                            searchResults.push(...item.Search.results);
                        }
                    }
                    
                    setSearchResults(searchResults)
                    setPerformance({"Indexed": indexed, "Ranked": ranked, "Time": convertMsToTime(duration)}) 
                    setLoadingResults(false)
                    setSearchBarOffset(0)

                    } catch (error) {
                        console.error(error)
                        setLoadingResults(false)
                    }
                
                    if (user != null) {
                        try {
                            const result = searchResults.map(({ url, title }) => ({ 
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

                    
                }
            
                handleSearch()
            }

    }, [search])

    useEffect(() => {
    const fetchSummaries = async () => {
        try {
            console.log("Search results: ", searchResults)
           
            // Initialize the current ID to 1 (or any starting number you prefer)
            let currentId = 1;

            // Might need to modify to generate some random integer ID
            // that has not been used so far.
            const prompts = searchResults.slice(0, searchResults.length - 1).reduce((acc, result) => {
                // If result.id is undefined, use the currentId as default
                const id = result.id !== undefined ? result.id : currentId++;
                
                // Use sliced content later (make sure to join on " " first).
                // Add to the accumulator with the id as the key
                acc[id] = result.description ? result.description : "";
                
                // Increment regardless
                currentId++

                return acc;
            }, {});

            console.log("Obtained prompts for summaries: ", prompts);
            
            let responses;
            
            try {
                responses = await axios.post('http://127.0.0.1:5000/summarise', prompts); 
                
            } catch (error) {
                console.error("Error during summary request: ", error.response ? error.response.data : error.message);
            }
            
            console.log("TinyGPT responses: ", responses.data);
            setSummaries(responses.data);


        } catch (error) {
            console.error("There was an error creating summaries: ", error);
        }
    };

    fetchSummaries();
    }, [searchResults]);

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
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <SearchBar searchQuery={searchQuery} searchBarOffset={searchBarOffset} setSearchQuery={setSearchQuery} setSearch={setSearch} completion={completion}/>
                <AutoSuggestions setCompletion={setCompletion} singleSuggestions={singleSuggestions}/> 
            </div>
        </div>
        ) : (
        <motion.div 
            style={{display: 'flex', flexDirection: 'column', position:'relative', justifyContent: 'center', alignItems: 'center', gap: '5rem'}}
            initial={{opacity: 0, y: 0}}
            animate={{opacity: 1, y: -10}}
            transition={{duration: 0.5}}
        > 
        <SearchBar searchQuery={searchQuery} setSearchQuery={setSearchQuery} setSearch={setSearch} completion={completion}/>
            <div style={{display:'flex', position:'relative', alignItems:'left', justifyContent:'left', textAlign:'left', flexDirection:'column'}}>
            {(!loadingResults) && <p style={{fontFamily:'helvetica', color:'darkslateblue', fontWeight:'bold'}}> [{performance["Ranked"]} search results were ranked in {performance["Time"]}]</p>}
            {!(loadingResults) ? (
                searchResults.map((document, index) => (
                    <div key={index} style={{display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center'}}>
                        <div onClick={() => window.open(document.url, '_blank')} key={index}>
                            <SearchResult document={document} />
                        </div>
                        <div>
                        {
                            summaries[document.id] ? (
                                <Summary summary={summaries[document.id]} key={document.id}/> 
                            ) : (
                                <div style={{display: 'flex', flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: '0.5rem'}}>
                                    <p style={{fontWeight: 'bold', fontFamily: 'helvetica'}}> LOADING SUMMARY </p> 
                                    <ClipLoader color="#800080" size={15} />
                                </div>
                            )
                        }
                        </div>
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
