import { useAuth } from '../AuthProvider';
import SearchHistory from '../components/SearchHistory';


const history = () => {
    const { history } = useAuth();

    // TO DO: map history object (should be id: [title, date])
    // to individual SearchHistory components.

    return (
        <div>
            <p> Welcome to the history page. </p>
        </div>
    )
}


export default history;
