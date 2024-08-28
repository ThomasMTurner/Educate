import { useContext, createContext, useState, useEffect } from "react";
import { useNavigate } from 'react-router-dom';
import { readConfig } from './config_utilities';
import axios from 'axios';

const AuthContext = createContext();
const SESSION_STORAGE_KEY = "app_auth_state";

const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [history, setHistory] = useState({});
  const [config, setConfig] = useState({});
  const navigate = useNavigate();

  const [config, setConfig] = useState(() => {
    const saved = sessionStorage.getItem(SESSION_STORAGE_KEY);
    return saved ? JSON.parse(saved): {
        user: {username: '', password: '', history: []},
        redis_connection_str: '',
        search_params: {
            crawl_depth: 1,
            number_of_seeds: 30,
            search_method: 0,
            browsers: {Google: false, DuckDuckGo: true},
            index_type: 0,
            q: ''
        }
    };
  });

  useEffect(() => {
    sessionStorage.setItem(SESSION_STORAGE_KEY, JSON.stringify(config));
    console.log("Config updated: ", config);
  }, [config])
  
  const loginAction = async (data) => {
  try {
    const response = await axios.post("http://localhost:9797/auth/login", data);
    console.log('Obtained login response', response.data);

    if (response.data) {
      setUser(response.data.username);
      setHistory(response.data.search_histories);
      navigate("/")
      try {
          const conf_data = {
             user: {
                username: response.data.username != null ? response.data.username : '',
                password: '',
                history: response.data.search_histories
             },
             redis_connection_str: '',
             search_params: {
                crawl_depth: 1,
                number_of_seeds: 32,
                search_method: 0,
                browsers: {
                    'ddg': true,
                    'google': false
                },
                index_type: 0,
                q: ''
             }
          }

         await readConfig(conf_data, setConfig)
    
      } catch (error) {
            throw new Error("No data received for configuration read");
        }
    } 
    } catch (error) {
    console.error("Error during login:", error);

    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      if (error.response.status === 500) {
        console.error("500 Internal Server Error");
        // Handle 500 error specifically (e.g., show a user-friendly message)
      } else {
        console.error(`Server responded with status: ${error.response.status}`);
      }
      console.error("Response data:", error.response.data);
    } else if (error.request) {
      // The request was made but no response was received
      console.error("No response received from server");
    } else {
      // Something happened in setting up the request that triggered an Error
      console.error("Error setting up the request:", error.message);
    }
    }
    };


  const registerAction = async (data) => {
    try {
    const response = await axios.post("http://localhost:9797/auth/register", data);

    if (response.data) {
      loginAction(data)
      window.location.href = "/";
    } else {
      throw new Error("No data received from server");
    }
    } catch (error) {
    console.error("Error during login:", error);

    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      if (error.response.status === 500) {
        console.error("500 Internal Server Error");
        // Handle 500 error specifically (e.g., show a user-friendly message)
      } else {
        console.error(`Server responded with status: ${error.response.status}`);
      }
      console.error("Response data:", error.response.data);
    } else if (error.request) {
      // The request was made but no response was received
      console.error("No response received from server");
    } else {
      // Something happened in setting up the request that triggered an Error
      console.error("Error setting up the request:", error.message);
    }
    } 
  }

  const logOut = () => {
    setUser(null);
    setConfig({});
    setHistory({});
    window.location.href="/login";
    // TO DO: Clear session storage.
  };

  return (
    <AuthContext.Provider value={{ user, loginAction, logOut, registerAction, history, config, setConfig}}>
      {children}
    </AuthContext.Provider>
  );
};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
