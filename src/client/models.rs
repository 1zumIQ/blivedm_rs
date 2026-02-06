// src/client/models.rs
//! Data models for Bilibili live danmaku WebSocket client

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct DanmuServer {
    pub host: String,
    pub port: i32,
    pub wss_port: i32,
    pub ws_port: i32,
}

impl Default for DanmuServer {
    fn default() -> Self {
        Self {
            host: String::from("broadcastlv.chat.bilibili.com"),
            port: 2243,
            wss_port: 443,
            ws_port: 2244,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MsgHead {
    pub pack_len: u32,
    pub raw_header_size: u16,
    pub ver: u16,
    pub operation: u32,
    pub seq_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMessage {
    pub uid: u64,
    pub roomid: u64,
    pub protover: i32,
    pub platform: String,
    pub type_: i32,
    pub key: String,
}

impl AuthMessage {
    pub fn from(map: &HashMap<String, String>) -> AuthMessage {
        AuthMessage {
            uid: map.get("uid").unwrap().parse::<u64>().unwrap(),
            roomid: map.get("room_id").unwrap().parse::<u64>().unwrap(),
            protover: 3,
            platform: "web".to_string(),
            type_: 2,
            key: map.get("token").unwrap().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BiliMessage {
    Danmu {
        user: DanmuUser,
        text: String,
    },
    Gift {
        user: String,
        gift: GiftData,
    },
    /// Online rank count message (ONLINE_RANK_COUNT)
    OnlineRankCount {
        /// Number of high-energy users in the live room
        count: u64,
        /// Number of online users in the live room
        online_count: u64,
    },
    // Add more variants as needed
    Raw(serde_json::Value),
    #[deprecated(note = "Use Raw variant instead")]
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CoinType {
    #[default]
    Silver,
    Gold,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct GiftData {
    #[serde(rename = "giftName")]
    pub gift_name: String,
    pub uname: String,
    pub uid: u64,
    pub num: i64,
    pub price: i64,
    pub coin_type: CoinType,
    pub medal_info: Option<Medal>,
    pub medal: Option<Medal>,
}

impl Display for GiftData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}个{}", self.num, self.gift_name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Medal {
    #[serde(alias = "medal_name")]
    pub name: String,
    #[serde(alias = "medal_level")]
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DanmuUser {
    pub uid: u64,
    pub base: UserBase,
    pub medal: Option<Medal>,
}

impl DanmuUser {
    pub fn new(name: &str) -> Self {
        DanmuUser {
            uid: 0,
            base: UserBase {
                name: name.to_string(),
            },
            medal: None,
        }
    }
}

impl Display for DanmuUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UserBase {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_message_from_map() {
        let mut map = std::collections::HashMap::new();
        map.insert("uid".to_string(), "12345".to_string());
        map.insert("room_id".to_string(), "67890".to_string());
        map.insert("token".to_string(), "test_token".to_string());
        let auth = AuthMessage::from(&map);
        assert_eq!(auth.uid, 12345);
        assert_eq!(auth.roomid, 67890);
        assert_eq!(auth.key, "test_token");
    }
}
