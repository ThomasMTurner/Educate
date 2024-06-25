extern crate redis;
use redis::{Commands, RedisResult, RedisError, FromRedisValue, ToRedisArgs};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchHistory {
    pub url: String,
    pub title: String,
    pub date: String
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


// May pass the entire String object - assuming string slices.
pub fn authenticate(cred: &Credentials) -> redis::RedisResult<()> {
    // Assuming the user has spawned & configured a Redis instance.
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    
    let result = get_credentials(&mut con, &cred.username)?;
    if &cred.password != result.password.as_str() {
        println!("Invalid password!");
        // Modify to return Error.
        return Ok(())
    }

    else {
        println!("Logged in!");
        println!("History: {:?}", result.history);
        println!("User: {:?}", result.username);
        println!("Password: {:?}", result.password);
        Ok(())
    }

}


// TO DO: needs implementing (CONSIDER PASSING THE CLIENT INSTANCE RATHER THAN CREATING NEW
// INSTANCES FOR EACH LOGIN & REGISTRATION REQUEST)
pub fn _make_registration(cred: &Credentials) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    save_credentials(&mut con, cred)?;
    println!("Saved credentials: {:?}", cred);
    Ok(())
}

