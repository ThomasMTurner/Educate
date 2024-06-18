import {AiOutlineSearch} from 'react-icons/ai';
import {useState, useEffect} from 'react';
import styles from '../styles/component-styles/SearchBar.module.css';

const SearchBar = ({setSearchQuery, setSearch, searchBarOffset}) => {
    const [searchIconColour, setSearchIconColour] = useState('grey');
    const [searchQueryTemp, setSearchQueryTemp] = useState('');
    
    useEffect(() => {
        setSearchQuery(searchQueryTemp);
    }, [searchQueryTemp]);

    const handleKeyPress = (e) => {
        if (e.key === 'Enter') {
            setSearch(true);
        }
    }

    return (
        <div className={styles.SearchBarContainer} style={{left: `${searchBarOffset}rem`}}>
             <input
                type="text"
                onKeyDown={handleKeyPress}
                className="search-input"
                value={searchQueryTemp}
                onChange={(e) => setSearchQueryTemp(e.target.value)}
                style={{
                    position: 'relative',
                    padding: '1rem',
                    border: 'none',
                    width: '50rem',
                    fontFamily: 'helvetica',
                    fontSize: '0.8rem',
                    boxShadow: '2px 2px 4px 4px solid black',
                    left:'2.5rem'
                }}
            />
            
            <AiOutlineSearch
                onClick={() => setSearch(true)}
                onMouseEnter={() => setSearchIconColour('white')}
                onMouseLeave={() => setSearchIconColour('grey')}
                size={50}
                color={searchIconColour}
                style={{
                    zIndex:1
                }}
            /> 
        </div>
    );
};

export default SearchBar;
