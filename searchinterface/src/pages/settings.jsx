import { SelectConfig, BoxConfig, MultiSelectConfig, TickConfig } from '../components/Config';
import { useState, useEffect } from 'react';
import { writeConfig } from '../config_utilities';
import { useAuth } from '../AuthProvider';


const getIndexTypeAndSearchMethodState = (config, setIndexTypeState) => {
    const indexInverseMap = {
        0: 'Document-Term',
        1: 'Inverted',
        2: 'B-Tree'
    }

    let setIndexType = {'Document-Term': false, 'Inverted': false, 'B-Tree': false};
    setIndexType[indexInverseMap[config['search_params']['index_type']]] = true;
    let searchMethodInverseMap;
    let setSearchMethodState;

    if (Object.keys(setIndexTypeState).length > 0) {
    console.log("USING INDEX TYPE STATE");
    if (setIndexTypeState['Document-Term'])  {
        searchMethodInverseMap = {
            0: 'Document Clustering with Word2Vec',
            1: 'Document Clustering with Doc2Vec',
            2: 'Document Clustering with Sentence Transformers',
        }
        setSearchMethodState = {"Document Clustering with Word2Vec": false, "Document Clustering with Doc2Vec": false, "Document Clustering with Sentence Transformers": false}
        setSearchMethodState[searchMethodInverseMap[0] = true];
    }

    else if (setIndexTypeState['Inverted']) {
        console.log('INDEX TYPE STATE WAS INVERTED')
        searchMethodInverseMap = {
            3: 'TF-IDF Search Ranking'
        }
        setSearchMethodState = {"TF-IDF Search Ranking": false};
        setSearchMethodState[searchMethodInverseMap[3] = true];
    }

    else {
        searchMethodInverseMap = {}
        setSearchMethodState = {};
    }
    }

    else {

    if (setIndexType['Document-Term'])  {
        searchMethodInverseMap = {
            0: 'Document Clustering with Word2Vec',
            1: 'Document Clustering with Doc2Vec',
            2: 'Document Clustering with Sentence Transformers',
        }
        setSearchMethodState = {"Document Clustering with Word2Vec": false, "Document Clustering with Doc2Vec": false, "Document Clustering with Sentence Transformers": false}
    }

    else if (setIndexType['Inverted']) {
        searchMethodInverseMap = {
            3: 'TF-IDF Search Ranking'
        }
        setSearchMethodState = {"TF-IDF Search Ranking": false};
    }

    else {
        searchMethodInverseMap = {}
        setSearchMethodState = {};
    }
    
    setSearchMethodState[searchMethodInverseMap[config['search_params']['search_method']]] = true;
    }



    return {
        setIndexTypeState: setIndexType,
        searchMethodState: setSearchMethodState
    }
}


const settings = () => {
    const { config, setConfig } = useAuth();
    const {setIndexTypeState, searchMethodState } = getIndexTypeAndSearchMethodState(config, {});
    console.log("Index Type: ", setIndexTypeState);
    console.log("Search Method State: ", searchMethodState);
    const [searchMethod, setSearchMethod] = useState(searchMethodState);
    const [indexType, setIndexType] = useState(setIndexTypeState);
    const [altSearchParams, setAltSearchParams] = useState({"Crawl depth": config.search_params.crawl_depth, 
        "Number of seed domains": config.search_params.number_of_seeds});
    const [browsers, setBrowsers] = useState(config.search_params.browsers); 
    // Replace with config.autosuggest & config.query_correction.
    const [checkedSuggestions, setCheckedSuggestions] = useState(config.autosuggest);
    const [checkedQueryCorrection, setCheckedQueryCorrection] = useState(config.query_correction);
    
    const indexMap = {
        'Document-Term': 0,
        'Inverted': 1,
        'B-Tree': 2
    }

    const searchMethodMap = {
        'Document Clustering with Word2Vec': 0,
        'Document Clustering with Doc2Vec': 1,
        'Document Clustering with Sentence Transformers': 2,
        'TF-IDF Search Ranking': 3
    }

    useEffect(() => {
        console.log("Changed index type HELLO HELLO THERE");
        const { _, searchMethodState } = getIndexTypeAndSearchMethodState(config, indexType);
        console.log("NEW SEARCH METHOD STATE:", searchMethodState);
        setSearchMethod(searchMethodState);
    }, [indexType])

    const collectSearchParameters = () => {
        console.log('Search method has initial value: ', searchMethod)
        console.log('Search method should be set: ', searchMethodMap[Object.keys(searchMethod).find(key => searchMethod[key])])

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
        let configUpdated = {...config, 'search_params': collectSearchParameters(), 
            'autosuggest': checkedSuggestions, 'query_correction': checkedQueryCorrection};
        console.log('Updated Config in Button press event: ', configUpdated)
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
            <h1 style={{fontFamily: 'helvetica', fontWeight: '500', fontSize: '2.5rem'}}> Other </h1>
            <TickConfig title='Autosuggestions with Trie Search (this will use your search history)' state={checkedSuggestions} setState={setCheckedSuggestions}/>
            <TickConfig title='Query Typo Correction with Levenshtein Distance' state={checkedQueryCorrection} setState={setCheckedQueryCorrection}/>
            <button onClick={() => updateAndWriteConfig()}> Save </button>
        </div>
   ) 
}


export default settings;
