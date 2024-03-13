import styles from '../styles/page.module.css';
import SearchBar from '../components/SearchBar';
import { IoIosSettings } from "react-icons/io";
import { FaHistory } from "react-icons/fa";
import { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import ClipLoader from 'react-spinners/ClipLoader';
//import { TbArrowBackUp } from "react-icons/tb";
import axios from 'axios';
import SearchResult from '../components/SearchResult';


const Home = () => {
  const [iconColours, setIconColours] = useState({"Settings": "gray", "History": "gray"})
  const [resultsScreen, setResultsScreen] = useState(false)
  const [searchQuery, setSearchQuery] = useState("")
  const [search, setSearch] = useState(false)
  const [searchResults, setSearchResults] = useState({})
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

                await axios.get("http://localhost:9797/search/fill")
                    .then((response) => {
                        console.log(response.data)
                    })
                    .catch((error) => {
                        console.error(error)
                    })

                // Deal with API call here to collect search results (axios)
                // Make the POST request
                await axios.post('http://localhost:9797/search/get-results', data)
                    .then(response => {
                    // Handle the response
                    setSearchResults(response.data);
                    console.log(response.data);
                })
                .catch(error => {
                    // Ha
                    // ndle the error
                    console.error('There was an error!', error);
                });

       
                // Generate an array of search results data, such that a set
                // of SearchResult components can be rendered capturing each result.
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
                <SearchBar searchBarPosition={23} setSearchQuery={setSearchQuery} setSearch={setSearch}/>
            </div>
        </div>
        ) : (
        <motion.div 
            style={{display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent:'center', textAlign:'center'}}
            initial={{opacity: 0 ,y: 0, x: 475}}
            animate={{opacity: 1, y: -200}}
            transition={{duration: 0.5}}
        >
            <div>
                <SearchBar searchBarPosition={18} setSearchQuery={setSearchQuery} setSearch={setSearch}/>
            </div>
            { !(searchResults.length == 0) && (
                !(loadingResults) ? (
                    documents.map((doc, index) => {
                        return (
                            <div key={index}>
                                <SearchResult document={document}/>
                            </div>
                        );
                    })
                ) : (
                    <div 
                        style={{position: 'relative', top:'10rem'}}
                        initial={{opacity: 0 }}
                        animate={{opacity: 1}}
                        transition={{duration: 0.5}} 
                    >
                        <ClipLoader color="#52bfd9" size={40} />
                    </div>
                )
            )


            }
        </motion.div>
        
    )
  );
};

export default Home;
