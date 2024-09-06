import { SelectConfig, BoxConfig, MultiSelectConfig } from '../components/Config';
import { useState } from 'react';
import { writeConfig } from '../config_utilities';
import { useAuth } from '../AuthProvider';


const settings = () => {
    const [searchMethod, setSearchMethod] = useState({"Document Clustering": true, "PageRank": false});
    const [indexType, setIndexType] = useState({"Document-Term": true, "Inverted": false, "B-Tree": false});
    const [altSearchParams, setAltSearchParams] = useState({"Crawl depth": 1, "Number of seed domains": 30});
    const [browsers, setBrowsers] = useState({"Google": true, "DuckDuckGo": false});

    const {config, setConfig} = useAuth();

    const indexMap = {
        'Document-Term': 0,
        'Inverted': 1,
        'B-Tree': 2
    }

    const searchMethodMap = {
        'Document Clustering': 0,
        'PageRank': 1
    }

    const collectSearchParameters = () => {
        const searchParameters = {
            'crawl_depth': parseInt(altSearchParams['Crawl depth'], 10),
            'number_of_seeds': parseInt(altSearchParams['Number of seed domains'], 10),
            'search_method': searchMethodMap[Object.keys(searchMethod).find(key => searchMethod[key])],
            'browsers': browsers,
            'index_type': indexMap[Object.keys(indexType).find(key => indexType[key])],
            'q': ''
        }
        console.log('Search Parameters: ', searchParameters)
        return searchParameters
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '2rem'}}>
            <h1 style={{fontFamily: 'helvetica', fontWeight: '500', fontSize: '2.5rem'}}> Search Settings </h1>
            <SelectConfig title="Index Type" state={indexType} setState={setIndexType}/>
            <SelectConfig title="Search Method" state={searchMethod} setState={setSearchMethod}/>
            <BoxConfig title="Search Parameters" state={altSearchParams} setState={setAltSearchParams}/>
            <h1 style={{fontFamily: 'helvetica', fontWeight: '500', fontSize: '2.5rem'}}> Meta-Search Settings </h1>
            <MultiSelectConfig title="Engines" state={browsers} setState={setBrowsers}/>
            <button onClick={() => writeConfig({
                ...config, 
                'search_params': collectSearchParameters() 
            }, setConfig)}> Save </button>
        </div>
   ) 
}


export default settings;
