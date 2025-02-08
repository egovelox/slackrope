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

<img width="764" alt="slackrope_architecture" src="https://github.com/user-attachments/assets/0b6d5952-651e-4586-a6d3-d55c9b6b3f7a">

## Example

Let's imagine you are connected with 2 `slack` workspaces `A` and `B`, where you currently have :

- 2 unread messages in threads you've participated in
- 0 unread private messages
- 1 unread message highlighting you with `@`

```console
$ slackrope hotlist
2 0 1
```

```console
$ slackrope hotlist -f detailed
{
  "priority_1": {
    "count": 1,
    "items": [
       "workspaceA.channelX.thread#1",
       "workspaceB.channelY.thread#2"
    ]
  },
  "priority_2": {
    "count": 0,
    "items": []
  },
  "priority_3": {
    "count": 2,
    "items": [
       "workspaceA.channelX",
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

See also paragraph below for a quick installation example on macOS with `brew`.

As explained above, the `slackrope` CLI connects to `weechat`
and brings back the hotlist in various formats.

Prerequisites:

- MacOS or Linux
- [weechat](https://weechat.org/) > 2.2
- [wee-slack](https://github.com/wee-slack/wee-slack)
- [slack](https://slack.com/)

You then need to configure the CLI (mainly for port and password) 
by creating a `slackrope.toml` configuration file :
- this file must be located in the `$XDG_CONFIG_HOME/slackrope/` directory, 
provided that the `$XDG_CONFIG_HOME` variable is set in your environment.
- if not, this file must be located in the `$HOME/.config/slackrope/` directory.

Here I listed the `slackrope` keys and default values :
```toml
# $HOME/.config/slackrope/slackrope.toml
sr_weechat_host = "127.0.0.1"
sr_weechat_relay_port = "8000"
sr_weechat_program_name = "weechat-headless"
sr_weechat_password = ""
sr_slack_register_baseurl = "https://slack.com/oauth/authorize"
sr_slack_register_weeslack_client_id = "2468770254.51917335286"
sr_slack_register_scope = "client"
sr_slack_register_redirect_uri = "https%3A%2F%2Fwee-slack.github.io%2Fwee-slack%2Foauth"
sr_wee_slack_plugin_directory = "$HOME/.local/share/weechat/python"
sr_wee_slack_plugin_filename = "wee_slack.py"
```

| Keys | Default value | Description |
| ---         |     ---      |          --- |
| sr_weechat_password | `""` | the password needed to connect to weechat via weechat-relay. It is required you set it, unless you've configured weechat to allow for an empty password. |
| sr_weechat_host | `127.0.0.1` | the host weechat is running on |
| sr_weechat_relay_port | `8000` | the port weechat-relay is listening on |
| sr_weechat_program_name | `weechat-headless` | the weechat executable, `weechat` or `weechat-headless` |
| sr_wee_slack_plugin_directory | `$HOME/.local/share/weechat/python` | the wee-slack python plugins directory |
| sr_wee_slack_plugin_filename | `wee_slack.py` | the wee-slack plugin file |
| sr_slack_register_baseurl | `https://slack.com/oauth/authorize` | needed to register your slack workspace, see [wee-slack](https://github.com/wee-slack/wee-slack) repository |
| sr_slack_register_weeslack_client_id | `2468770254.51917335286` | needed to register your slack workspace, see [wee-slack](https://github.com/wee-slack/wee-slack) repository |
| sr_slack_register_scope | `client` | needed to register your slack workspace, see [wee-slack](https://github.com/wee-slack/wee-slack) repository |
| sr_slack_register_redirect_uri | `https%3A%2F%2Fwee-slack.github.io%2Fwee-slack%2Foauth` | needed to register your slack workspace, see [wee-slack](https://github.com/wee-slack/wee-slack) repository |

You can use this command, meant to help you monitoring various indicators and settings.
> `slackrope health`

You can use this command to add a new slack workspace. It will help you following the procedure to get your `slack` token.
> `slackrope register`

Once you get your token, run it again with the `token` param. 
This will setup for you the token in the corresponding `weechat` config file (see `python.slack.slack_api_token` in `$HOME/.config/weechat/plugins.conf`) :
> `slackrope register --token **********`


## Quick installation example on `macOS` with `brew`

Tested on MacOS Sequoia 15.3 :

```bash

# install weechat
brew install weechat

# install wee_slack.py ( weechat plugin )
curl -L https://github.com/wee-slack/wee-slack/raw/refs/heads/master/wee_slack.py > $HOME/.local/share/weechat/python/wee_slack.py

# autoloading wee_slack.py when weechat starts
ln -s $HOME/.local/share/weechat/python/wee_slack.py $HOME/.local/share/weechat/python/autoload/

# install this wee-slack required dependency ( installed on the brew python3, that weechat will use if you installed it with brew )
/opt/homebrew/bin/python3 -m pip install --break-system-packages websocket-client

# Option 1: install slackrope from crates.io
cargo install slackrope

# Option 2: clone this repo
git clone https://github.com/egovelox/slackrope.git
cd slackrope && cargo build --release
cp ./target/release/slackrope $HOME/bin/slackrope

# check slackrope health, but it should be ko the first time
slackrope health

# ensure that all weechat instances are killed before further configuration
slackrope kill

# network configuration (connection between slackrope and weechat)
echo -e "[port]\nweechat = 8000\n[network]\npassword = \"password\"" >> $HOME/.config/weechat/relay.conf
echo "sr_weechat_password = \"password\"" > $HOME/.config/slackrope/slackrope.toml

# register your first slack workspace
slackrope register

```

## Notes

> use `slackrope -h` or `slackrope COMMAND -h` to get help on the cli parameters

> use `slackrope -d -d COMMAND` to get a quick grasp on the cli background process

> use `slackrope clear` to reset your hotlist when you notice a desynchro with `slack`.
This command was meant to help, when your slackrope hotlist keeps indicating you unread messages, whereas you have in fact no unread messages in `slack`.
Hopefully this should not happen very often.

## My way to use it within `tmux`

I mostly use `slackrope hotlist -t` inside `tmux` status-bar, with a 5 seconds refresh.

```bash
# $HOME/.tmux.conf
%hidden DEFAULT="default"
%hidden MAGENTA='#a6077b'
%hidden GREEN='#32a87d'
%hidden ORANGE='#cf6f0e'
%hidden LIGHT_BLUE='#81a1c1'

%hidden HOTLIST='$HOME/bin/slackrope hotlist -t "\033[38;5;208m󰁥\e[0m{{priority_3}} \e[38;5;200m\e[0m{{priority_2}} \e[38;5;112m\e[0m{{priority_1}}"'

set -g status-position bottom
set -g status-interval 5
# Span status line on 2 lines
set -g status-format[0] ''
set -g status-format[1] ''
set -g status-right ''
set -g status-left ''

set -g status-right "#[fg=#{GREEN},bg=#{DEFAULT}] #S:#I "
set -g status-left "#[fg=#{LIGHT_BLUE},bg=#{DEFAULT}]#(#{HOTLIST} | sed -r 's/\\[38;5;208m/#[fg=#{ORANGE}]/g' | sed -r 's/\\[38;5;200m/#[fg=#{MAGENTA}]/g' | sed -r 's/\\[38;5;112m/#[fg=#{GREEN}]/g' | sed -r 's/\\[0m/#[fg=#{LIGHT_BLUE}]/g') #[#{DEFAULT}]"
set -gF status-format[0] '#{status-left}#[align=right]#{status-right}'

# ...
# Print both status lines
set -g status 2
```

<br>
<img width="970" alt="slackrope_tmux" src="https://github.com/user-attachments/assets/35db4786-57d0-49ed-9ad5-a4c34b7074e3" />

## Releases

> To do

## Acknowledgement

[weechat-relay-rs](https://github.com/jtracey/weechat-relay-rs) offers the rust bindings 
to connect to weechat via the `relay` protocol.

[wee-slack](https://github.com/wee-slack/wee-slack) is a plugin 
allowing `weechat` or `weechat-headless` to be connected with `slack`.
