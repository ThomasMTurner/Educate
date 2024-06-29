import { useState } from "react";
import { useAuth } from "../AuthProvider";


export const Register = () => {
    return (
        <div>
            Currently empty component.
        </div>
    )
}

const Login = () => {
    const [input, setInput] = useState({ username: "", password: "", history: []})
    const [valid, setValid] = useState({ "username": true, "password": true })
    const auth = useAuth();
    
    const handleLoginEvent = (e) => {
        e.preventDefault();
        if (input.username !== "" && input.password !== "" && valid.username && valid.password) {
            auth.loginAction(input);
            return;
        }
        alert("Please enter valid details");
    }

    const handleRegisterEvent = (e) => {
        e.preventDefault();
        if (input.username !== "" && input.password !== "" && valid.username && valid.password) {
            auth.registerAction(input);
            return;
        }
        alert("Please enter valid details");
    }

    const handleInput = (e) => {
        const { name, value } = e.target;
        setInput((prev) => ({
            ...prev,
            [name]: value,
            }));
        setValid((prev) => ({...prev, 
            username: input.username.length > 5 && input.username.length < 21,
            password: input.password.length > 7 && input.password.length < 31 }))
    };

    return (
        <form style={{display: 'flex', flexDirection: 'column', gap: '1rem'}}>
            <div className="form_control">
                <label style={{marginRight: "1rem"}} htmlFor="username">Username</label>
                <input
                type="username"
                id="username"
                name="username"
                aria-describedby="username"
                aria-invalid={!valid.username}
                onChange={handleInput}
                minLength="6"
                maxLength="20"
                />
                {!valid.username &&
                <div style={{color: "red"}} id="user-email" className="sr-only">
                Please enter a valid username. It must contain at least 6 characters.
                </div>
                }
            </div>
            <div className="form_control">
                <label style={{marginRight: "1rem"}} htmlFor="password">Password</label>
                <input
                type="password"
                id="password"
                name="password"
                aria-describedby="user-password"
                aria-invalid={!valid.password}
                minLength="8"
                maxLength="30"
                onChange={handleInput}
                />
                {!valid.password && 
                <div style={{color: "red"}} id="user-password" className="sr-only">
                Your password should be more than 6 characters.
                </div>
                }
            </div>
            <button type="button" style={{marginTop: "1rem"}} onClick={handleLoginEvent} className="btn-submit"> Login </button>
            <button type="button" style={{marginTOp: "1rem"}} onClick={handleRegisterEvent} className="btn-submit"> Register </button>
            </form>
        )
}

export default Login;

