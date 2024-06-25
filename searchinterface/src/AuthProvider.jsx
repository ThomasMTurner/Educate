import { useContext, createContext, useState } from "react";
import axios from 'axios';

const AuthContext = createContext();

const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [history, setHistory] = useState({});
  const [token, setToken] = useState(localStorage.getItem("site") || "");

  const loginAction = async (data) => {
  try {
    const response = await axios.post("http://localhost:9797/auth/login", data);

    if (response.data) {
      setUser(response.data.username);
      setHistory(response.data.history);
      setToken(response.data.token);
      localStorage.setItem("site", response.data.token);
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
    };


  const registerAction = async (data) => {
    try {
        await axios.post("http://localhost:9797/auth/register", data)
            .then((response) => {
                console.log(response)
            })
            .catch((error) => {
                console.error(error)
            })

    } catch (error) {
        console.error(error)
    }

  }

  const logOut = () => {
    setUser(null);
    setToken("");
    localStorage.removeItem("site");
    window.location.href="/login";
  };

  return (
    <AuthContext.Provider value={{ token, user, loginAction, logOut }}>
      {children}
    </AuthContext.Provider>
  );

};

export default AuthProvider;

export const useAuth = () => {
  return useContext(AuthContext);
};
