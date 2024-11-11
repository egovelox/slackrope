pub use crate::environment::get_slack_register_url;
pub use crate::models::SlackTeam;
use crate::utils::sleep;
pub use crate::utils::{clean_string, match_string};
pub use crate::weechat_connection::init_connection;
pub use crate::weechat_process::{is_weechat_running, spawn_weechat_process};
use anyhow::Result;
use log::{debug, info};
use std::process::exit;
use sysinfo::{System, SystemExt};
use weechat_relay_rs::commands::{
    Command, InfolistCommand, InputCommand, PointerOrName, StrArgument,
};
use weechat_relay_rs::messages::{InfolistItem, Object, WInfolist};
use weechat_relay_rs::Connection;

pub fn print_register_url() -> Result<()> {
    let slack_register_url = get_slack_register_url();
    println!("To register a new slack workspace, first you need a workspace token. Please follow this link:");
    println!();
    println!("  {slack_register_url}");
    println!();
    println!("Then you need to register your token. You can use the following command: ");
    println!();
    println!("  register --token <TOKEN>");
    Ok(())
}

pub fn register_slack_token(sys: &mut System, token: &String) -> Result<()> {
    if !is_weechat_running(sys) {
        spawn_weechat_process()?;
        sleep(2);
    }

    let mut connection = init_connection()?;
    debug!("connection initiated");

    println!("Checking that wee-slack python plugin is loaded...");
    debug!("connection initiated");
    match check_connection_and_python_wee_slack_plugin(&mut connection)? {
        (true, true) => {
            debug!("wee-slack python plugin checked");
        }
        (true, false) => {
            debug!("wee-slack python plugin is not loaded");
            println!("Error : wee-slack python plugin is not loaded in weechat.");
            println!("Did you correctly install wee_slack.py python script ?");
            exit(1);
        }
        _ => {
            debug!("A failure occured while connecting to weechat");
            println!("Error : could not check if wee-slack python plugin is loaded in weechat.");
            println!("Did you correctly configure the weechat-relay connection ?");
            exit(1);
        }
    };

    println!("Registering slack token...");
    handle_register_request(&mut connection, token)?;
    sleep(2);

    println!("Reloading weechat...");
    send_quit_command(&mut connection)?;
    while is_weechat_running(sys) {
        sys.refresh_processes();
        debug!("waiting for weechat to quit...");
        sleep(1);
    }

    // restart
    sleep(2);
    spawn_weechat_process()?;
    sleep(2);

    let mut connection = init_connection()?;
    debug!("connection initiated");
    send_infolist_buffer_request(&mut connection)?;
    let infolist = get_infolist_buffer_response(&mut connection)?;
    debug!("successfully got infolist response");

    let teams = build_slack_registered_teams(&infolist)?;

    if teams.len() != 0 {
        println!();
        println!(
            "You got currently {} registered slack team(s) aka workspace(s) :",
            teams.len()
        );
        for team in teams {
            println!("  - {}", team.name)
        }
        println!();
        println!("Note that, in case you don't see the slack team (aka workspace) you just tried to register :");
        println!();
        print_anomaly_advice();
    } else {
        println!();
        println!("Weechat currently couldn't find any registered slack team (aka workspace) :");
        println!();
        print_anomaly_advice();
    }

    Ok(())
}

pub fn list_registered_slack_teams(sys: &mut System) -> Result<()> {
    if !is_weechat_running(sys) {
        spawn_weechat_process()?;
        sleep(2);
    }

    let mut connection = init_connection()?;
    debug!("connection initiated");

    send_infolist_buffer_request(&mut connection)?;
    let infolist = get_infolist_buffer_response(&mut connection)?;
    debug!("successfully got infolist response");

    let teams = build_slack_registered_teams(&infolist)?;
    if teams.len() != 0 {
        println!();
        println!(
            "You got currently {} registered slack team(s) aka workspace(s) :",
            teams.len()
        );
        for team in teams {
            println!("  - {}", team.name)
        }
    } else {
        println!();
        println!("Weechat currently couldn't find any registered slack team (aka workspace) :");
    }
    Ok(())
}

fn print_anomaly_advice() {
    println!("  - Either your newly registered slack team isn't loaded in Weechat yet");
    println!("      > You may check again later, using the 'list-slack-teams' command.");
    println!();
    println!("  - Or your token was invalid");
    println!("      > You may retry the whole process, ");
    println!(
        "      > ensuring that you use a valid token in the 'register --token <TOKEN>' command."
    );
}

fn handle_register_request(connection: &mut Connection, token: &String) -> Result<()> {
    connection.send_command(&build_input_command(
        "core.weechat",
        format!("/slack register {}", token).as_str(),
    ))?;

    debug!("successfully sent register request");

    Ok(())
}

