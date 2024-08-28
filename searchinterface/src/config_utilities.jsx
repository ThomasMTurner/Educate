import axios from 'axios';


export const writeConfig = async (config) => {
    try {
        // const response =
        await axios.post("http://localhost:9797/config/write", config);
        // Previously had redundant call to response.data, my write request currently
        // does not have a return body.
    } catch (error) {
        console.error(error);
        throw new Error('Failed to write new config: ', error);
    }
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


