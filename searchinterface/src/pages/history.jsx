import { useAuth } from '../AuthProvider';
import {useState, useEffect} from 'react';
import SearchHistory from '../components/SearchHistory';
import { AiOutlineSearch } from "react-icons/ai";


const HistorySearchBar = (props) => {

    return (
        <div style={{display: 'flex', position: 'relative', flexDirection: 'row', alignItems: 'center', justifyContent: 'center', textalign: 'center', gap:'-2rem'}}>
           
            <input
                type="text"
                //onKeyDown={}
                className="search-input"
                value={props.historyQuery}
                placeholder="Search history"
                onChange={(e) => props.setHistoryQuery(e.target.value)}
                style={{
                    position: 'relative',
                    display: 'inline-block',
                    padding: '0.55rem',
                    border: 'none',
                    width: '10rem',
                    fontFamily: 'helvetica',
                    fontSize: '0.8rem',
                    boxShadow: '2px 2px 4px 4px solid black',
                    left:'2.5rem'
                }}
            />
            <AiOutlineSearch
                //onClick={() => setSearch(true)}
                //onMouseEnter={() => setSearchIconColour('white')}
                size={20}
                style={{
                    position:'relative',
                    left:'0.9rem',
                    zIndex:1,
                }}
            /> 
            
        </div>
    )
}

const SortHistory = (props) => {
    const [backgroundColour, setBackgroundColour] = useState('#454545');

    return (
    <div
    onMouseEnter={() => setBackgroundColour('green')}
    onMouseLeave={() => setBackgroundColour('#454545')}
    onClick={() => {
        props.setSortType(props.sortType);
        setBackgroundColour('green');
    }}
    style={{display: 'flex', flexDirection: 'row', gap: '1rem', backgroundColor:backgroundColour, paddingRight:'0.5rem', paddingLeft:'0.5rem', alignItems: 'center', justifyContent: 'center', textalign: 'center', height:'2rem', boxShadow: '2px 2px 4px 4px solid black'}}
    >
            <p style={{color: 'whitesmoke'}}> Sort by {props.sortType} </p>
        </div>
    )
}

const history = () => {
    let { history, user } = useAuth();
    const [sortType, setSortType] = useState('date');
    const [groupedHistory, setGroupedHistory] = useState({});
    const [historyQuery, setHistoryQuery] = useState('')

    useEffect(() => {
        console.log("New history query: ", historyQuery)
        console.log("History before filtering: ", history)
        history = history.filter(history => history.query.startsWith(historyQuery))
        console.log("New history filtered: ", history)
        setGroupedHistory(groupAndSortHistory(history, sortType));
    }, [historyQuery])
    

const groupAndSortHistory = (history, sortType) => {
    console.log(sortType);

    let sortedHistory = [...history]; // Create a copy of the history array

    switch (sortType) {
        case 'date':
            sortedHistory.sort((a, b) => new Date(b.date) - new Date(a.date));
            sortedHistory = sortedHistory.reverse();
            break;
        case 'search term':
            sortedHistory.sort((a, b) => a.query.localeCompare(b.query));
            break;
    }

    const groupedHistory = sortedHistory.reduce((acc, item) => {
        const key = `${item.query} ${item.date}`;
        if (!acc[key]) {
            acc[key] = [];
        }
        acc[key].push(item);
        return acc;
    }, {});
    
    console.log("Grouped History", groupedHistory);

    return groupedHistory;
};

    useEffect(() => {
        setGroupedHistory(groupAndSortHistory(history, sortType));
    }, [sortType])

    return ( 
        <div style={{display: 'flex', flexDirection: 'column', position: 'relative'}}>
            <p style={{fontFamily: 'helvetica', fontSize: '3rem', fontWeight: 'bold'}}> Recent History for <b style={{color: 'gray'}}>{user}</b> </p>
            <div style={{display:'flex', flexDirection: 'row', gap: '3rem', paddingBottom:'2rem'}}>
                <HistorySearchBar historyQuery={historyQuery} setHistoryQuery={setHistoryQuery}/>
                <SortHistory sortType="date" setSortType={setSortType} />
                <SortHistory sortType="search term" setSortType={setSortType} />
            </div>
            {
                Object.entries(groupedHistory).map(([_, items], groupIndex) => (
                    <div key={`group-${groupIndex}`}>
                    <p style={{fontFamily: 'helvetica', fontSize: '1.3rem', 
                            fontWeight: '200', alignSelf:'left', textAlign: 'left', color:'gray', 
                            fontWeight: 'helvetica'}}> Searched for <b style={{fontWeight: 'bold', color: 'white'}}>{items[0].query}</b> on <b style={{fontFamily: 'helvetica', fontWeight: '200', color: 'green'}}>[{items[0].date}]</b></p>
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
