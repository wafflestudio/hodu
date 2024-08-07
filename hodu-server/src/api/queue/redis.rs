use redis::AsyncCommands;
use redis::RedisError;

pub enum RedisQueueError {
    ClientCreationError(RedisError),
    ConnectionError(RedisError),
    CommandError(String, RedisError),
}

impl RedisQueueError {
    pub fn messaging(&self) -> String {
        match self {
            RedisQueueError::ClientCreationError(err) => {
                format!("Failed to create Redis client: {}", err.to_string())
            }
            RedisQueueError::ConnectionError(err) => {
                format!("Failed to get Redis connection: {}", err.to_string())
            }
            RedisQueueError::CommandError(cmd, err) => {
                format!("Failed to run command {} : {}", cmd, err.to_string())
            }
        }
    }
}

//put request in redis queue(name : usercode)
//data protocol: (Id)*&@*@&(LANGUAGE)*&@*@&(Code Text)
//return Result<("Success"(string)), Error String>
//You need to match or unwrap result to retrieve data
pub async fn redisIn(id: u32, language: &str, code: &str) -> Result<String, String> {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
    };

    let queue_name = "usercode";
    let lang_code = format!(
        "{Id}*&@*@&{lang}*&@*@&{codetext}",
        Id = id,
        lang = language,
        codetext = code
    );

    let result = match con.rpush(queue_name, lang_code).await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::CommandError("RPUSH".to_string(), e).messaging()),
    };

    Ok("Success".to_string())
}

//take out request in redis queue(name : usercode)
//return Result< (tuple : ((Id : u32), (LANGUAGE : String), (Code Text : String))), Error String>
//You need to match or unwrap result to retrieve data
pub async fn redisOut() -> Result<(u32, String, String), String> {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
    };

    let queue_name = "usercode";

    let lang_code: String = match con.lpop(queue_name, None).await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::CommandError("LPOP".to_string(), e).messaging()),
    };

    let parts: Vec<&str> = lang_code.split("*&@*@&").collect();

    let code = parts[0].parse::<u32>().unwrap(); //RedisIn ensures it is an integer, so no error handling required

    Ok((code, parts[1].to_string(), parts[2].to_string()))
}

//return number of left request in redis
//return Result<(u32 : (number of left request)), Error STring>
//You need to match or unwrap result to retrieve data
pub async fn redisCount() -> Result<u32, String> {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
    };

    let queue_name = "usercode";

    let length: u32 = match con.llen(queue_name).await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::CommandError("LLEN".to_string(), e).messaging()),
    };

    Ok(length)
}

//return the existance of item by id
//return Result<bool(existance), Error String>
//You need to match or unwrap result to retrieve data
pub async fn redisExistbyId(id: u32) -> Result<bool, String> {
    let client = match redis::Client::open("redis://127.0.0.1/") {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
    };

    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
    };

    let queue_name = "usercode";

    let elements: Vec<String> = match con.lrange(queue_name, 0, -1).await {
        Ok(c) => c,
        Err(e) => return Err(RedisQueueError::CommandError("LRANGE".to_string(), e).messaging()),
    };

    let pattern: &str = &id.to_string();
    for element in elements.iter() {
        let parts: Vec<&str> = element.split("*&@*@&").collect();
        if parts[0] == pattern {
            return Ok(true);
        }
    }

    Ok(false)
}