import {useState, useEffect} from 'react';
import { IoPencil } from "react-icons/io5";
import { CiSearch } from "react-icons/ci";

// TO DO:
// (1) Delineate (icon for single completion)
// (2) When hovering, reset colours.
const SingleAutoComplete = (props) => {
    const [completeColour, setCompleteColour] = useState('#1a1a1a');
    console.log(props.suggestion);

    return (
    <div onClick={() => props.setCompletion(props.suggestion.key)} style={{display: 'flex', flexDirection: 'row', width: '30rem', gap:'0.5rem', position: 'relative', backgroundColor:completeColour, border: '0.7px solid #333333', 
                right:'1.8rem', padding: '1rem', alignItems: 'center', height:'0.5rem', borderRadius:'0.1rem'}}  onMouseEnter={() => setCompleteColour('#333333') } onMouseLeave={() => setCompleteColour('#1a1a1a')}>
        <li key={props.index} style={{color: 'white', listStyleType: 'none'}}> {props.suggestion.key}</li>
        <IoPencil size={20}/>
    </div>
    )
}

export const AutoSuggestions = (props) => {
    let suggestions = [];
    // TO DO: make as pairs with 'Single' to identify which component to render.
    suggestions.push(props.singleSuggestions);

    return (
        <div style={{display: 'flex', position: 'relative', top:'1rem', flexDirection: 'column', width: '30rem', justifyContent: 'center', alignItems: 'center', alignSelf: 'center'}}>
        {
        <div>
        <ul>
            {props.singleSuggestions.map((suggestion, index) => {
                return (
                  <SingleAutoComplete index={index} suggestion={suggestion} setCompletion={props.setCompletion} /> 
                );
            })}
        </ul>
        </div>
        }
        </div>
    )
}

