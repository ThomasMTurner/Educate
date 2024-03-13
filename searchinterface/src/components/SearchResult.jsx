import styles from '../styles/component-styles/SearchResult.module.css'

// TO DO:
// 1. SearchResult component to display title + short segment of description + url (see other components usually included).


const SearchResult = ({document}) => { 
    return (
        <div className={styles.SearchResultContainer}>
            <h1 style={{fontSize:'0.7rem', fontWeight:'200'}}>{document.url}</h1>
            <h1 style={{fontSize:'1.5rem', fontWeight:'500'}}>{document.title}</h1>
            <h1 style={{fontWeight: '300', fontSize:'0.8rem'}}>{document.content.slice(0, 30)}</h1>
        </div>
    )
}


export default SearchResult;
