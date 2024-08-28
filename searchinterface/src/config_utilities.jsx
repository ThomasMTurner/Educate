import axios from 'axios';
import { useAuth } from '.../../AuthProvider';

export async function writeConfig (newSearchParameters) {
    console.log('Writing new config');
    
    // Write new config.
    const response = await axios.post("http://localhost:9797/config/write", config);
    console.log('Obtained response from config writing: ', response.data);
}

export const readConfig = async (conf, setConf) => {
    try {
        const conf_resp = await axios.post("http://localhost:9797/config/read", conf);
        if (conf_resp.data) {
            setConf(conf_resp.data)
        }
    } catch (error) {
        throw new Error('Failed to read config: ', error)        
    }
  }


