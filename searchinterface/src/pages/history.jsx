import { useAuth } from '../AuthProvider';
import SearchHistory from '../components/SearchHistory';


const history = () => {
    const { history, user } = useAuth();
    
    // Grouping Logic:
    // We require objects collected on the basis of the same query and the same date.
    // These should then be ordered by datetime.
    const groupAndSortByQueryDate = (history) => {
        // Sort by date.
        [].slice.call(history).sort((a, b) => new Date(b.date) - new Date(a.date));

        const groupedHistory = [].slice.call(history).reduce((acc, item) => {
            const key = `${item.query} ${item.date}`;
            if (!acc[key]) {
                acc[key] = [];
            }
            acc[key].push(item);
            return acc;
        }, {})
        
        console.log("Grouped History", groupedHistory);

        return groupedHistory;
    };

    const groupedAndSortedHistory = groupAndSortByQueryDate(history);

    return ( 
        <div style={{display: 'flex', flexDirection: 'column', position: 'relative'}}>
            <p style={{fontFamily: 'helvetica', fontSize: '3rem', fontWeight: 'bold'}}> Recent History for <b style={{color: 'gray'}}>{user}</b> </p>
            {
                Object.entries(groupedAndSortedHistory).map(([_, items], groupIndex) => (
                    <div key={`group-${groupIndex}`}>
                    <p style={{fontFamily: 'helvetica', fontSize: '1.3rem', 
                            fontWeight: '200', alignSelf:'left', textAlign: 'left', color:'gray', 
                            fontWeight: 'helvetica'}}> Searched for <b style={{fontWeight: 'bold', color: 'white'}}>{items[0].query}</b> at <b style={{fontWeight: '500', color: 'red'}}>({items[0].date})</b></p>
                        {items.map((item, itemIndex) => (
                            <SearchHistory
                                key={`history-${groupIndex}-${itemIndex}`}
                                url={item.url}
                                title={item.title}
                            />
                        ))} 
                    </div>
                ))
            } 
        </div>
    )
}

            /*
            {Object.entries(groupByQuery(history)).map(([query, items], groupIndex) => (
            <div key={`group-${groupIndex}`}>
            <p style={{fontFamily: 'helvetica', fontSize: '1.3rem', fontWeight: '200', alignSelf:'left', textAlign: 'left', color:'gray', fontWeight: 'helvetica'}}> Searched for <b style={{fontWeight: 'bold', color: 'white'}}>{query}</b> at <b style={{fontWeight: '500', color: 'red'}}>({items[0].date})</b></p>
                {items.map((item, itemIndex) => (
                    <SearchHistory 
                        key={`history-${groupIndex}-${itemIndex}`}
                        url={item.url}
                        title={item.title}
                    />
                ))}
            </div>
            ))}
            */



export default history;
