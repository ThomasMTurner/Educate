import {AiOutlineSearch} from 'react-icons/ai';
import {useState} from 'react';
import styles from '../styles/component-styles/SearchBar.module.css';

const SearchBar = ({searchQuery, setSearchQuery, setSearch, searchBarOffset, completion}) => {
    const [searchIconColour, setSearchIconColour] = useState('grey');

    const getQueryUpdatedWithCompletion = () => {
        let newQuery = searchQuery.split(" ");
        newQuery[newQuery.length - 1] = completion;
        return newQuery.join(" ")
    }

    const handleKeyPress = (e) => {
        if (e.key === 'Enter') {
            setSearch(true);
        }
        if (e.key === 'Shift') {
            setSearchQuery(getQueryUpdatedWithCompletion());
        }
    }

    return (
        <div className={styles.SearchBarContainer} style={{left: `${searchBarOffset}rem`}}>
             <input
                type="text"
                onKeyDown={handleKeyPress}
                className="search-input"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                style={{
                    position: 'relative',
                    display: 'inline-block',
                    paddingRight: '4rem',
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
