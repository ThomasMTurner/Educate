import styles from '../styles/component-styles/SearchResult.module.css'

// TO DO:
// API call to local Llama instance to summarise incoming document parameter.
// Click event causes default browser to open (currently just macOS support).

const SearchResult = ({document, summary}) => { 
    return (
        <div className={styles.SearchResultContainer}>
            <h1 style={{fontSize:'0.7rem', fontWeight:'200', color:'gray'}}>{document.url}</h1>
            <h1 style={{fontSize:'1.5rem', fontWeight:'bold', color:'white'}}>{document.title}</h1>
            <h1 style={{fontWeight: '200', fontSize:'0.8rem', color:'gray'}}><b style={{fontSize:'1rem'}}>AI Summary:</b> {summary}</h1>
        </div>
    )
}


export default SearchResult;
