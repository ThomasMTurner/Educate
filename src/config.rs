extern crate redis;
use redis::{Commands, RedisError, RedisResult};
use crate::auth::Credentials;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::collections::HashMap;
use serde_json::Value;

// TO DO:
// Create configuration structure.
// Implement Redis cache writing & reading of the config structure.
// Export & implement API call for updating & reading the config with the below utilities.

/*
Reference guide:
indexType: 0 -> Document-Term, 1 -> Inverted, 2 -> B-Tree
searchMethod: 0 -> Document Clustering, 1 -> PageRank
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchParams {
    pub crawl_depth: u8,
    pub number_of_seeds: u8,
    search_method: u8,
    pub browsers: HashMap<String, bool>,
    index_type: u8,
    pub q: String,
    //location: String
}


#[derive(Serialize, Deserialize,  Debug)]
pub struct Config {
    user: Credentials,
    redis_connection_str: String,
    pub search_params: SearchParams,
}


impl Config {
    // TO DO: Modify to set default values.
    // Will need to manually enter the user & redis connection string,
    // default constructor necessary for search_params.
    pub fn new(user: Credentials, mut redis_connection_str: String, search_params: SearchParams) -> Self {
        // Set default Redis connection string if not provided.
        if redis_connection_str.is_empty() {
            redis_connection_str = String::from("redis://127.0.0.1:6379");
        }
        Config {user, redis_connection_str, search_params} 
    }
        
    pub fn write(&mut self) -> RedisResult<()> {
        let config_data;

        match serde_json::to_string(&json!({
            "searchParams": self.search_params,
        })) {
            Ok(value) => config_data = value,
            Err(_) => return Err(RedisError::from((redis::ErrorKind::TypeError, 
                "Could not write Redis configuration data - serialisation error")))
        }

        if self.redis_connection_str.is_empty() {
            println!("No Redis connection string provided. Using default.");
            self.redis_connection_str = String::from("redis://127.0.0.1:6379");
        }
        
        let client = redis::Client::open(self.redis_connection_str.as_str())?;
        let mut conn = client.get_connection()?;
        
        let key = format!("{}_config", self.user.username);
        conn.set(key, config_data)?;

        Ok(())
    }
    
    // TO DO: need to handle edge case where config does not yet exist in the Redis cache.
    pub fn read(&mut self) -> RedisResult<()> {
        if self.redis_connection_str.is_empty() {
            println!("No Redis connection string provided. Using default.");
            self.redis_connection_str = String::from("redis://127.0.0.1:6379")
        }

        let client = redis::Client::open(self.redis_connection_str.as_str())?;
        let mut conn = client.get_connection()?;

        let key = format!("{}_config", self.user.username);

        let mut result = String::new();
        
        if conn.exists(&key)? {
            println!("Key {} exists", key);
            result = conn.get(&key)?;
        }
        
        else {
            println!("Key {} does not exist", key);
            return self.write();
        }

        println!("Obtained result: {}", result);

        let value: Value = serde_json::from_str(&result).map_err(|_| 
            RedisError::from((redis::ErrorKind::TypeError, 
                "Could not obtain Redis configuration data - deserialisation error")))?;

        let search_params = serde_json::from_value(value.get("searchParams")
            .ok_or_else(|| RedisError::from((redis::ErrorKind::TypeError, "Missing searchParams")))?
            .clone()) 
            .map_err(|_| RedisError::from((redis::ErrorKind::TypeError, "Could not deserialize searchParams")))?;
        
        self.search_params = search_params;

        println!("Obtained search parameters: {:?}", self.search_params);

        Ok(())

    }
}


