import styles from '../styles/page.module.css';
import SearchBar from '../components/SearchBar';
import { IoIosSettings } from "react-icons/io";
import { FaHistory } from "react-icons/fa";
import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import ClipLoader from 'react-spinners/ClipLoader';
import axios from 'axios';
import SearchResult from '../components/SearchResult';

// Overriding the default timeout to 10 minutes while we speedup the search request.
axios.defaults.timeout = 600000;

const Home = () => {
  const [iconColours, setIconColours] = useState({"Settings": "gray", "History": "gray"})
  const [resultsScreen, setResultsScreen] = useState(false)
  const [searchQuery, setSearchQuery] = useState("")
  const [search, setSearch] = useState(false)
  const [searchResults, setSearchResults] = useState({})
  const [summaries, setSummaries] = useState({})
  const [loadingResults, setLoadingResults] = useState(false)
 
  const setIconColour = (iconName, colour) => {
    setIconColours(prev => ({...prev, [iconName]: colour}));
  }

  useEffect(() => {
        if (search) {
            const handleSearch = async () => {

                // Trigger search bar animation.
                setResultsScreen(true)

                // Trigger loading animation.
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
                
    

                // Call to rank existing indices based on search query.
                const response = await axios.post('http://localhost:9797/search/get-results', data)  
                const indexedResults = response.data.map((result, index) => {
                // Start indexing from 1
                    const id = index + 1;
                    return { ...result, id }
                })

                setSearchResults(indexedResults)
                
                // Call to summarise each search result
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
                <h1 style={{fontFamily: 'helvetica', fontWeight: '200'}}> Welcome to <b>Edu-cate Search.</b></h1> 
                <div style={{display: 'flex', position: "relative", gap: '1rem'}}>
                    <IoIosSettings size={25} color={iconColours.Settings} onMouseEnter={() => setIconColour("Settings", "black")} onMouseLeave={() => setIconColour("Settings", "gray")} />
                    <FaHistory size={25} color={iconColours.History} onMouseEnter={() => setIconColour("History", "black")} onMouseLeave={() => setIconColour("History", "gray")}/>
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
                <SearchBar searchBarPosition={10} setSearchQuery={setSearchQuery} setSearch={setSearch}/>
            </div>
        </div>
        ) : (
        <motion.div 
            style={{display: 'flex', flexDirection: 'column', gap:'4rem', position: 'relative', alignItems: 'center', justifyContent: 'center'}}
            initial={{opacity: 0 ,y: 0, x: 475}}
            animate={{opacity: 1, y: -200}}
            transition={{duration: 0.5}}
        >   
            { !(searchResults.length == 0) && (
                !(loadingResults) ? (
                    searchResults.map((document, index) => {
                        return (
                            <div key={index} style={{position:'relative', display: 'flex'}}>
                                <SearchResult document={document} summary={summaries[document.id]}/>
                            </div>
                        );
                    })
                ) : (
                    <div style={{display:'flex', position:'relative',flexDirection:'column', gap:'10rem', alignItems:'center', justifyContent:'center'}}>
                        <SearchBar searchBarPosition={24} containerPosition={0} setSearchQuery={setSearchQuery} setSearch={setSearch}/>
                        <div style={{position: 'relative', left:'13rem'}}>
                            <ClipLoader color="#52bfd9" size={40} />
                        </div>
                    </div>
                )
            )


            }
        </motion.div>
        
    )
  );
};
  


export default Home;
