use bajam_common::models::escapade::Escapade;
use reqwest::{StatusCode, header};

use crate::EscInput;
use crate::{errors::CliError};
use crate::commands::load_session_config;
use bajam_common::models::heartbeat::{HeartbeatPost};

// Create a simple heartbeat without image
pub async fn create_heartbeat(escinput: EscInput, latitude: f64, longitude: f64, description: Option<String>) -> Result<(), CliError> {

    // Load config
    let config = load_session_config()?;

    // Check if config contains a user
    if let Some(user) = config.connected_user.as_ref() {

      // Start the client for the two requests coming
      let client = reqwest::Client::new();

      // Get escapade_id, either directly from user input or from name matching
      let escapade_id: i64 = match escinput {
        EscInput::Id(escapade_id) => escapade_id,
        EscInput::Name(escapade_name) => {
          let response = client
              .get(format!("http://{}:{}/escapades", config.server_info.address, config.server_info.port))
              .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
              .send()
              .await?;

          match response.status() {
            StatusCode::OK => {
              let list: Vec<Escapade> = response.json().await?;

              let mut matches = list
                  .iter()
                  .filter(|e| e.name == escapade_name);

              match (matches.next(), matches.next()) {
                  (Some(escapade), None) => escapade.id,
                  (None, _) => return Err(CliError::Missmatch("No match for this name".to_string())),
                  (Some(_), Some(_)) => return Err(CliError::Missmatch("At least two matches for this name, please provide escapade_id instead".to_string())),
              }
          }
            _ => return Err(
              CliError::HttpError(response.status(), "Error in fetching escapade list.".to_string())),
          }
        },
    };


      let heartbeat = HeartbeatPost {
        latitude,
        longitude,
        description, 
      };

      // To value to match HeartbeatPost struct !
      let heartbeat_json = serde_json::to_value(&heartbeat)?;

      // Post hearbeat
      let response = client
          .post(format!("http://{}:{}/api/escapades/{}/heartbeat", config.server_info.address, config.server_info.port, escapade_id))
          .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
          .json(&heartbeat_json)
          .send()
          .await?;

        dbg!(&response);

      match response.status() {
        StatusCode::OK => println!("Heartbeat successfuly created."),
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
