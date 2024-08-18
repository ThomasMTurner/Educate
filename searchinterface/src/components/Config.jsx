export const SelectConfig = (props) => {
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
                    <input style={{width:"2rem", height:"2rem"}}onChange={(e) => props.setState(prevState => ({...prevState, [k]: e.target.value}))} ></input>
                </form>
                </div>
            ))}
        </div>
    )
};

