import styles from '../styles/component-styles/SearchResult.module.css'

// TO DO:
// API call to local Llama instance to summarise incoming document parameter.
// Click event causes default browser to open (currently just macOS support).
// Also want to make use of description.
// Fix formatting when you have the time.


const SearchResult = ({document, summary}) => {
    return (
        <div className={styles.SearchResultContainer}>
            <h1 style={{fontSize:'0.7rem', fontWeight: 'bold', color:'whitesmoke'}}> {document.type === 'meta' ? document.engine: 'Local'} </h1>
            <h1 style={{fontSize:'0.7rem', fontWeight:'200', color:'gray'}}>
               {document.url}</h1>
            <h1 style={{fontSize:'1.2rem', fontWeight:'500', color: 'aqua'}}>
                {document.title}</h1>
            <div style={{display:'flex', flexDirection:'row'}}>
                             
        {document.images && (
            <>
                <p>Images:</p>
                {document.images.map((image, index) => (
                    <div key={index} style={{ width: '200px', height: '200px', position: 'relative', overflow: 'hidden' }}>
                        <img 
                            src={image} 
                            style={{ 
                                display: 'block', 
                                maxWidth: '100%', 
                                maxHeight: '100%', 
                                position: 'absolute', 
                                top: '50%', 
                                left: '50%', 
                                transform: 'translate(-50%, -50%)' 
                            }} 
                        />
                </div>
                ))}
            </>
        )}

            </div>
            <h1 style={{fontStyle: 'helvetica', fontWeight: '200', fontSize:'0.8rem', color:'whitesmoke'}}><b style={{fontSize:'1rem'}}>Summarised by Ollama:</b> {summary}</h1>
        </div>
    )
}


export default SearchResult;
