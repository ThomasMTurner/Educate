import { SelectConfig, BoxConfig, MultiSelectConfig } from '../components/Config';
import { useState } from 'react';
import { writeConfig } from '../config_utilities';
import { useAuth } from '../AuthProvider';

const getSearchParametersState = (config) => {
    const searchMethodInverseMap = {
        0: 'Document Clustering with Word2Vec',
        1: 'Document Clustering with Doc2Vec',
        2: 'Document Clustering with Sentence Transformers',
        3: 'TF-IDF Search Ranking'
    }
    let setSearchParametersState = {"Document Clustering with Word2Vec": false, "Document Clustering with Doc2Vec": false, "Document Clustering with Sentence Transformers": false, "TF-IDF Search Ranking": false, "PageRank": false};
    setSearchParametersState[searchMethodInverseMap[config['search_params']['search_method']]] = true;
    return setSearchParametersState;
}

const getIndexTypeState = (config) => {
    const indexInverseMap = {
        0: 'Document-Term',
        1: 'Inverted',
        2: 'B-Tree'
    }
    let setIndexType = {'Document-Term': false, 'Inverted': false, 'B-Tree': false};
    setIndexType[indexInverseMap[config['search_params']['index_type']]] = true;
    return setIndexType;
}


const settings = () => {
    const { config, setConfig } = useAuth();
    const [searchMethod, setSearchMethod] = useState(getSearchParametersState(config));
    const [indexType, setIndexType] = useState(getIndexTypeState(config));
    const [altSearchParams, setAltSearchParams] = useState({"Crawl depth": config.search_params.crawl_depth, 
        "Number of seed domains": config.search_params.number_of_seeds});
    const [browsers, setBrowsers] = useState(config.search_params.browsers); 
    

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

    const updateAndWriteConfig = () => {
        let configUpdated = {...config, 'search_params': collectSearchParameters()};
        writeConfig(configUpdated);
        setConfig(configUpdated);
    }

    return (
        <div style={{display: 'flex', flexDirection: 'column', gap: '2rem'}}>
            <h1 style={{fontFamily: 'helvetica', fontWeight: '500', fontSize: '2.5rem'}}> Search Settings </h1>
            <SelectConfig title="Index Type" state={indexType} setState={setIndexType}/>
            <SelectConfig title="Search Method" state={searchMethod} setState={setSearchMethod}/>
            <BoxConfig title="Search Parameters" state={altSearchParams} setState={setAltSearchParams}/>
            <h1 style={{fontFamily: 'helvetica', fontWeight: '500', fontSize: '2.5rem'}}> Meta-Search Settings </h1>
            <MultiSelectConfig title="Engines" state={browsers} setState={setBrowsers}/>
            <button onClick={() => updateAndWriteConfig()}> Save </button>
        </div>
   ) 
}


export default settings;
