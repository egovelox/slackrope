# slackrope

A rust-utility to get the hotlist from weechat via weechat-relay.

## Todos

Document content of

### Configuration



First read [https://rust-cli-recommendations.sunshowers.io/configuration.html](https://rust-cli-recommendations.sunshowers.io/configuration.html).

Fields to set :
slackrope_provider : weechat | weechat-headless
slackrope_port : the relay port ?
slackrope_slack_client_id : ?


## Caveats
- `clear` command will effectively set all messages as read in `weechat`.  
In fact, it will set all messages as read in `slack`, except for messages in threads.
Meaning that, if you shut down and restart `weechat`,  
your hotlist may not be cleared for these messages in threads. 
As it restarts, `weechat` will synchronize with `slack` 
and those unread `slack` thread messages will appear again in `weechat` hotlist.


