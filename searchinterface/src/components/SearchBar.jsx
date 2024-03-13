import {AiOutlineSearch} from 'react-icons/ai';
import {useState, useEffect} from 'react';
import styles from '../styles/component-styles/SearchBar.module.css';

const SearchBar = ({setSearchQuery, setSearch, searchBarPosition}) => {
    const [searchIconColour, setSearchIconColour] = useState('grey');
    const [searchQueryTemp, setSearchQueryTemp] = useState('');
    
    useEffect(() => {
        setSearchQuery(searchQueryTemp);
    }, [searchQueryTemp]);


    return (
        <div className={styles.SearchBarContainer}>
             <input
                type="text"
                className="search-input"
                value={searchQueryTemp}
                onChange={(e) => setSearchQueryTemp(e.target.value)}
                style={{
                    position: 'fixed',
                    padding: '1rem',
                    border: 'none',
                    width: '25rem',
                    fontFamily: 'helvetica',
                    fontSize: '0.8rem',
                    boxShadow: '2px 2px 4px 4px solid black'
                }}
            />
            
            <AiOutlineSearch
                onClick={() => setSearch(true)}
                onMouseEnter={() => setSearchIconColour('white')}
                onMouseLeave={() => setSearchIconColour('grey')}
                size={25}
                color={searchIconColour}
                style={{
                    position: 'fixed',
                    top: '-0.7rem',
                    left: `${searchBarPosition}rem`
                }}
            /> 
        </div>
    );
};

export default SearchBar;
