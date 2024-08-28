import { SelectConfig, BoxConfig, MultiSelectConfig } from '../components/Config';
import { useState } from 'react';
import { writeConfig } from '../config_utilities';
import { useAuth } from '../AuthProvider';

/* 
POSSIBLE TO DO: currently the search configuration is only saved on the button press event.
May wish to enable writing to a swap file or buffer in the event of the application crashing.
*/

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

    const configWriter = (updatedSearchParameters) => {
        let updatedConfig = {...config, search_params: updatedSearchParameters}
        console.log(updatedConfig);
        setConfig(updatedConfig);
        writeConfig(updatedSearchParameters);
    }

    const collectSearchParameters = () => {
        const searchParameters = {
            'crawl_depth': altSearchParams['Crawl depth'],
            'number_of_seeds': altSearchParams['Number of seed domains'],
            'search_method': searchMethodMap[Object.keys(searchMethod).find(key => searchMethod[key])],
            'browsers': browsers,
            'index_type': indexMap[Object.keys(indexType).find(key => indexType[key])],
            'q': ''
        }
        console.log('Obtained search parameters', searchParameters)
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
            <button onClick={() => configWriter(collectSearchParameters())}> 
                Save </button>
        </div>
   ) 
}

export default settings;
