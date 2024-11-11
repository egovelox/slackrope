use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/*
* Hotlist priorities: 1, 2, 3
* @see https://weechat.org/files/doc/stable/weechat_user.en.html
* 1 = thread message 
*   - not triggered for private threads messages
* 2 = private 
*   - triggered only once when somebody sends one (or more) message to you privately
*   - note that it's never triggered for messages in private threads
* 3 = highlight 󰁥
*   - triggered when somebody sends one (or more) message to you on a public channel
*   - triggered when somebody uses the slack @ prefix : either `@you` or `@channel` or `@here`.
*   - note that customized slack's @ (e.g `@my-team`) are not supported.
*   - note that it's triggered also if somebody uses @you in a private context.
*/
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
