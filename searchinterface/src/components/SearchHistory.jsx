import styles from '../styles/component-styles/SearchHistory.module.css';

const SearchHistory = ({title, url, date}) => {
    return (
        <div onClick={() => window.open(url, '_blank')} className={styles.searchHistoryContainer}>
            <p style={{fontWeight: 600, color: 'white'}}> {title} </p>
            <p style={{color: 'gray'}}> {(url)} </p>
            <p style={{textDecoration: 'underline', textAlign:'right', color: 'darkblue'}}> {date} </p>
        </div>
    )
}

export default SearchHistory;
