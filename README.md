# 🪢 slackrope 🪢

A simple [Rust](https://www.rust-lang.org/) CLI to get a [slack](https://slack.com/) hotlist.

## What is a slack hotlist ?

A `hotlist` primarily is a feature available in [weechat](https://weechat.org/).  

It's a set of counters allowing to know at first glance the number 
of unread messages in one or many [IRC](https://en.wikipedia.org/wiki/IRC) channels.

Once you setup this amazing [wee-slack](https://github.com/wee-slack/wee-slack) plugin, 
slack workspaces, channels and threads can all be integrated in `weechat`.  

So the `hotlist` ( weechat feature ) now is turned into a `slack hotlist`.

The `slackrope` CLI does a simple job : it connects to a running instance of `weechat`  
( or `weechat-headless` which runs as a daemon process )  
and brings these hotlist counters back in your terminal.

<img width="764" alt="slackrope_architecture" src="https://github.com/user-attachments/assets/5f10f791-93dc-4f99-ba07-f735717b300f">

## Example

Let's imagine you are connected with 2 `slack` workspaces `A` and `B`, where you currently have :

- 1 unread message highlighting you with `@`
- 0 unread private messages
- 2 unread messages in slack threads

```console
$ slackrope hotlist
1 0 2
```

```console
$ slackrope hotlist -f detailed
{
  "priority_1": {
    "count": 1,
    "items": [
       "workspaceA.channelX",
    ]
  },
  "priority_2": {
    "count": 0,
    "items": []
  },
  "priority_3": {
    "count": 2,
    "items": [
       "workspaceA.channelX.thread#1",
       "workspaceB.channelY.thread#2"
    ]
  }
}
```

```console
$ slackrope list-teams

You have currently 2 registered slack team(s) a.k.a workspace(s) :
  - slack.workspaceA
  - slack.workspaceB

```

## Installation and Configuration

As explained above, the `slackrope` CLI connects to `weechat`
and brings back the hotlist in various formats.

Prerequisites:

- MacOS or Linux
- [weechat](https://weechat.org/) > 2.2
- [wee-slack](https://github.com/wee-slack/wee-slack)
- [slack](https://slack.com/)

You can use the `slackrope health` command to display various indicators and settings.

You can use the `slackrope register` command to add a new slack workspace  
( then follow the procedure to get your token and finally run `slackrope register --token **********` )

You may want to configure the CLI (e.g customize port or password) 
by creating a `slackrope.toml` configuration file :
- this file must be located in the `$XDG_CONFIG_HOME/slackrope/` directory, 
provided that the `$XDG_CONFIG_HOME` variable is set in your environment.
- if not, this file must be located in the `$HOME/.config/slackrope/` directory.

Here I listed the `slackrope` default keys and values :
```toml
# slackrope.toml
sr_weechat_host = "127.0.0.1"
sr_weechat_relay_port = "8000"
sr_weechat_program_name = "weechat-headless"
sr_weechat_password = ""
sr_slack_register_baseurl = "https://slack.com/oauth/authorize"
sr_slack_register_weeslack_client_id = "2468770254.51917335286"
sr_slack_register_scope = "client"
sr_slack_register_redirect_uri = "https%3A%2F%2Fwee-slack.github.io%2Fwee-slack%2Foauth"
sr_wee_slack_plugin_directory = "/Users/egovelox/.local/share/weechat/python"
sr_wee_slack_plugin_filename = "wee_slack.py"
```

## Notes

> `slackrope clear` will set all messages as read in `slack`, except for messages located in threads.  

Meaning that, if you run `slackrope kill` and then restart, 
`weechat` will synchronize with `slack`
and those unread `slack` threads messages will appear again in `slackrope hotlist`.

## Releases

> To do

## Acknowledgement

[weechat-relay-rs](https://github.com/jtracey/weechat-relay-rs) offers the rust bindings 
to connect to weechat via the `relay` protocol.

[wee-slack](https://github.com/wee-slack/wee-slack) is a plugin 
allowing `weechat` or `weechat-headless` to be connected with `slack`.
