extern crate redis;
use redis::{Commands, RedisResult, RedisError, FromRedisValue, ToRedisArgs};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchHistory {
    pub url: String,
    pub title: String,
    pub date: String,
    pub query: String
}


#[derive(Deserialize, Debug, Serialize)]
pub struct SearchHistoryResponse {
    search_histories: Vec<SearchHistory>,
    username: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub history: Vec<SearchHistory>
}

impl Credentials {
    fn new(username: &str, password: &str, history: &Vec<SearchHistory>) -> Self {
        Credentials {
            username: username.to_string(),
            password: password.to_string(),
            history: history.to_vec()
        }
    }
    fn redis_key(&self) -> String {
        format!("user:{}", self.username)
    }
}

impl FromRedisValue for Credentials {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let hash: HashMap<String, String> = HashMap::from_redis_value(v)?;
        let username = hash.get("username").ok_or(RedisError::from((redis::ErrorKind::TypeError, "Missing username")))?;
        let password = hash.get("password").ok_or(RedisError::from((redis::ErrorKind::TypeError, "Missing password")))?;
        let serialized_history = hash.get("history").ok_or(RedisError::from((redis::ErrorKind::TypeError, "Missing history")))?;
        let history: Vec<SearchHistory> = serde_json::from_str(serialized_history)
            .map_err(|_| RedisError::from((redis::ErrorKind::TypeError, "Failed to deserialize history")))?;
        Ok(Credentials::new(username, password, &history))
    }
}


impl ToRedisArgs for Credentials {
    fn write_redis_args<W>(&self, out: &mut W) where W: ?Sized + redis::RedisWrite {
        let mut hash = HashMap::new();
        hash.insert("username", &self.username);
        hash.insert("password", &self.password);
        let serialized_history = json!(&self.history).to_string();
        hash.insert("history", &serialized_history);
        hash.write_redis_args(out);
    }
}

fn save_credentials(con: &mut redis::Connection, cred: &Credentials) -> RedisResult<()> {
    let history = json!(cred.history).to_string();

    con.hset_multiple(cred.redis_key(), &[
    ("username", cred.username.as_str()),
    ("password", cred.password.as_str()),
    ("history", &history)
    ])
}

fn get_credentials(con: &mut redis::Connection, username: &str) -> RedisResult<Credentials> {
    let key = format!("user:{}", username);
    con.hgetall(key)
}


// May pass the entire String object - assuming string slices (Result<(), RedisError>).
pub fn authenticate(cred: &Credentials) -> RedisResult<SearchHistoryResponse> {
    // Assuming the user has spawned & configured a Redis instance.
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    
    let result = get_credentials(&mut con, &cred.username)?;

    if &cred.password != result.password.as_str() {
        println!("Invalid password!");
        // Modify based on available redis Error Kinds.
        Err(RedisError::from((redis::ErrorKind::TypeError, "Invalid password.")))
    }

    else {
        println!("Logged in!");
        println!("Obtained search history: {:?}", result.history);
        Ok(SearchHistoryResponse {
            search_histories: result.history,
            username: result.username
        })
    }

}


// TO DO: needs implementing (CONSIDER PASSING THE CLIENT INSTANCE RATHER THAN CREATING NEW
// INSTANCES FOR EACH LOGIN & REGISTRATION REQUEST)
pub fn make_registration(cred: &Credentials) -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    save_credentials(&mut con, cred)?;
    Ok(())
}

pub fn update_history(cred: &Credentials) -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let curr: Option<String> = con.hget(&cred.redis_key(), "history")?;
    let mut curr_history: Value = match curr {
        Some(h) => serde_json::from_str(&h).unwrap_or(Value::Array(Vec::new())),
        None => Value::Array(Vec::new())
    };

    if let Value::Array(ref mut arr) = curr_history {
        arr.extend(cred.history.iter().map(|h| json!(h)));
    }

    con.hset(cred.redis_key(), "history", curr_history.to_string())?;
    Ok(())
}
