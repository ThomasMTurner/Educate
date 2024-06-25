import styles from '../styles/component-styles/SearchHistory.module.css';

const SearchHistory = ({title, url, date}) => {
    return (
        <div onClick={() => window.open(url, '_blank')} className={styles.searchHistoryContainer}>
            <p> {title} </p>
            <p> {url} </p>
            <p> {date} </p>
        </div>
    )
}

export default SearchHistory;
