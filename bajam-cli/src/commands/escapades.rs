use std::collections::HashMap;

use reqwest::{StatusCode, header};

use crate::errors::CliError;
use crate::commands::load_session_config;
use bajam_common::models::escapade::Escapade;


pub async fn create_escapade(escapade_name: String) -> Result<(), CliError> {
    let config = load_session_config()?;

    if let Some(user) = config.connected_user {
      let client = reqwest::Client::new();

      let mut body_json = HashMap::new();
      body_json.insert("name", escapade_name);

      let response = client
          .post(format!("http://{}:{}/escapades", config.server_info.adress, config.server_info.port))
          .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
          .json(&body_json)
          .send()
          .await?;

      match response.status() {
        StatusCode::OK => println!("Escapade successfuly created."),
        status => {
            let body_text = response.text().await?;
            return Err(CliError::HttpError(status, body_text));
        }
      }
      
    } else {
      return Err(CliError::UncompleteConfig);
    }

    Ok(())
}


pub async fn list_escapades() -> Result<(), CliError> {
    let config = load_session_config()?;

    if let Some(user) = config.connected_user {
      let client = reqwest::Client::new();

      let response = client
          .get(format!("http://{}:{}/escapades", config.server_info.adress, config.server_info.port))
          .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
          .send()
          .await?;

      dbg!(&response);

      match response.status() {
        StatusCode::NOT_FOUND => println!("No escapade found for {} in {}:{}.", user.username, config.server_info.adress, config.server_info.port),
        StatusCode::OK => {
          let list: Vec<Escapade> = response.json().await?;
          println!("Available escapades:");
          for esc in list {
            println!("{}", esc.name);
          }
        },
        _ => eprintln!("Error in decoding response, {}", response.status()),
      }
    } else {
      return Err(CliError::UncompleteConfig);
    }

    Ok(())
}

