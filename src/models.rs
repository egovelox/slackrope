use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleHotlist {
    pub priority_1: i32,
    pub priority_2: i32,
    pub priority_3: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedHotlist {
    pub priority_1: Detailed,
    pub priority_2: Detailed,
    pub priority_3: Detailed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Detailed {
    pub count: i32,
    pub items: Vec<Buffer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buffer {
    pub buffer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackTeam {
    pub name: String,
}
