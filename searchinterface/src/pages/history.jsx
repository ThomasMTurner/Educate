import { useAuth } from '../AuthProvider';
import SearchHistory from '../components/SearchHistory';


const history = () => {
    // Possibly naive approach - adding a query field to each history entry (hence some copying of data)
    // and performing the grouping logic in the rendering below.
    const { history, user } = useAuth();

    const groupByQuery = (history) => {
        return history.reduce((groups, item) => {
            const group = (groups[item.query] || []);
            group.push(item);
            groups[item.query] = group;
            return groups;
        }, {});
    };

    return ( 
        <div style={{display: 'flex', flexDirection: 'column', position: 'relative'}}>
            <p style={{fontFamily: 'helvetica', fontSize: '3rem', fontWeight: 'bold'}}> Recent History for <b style={{color: 'gray'}}>{user}</b> </p>
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
        </div>
    )
}


export default history;
