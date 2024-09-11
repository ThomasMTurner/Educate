import {useState, useEffect} from 'react';

const AutoComplete = (props) => {
    const [completeColour, setCompleteColour] = useState('white');

    return (
    <div style={{display: 'flex', flexDirection: 'row', width: '30rem', position: 'relative'}}>
        <li onClick={() => props.setCompletion(props.suggestion.key)} onMouseEnter={() => setCompleteColour('gray') } onMouseLeave={() => setCompleteColour('white')} key={props.index} style={{color: completeColour, listStyleType: 'none'}}> {props.suggestion.key}</li>
    </div>
    )
}

export const AutoSuggestions = (props) => {
    return (
        <div style={{display: 'flex', position: 'relative', top:'1rem', flexDirection: 'column', width: '30rem', justifyContent: 'center', alignItems: 'center', alignSelf: 'center'}}>
        <h1 style={{fontSize:'1rem'}}> Autosuggestions </h1>
        {
        <div>
        <ul>
            {props.suggestions.map((suggestion, index) => {
                console.log('Mapped suggestion: ', suggestion);
                return (
                  <AutoComplete index={index} suggestion={suggestion} setCompletion={props.setCompletion} /> 
                );
            })}
        </ul>
        </div>
        }
        </div>
    )
}