fn send_quit_command(connection: &mut Connection) -> Result<()> {
    connection.send_command(&build_input_command("core.weechat", "/quit"))?;

    debug!("successfully sent /quit command");

    Ok(())
}

fn build_input_command(buffer_name: &str, command: &str) -> Command<InputCommand> {
    let input_command = InputCommand::new(
        PointerOrName::Name(StrArgument::new(buffer_name).unwrap().to_stringargument()),
        StrArgument::new(command).unwrap().to_stringargument(),
    );
    Command {
        id: None,
        command: input_command,
    }
}

fn send_infolist_buffer_request(connection: &mut Connection) -> Result<()> {
    let info_command = InfolistCommand::new(
        StrArgument::new("buffer").unwrap().to_stringargument(),
        None,
        vec![StrArgument::new("python.slack.*")
            .unwrap()
            .to_stringargument()],
    );
    let info_command: Command<InfolistCommand> = Command {
        id: None,
        command: info_command,
    };
    connection.send_command(&info_command)?;
    Ok(())
}

fn get_infolist_buffer_response(connection: &mut Connection) -> Result<Option<WInfolist>> {
    let message = connection.get_message();
    match message {
        Ok(m) => {
            if m.objects.len() > 0 {
                match &m.objects[0] {
                    Object::Inl(infolist) => Ok(Some(infolist.clone())),
                    _ => {
                        info!("Could not parse the hotlist response");
                        Ok(None)
                    }
                }
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            info!("Could not receive the hotlist response");
            Ok(None)
        }
    }
}

fn build_slack_registered_teams(infolist: &Option<WInfolist>) -> Result<Vec<SlackTeam>> {
    match infolist {
        Some(infolist) => {
            let mut teams: Vec<SlackTeam> = vec![];
            for item in infolist.items.iter() {
                for variable in item.variables.iter() {
                    if match_string(&variable.name, "localvar_value_00008") {
                        if let Some(buffer_type) = match &variable.value {
                            Object::Str(s) => Some(clean_string(&s)),
                            _ => None,
                        } {
                            match buffer_type.as_str() {
                                "team" => count_slack_team(&item, &mut teams),
                                _ => (),
                            }
                        }
                    }
                }
            }
            Ok(teams)
        }
        None => Ok(vec![]),
    }
}

fn count_slack_team(item: &InfolistItem, teams: &mut Vec<SlackTeam>) {
    for variable in item.variables.iter() {
        if match_string(&variable.name, "name") {
            if let Some(buffer_name) = match &variable.value {
                Object::Str(buffer_name) => Some(clean_string(&buffer_name)),
                _ => None,
            } {
                teams.push(SlackTeam { name: buffer_name })
            }
        }
    }
}

fn send_infolist_python_script_request(connection: &mut Connection) -> Result<()> {
    let info_command = InfolistCommand::new(
        StrArgument::new("python_script")
            .unwrap()
            .to_stringargument(),
        None,
        vec![],
    );
    let info_command: Command<InfolistCommand> = Command {
        id: None,
        command: info_command,
    };
    connection.send_command(&info_command)?;
    Ok(())
}

fn get_infolist_python_script_response(connection: &mut Connection) -> Result<Option<WInfolist>> {
    let message = connection.get_message();
    match message {
        Ok(m) => {
            if m.objects.len() > 0 {
                match &m.objects[0] {
                    Object::Inl(infolist) => Ok(Some(infolist.clone())),
                    _ => {
                        info!("Could not parse the infolist response");
                        Ok(None)
                    }
                }
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            debug!("{:?}", e);
            info!("Could not receive the infolist response");
            Ok(None)
        }
    }
}

pub fn check_connection_and_python_wee_slack_plugin(
    connection: &mut Connection,
) -> Result<(bool, bool)> {
    send_infolist_python_script_request(connection)?;
    let infolist = get_infolist_python_script_response(connection)?;

    let mut is_wee_slack_connection_ok = true;
    let mut is_python_wee_slack_installed = false;
    match infolist {
        Some(infolist) => {
            for item in infolist.items.iter() {
                for variable in item.variables.iter() {
                    if match_string(&variable.name, "name") {
                        if let Some(name) = match &variable.value {
                            Object::Str(s) => Some(clean_string(&s)),
                            _ => None,
                        } {
                            match name.as_str() {
                                "slack" => is_python_wee_slack_installed = true,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        None => is_wee_slack_connection_ok = false,
    }

    Ok((is_wee_slack_connection_ok, is_python_wee_slack_installed))
}
