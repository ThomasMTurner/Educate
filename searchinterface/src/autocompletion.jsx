import { distance, closest } from 'fastest-levenshtein';
import { useAuth } from './AuthProvider';

// Have the following requirements:
// (1) Utilising trie-search package to provide autocompletions stream. Preferably, this should accumulate as query
// state is updated. Furthermore, we need to download an existing JSON dataset for prefixes.
// (2) Utilising fastest-levenshtein package in order to provide typo corrections (or suggestions - but corrections need
// to be strict for many of the document clustering methods to work).

// Trie search autcompletions logic. 
// If event updates are not made quickly enough, made require reactive programming with observable streams (our original approach).

export const getCompletion = (query, setCompletion) => {
    const { trie } = useAuth();
    setCompletion(trie.search(query));
}

// Levenshtein typo correction logic (post-processing so no stream required).

