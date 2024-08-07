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
//data protocol: json string by serde
//return Result<("Success"(string)), Error String>
//You need to match or unwrap result to retrieve data
pub async fn redisIn(data: CodeSubmission) -> Result<String, String> {
	let client = match redis::Client::open("redis://127.0.0.1/") {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
	};

	let mut con = match client.get_multiplexed_async_connection().await {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
	};

	let queue_name = "usercode";
	let submission_data = serde_json::to_string(&data).unwrap();

	let result = match con.rpush(queue_name, submission_data).await {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::CommandError("RPUSH".to_string(), e).messaging()),
	};

	Ok("Success".to_string())
}

//take out request in redis queue(name : usercode)
//return Result<CodeSubmission, Error String>
//You need to match or unwrap result to retrieve data
pub async fn redisOut() -> Result<CodeSubmission, String> {
	let client = match redis::Client::open("redis://127.0.0.1/") {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::ClientCreationError(e).messaging()),
	};

	let mut con = match client.get_multiplexed_async_connection().await {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::ConnectionError(e).messaging()),
	};

	let queue_name = "usercode";

	let data: String = match con.lpop(queue_name, None).await {
			Ok(c) => c,
			Err(e) => return Err(RedisQueueError::CommandError("LPOP".to_string(), e).messaging()),
	};

	Ok(serde_json::from_str(&data).unwrap())
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
pub async fn redisExistbyId(id: String) -> Result<bool, String> {
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

	for element in elements.iter() {
			let deserialized_data : CodeSubmission = serde_json::from_str(&element).unwrap();
			if deserialized_data.id == id {
					return Ok(true);
			}
	}

	Ok(false)
}