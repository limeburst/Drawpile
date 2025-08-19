use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCommand {
    pub cmd: String,
    pub args: Vec<Value>,
    pub kwargs: serde_json::Map<String, Value>,
}

impl ServerCommand {
    pub fn from_payload(payload: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let json_str = std::str::from_utf8(payload);

        match json_str {
            Ok(json_str) => {
                let mut map: serde_json::Map<String, Value> = serde_json::from_str(json_str)?;

                // Extract cmd and args, remove them from the map
                let cmd = match map.remove("cmd") {
                    Some(Value::String(s)) => s,
                    Some(v) => {
                        return Err(format!("ServerCommand: cmd is not a string: {:?}", v).into());
                    }
                    None => return Err("ServerCommand: Missing 'cmd' field".into()),
                };

                let args = match map.remove("args") {
                    Some(Value::Array(arr)) => arr,
                    Some(Value::Null) | None => Vec::<Value>::new(),
                    Some(v) => {
                        return Err(format!("ServerCommand: args is not an array: {:?}", v).into());
                    }
                };

                // The rest of the map is kwargs
                let kwargs = map;

                Ok(ServerCommand { cmd, args, kwargs })
            }
            Err(e) => {
                println!("ServerCommand: Invalid JSON: {}", e);
                print!("ServerCommand: Payload (hex): ");
                for byte in payload {
                    print!("{:02x}", byte);
                }
                println!();
                return Err(format!("ServerCommand: Invalid JSON: {}", e).into());
            }
        }
    }
}
