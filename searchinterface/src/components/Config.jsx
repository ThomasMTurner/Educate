export const SelectConfig = (props) => {
console.log("Select config state: ", props.state)
return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
        <h1 style={{ fontSize: '1rem' }}>{props.title}</h1>
        <div style={{ display: 'flex', flexDirection: 'column', gap:"1rem"}}>
        {Object.entries(props.state).map(([k, v], index) => (
                <button
                    key={index}
                    style={{ backgroundColor: v ? 'green' : 'red' }}
                    onClick={() =>
                        props.setState(prevState => ({
                            ...prevState,
                            [k]: true, // Set the clicked key's value to true
                            ...Object.keys(prevState).reduce((acc, key) => {
                                if (key !== k) acc[key] = false; // Set all other keys' values to false
                                return acc;
                            }, {})
                        }))
                    }
                > {k} </button>
        ))}
        </div>
    </div>
);

}

export const TickConfig = (props) => {
    return (
        <div style={{display: 'flex', flexDirection:'row', gap: '2rem', justifyContent: 'left', alignItems: 'center'}}>
            <h1 style={{fontSize: '1rem', fontWeight: '500', fontFamily: 'helvetica'}}> {props.title} </h1>
            <input style={{width:"1.5rem", height:"1.5rem"}} type="checkbox" checked={props.state}
                 onChange={(_) => props.setState(!props.state)}/>
        </div>
    )
}

export const MultiSelectConfig = (props) => {
    return  ( 
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem'}}>
            <h1 style={{ fontSize: '1rem'}}>{props.title}</h1>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem'}}>
            {Object.entries(props.state).map(([k, v], index) => (
                <button
                    key={index}
                    style={{ backgroundColor: v ? 'green' : 'red' }}
                    onClick={() => 
                        props.setState(prevState => ({
                            ...prevState,
                            [k]: !v
                        }))
                    }
                > {k} </button>
            ))}
            </div>
        </div>
    )
}

export const BoxConfig = (props) => {
    return (
        <div>
            <h1 style={{ fontSize: '1rem' }}>{props.title}</h1>
            {Object.entries(props.state).map(([k ,v], index) => (
                <div key={k} style={{display:"flex", flexDirection:"row", alignItems:"center", gap:"1rem"}}>
                <p>{k}</p>
                <form>
                    <input style={{width:"2rem", height:"1.3rem"}} placeholder={v} onChange={(e) => props.setState(prevState => ({...prevState, [k]: e.target.value}))} ></input>
                </form>
                </div>
            ))}
        </div>
    )
};

