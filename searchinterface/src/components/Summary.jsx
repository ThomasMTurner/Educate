import styles from '../styles/component-styles/Summary.module.css';

const Summary = ({summary}) => {
    return (
        <div onClick={() => window.open(url, '_blank')} className={styles.summaryContainer}>
            <p> {summary} </p>
        </div>
    )
}

export default Summary;
