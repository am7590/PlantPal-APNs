pub mod plant;
mod push;

mod plant_proto {
        include!("plant.rs");

        pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
                tonic::include_file_descriptor_set!("plant_descriptor");
}

use std::{error::Error, time::{SystemTime, UNIX_EPOCH}, ptr::null};
use tonic::transport::Server;
use dotenv::dotenv;
use plant::plant_service_client::PlantServiceClient;
use tonic::transport::Channel;
use tokio::time::{sleep, Duration};

// Connect to psql: heroku pg:psql -a plant-app-postgres

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let channel = Channel::from_static("https://rust-grpc-tutorial-git-plantpal.apps.okd4.csh.rit.edu:9001")
        .connect()
        .await?;

    let mut client = PlantServiceClient::new(channel);


    loop {
        // Get plants that need to be watered
        let response = client.get_watered(()).await?;
        let plants = &response.get_ref().plants;
        println!("Number of notifications to send: {}", plants.len()); 

        for plant in plants {
            match &plant.information {
                Some(information) => {
                    let name = &information.name();
                    let device = &plant.identifier.as_ref().unwrap().device_identifier;
                    let sku = &plant.identifier.as_ref().unwrap().sku;

                    let now = SystemTime::now();
                    let two_seconds = Duration::new(60*5, 0);
                    let new_time = now + two_seconds;

                    // Convert to int64 (Unix timestamp)
                    let unix_timestamp = match new_time.duration_since(UNIX_EPOCH) {
                        Ok(duration) => duration.as_secs() as i64,
                        Err(_) => -1, // Handle errors here if necessary
                    };

                    println!("unix epoch: {}", unix_timestamp); 

                    // Create the PlantUpdateRequest
                    let request = tonic::Request::new(plant::PlantUpdateRequest {
                        identifier: Some(plant::PlantIdentifier {
                            sku: sku.clone(),
                            device_identifier: device.clone(),
                        }),
                        information: Some(plant::PlantInformation {
                            last_watered: Some(unix_timestamp),
                            last_health_check: information.last_health_check,
                            last_identification: information.last_identification,
                            name: information.name.clone(), // Keep the user's current name
                        }),
                    });

                    // Send push notification for the plant that needs to be watered
                    match send_push_notification(&name, &device).await {
                        Ok(_) => {
                            let response = client.update_plant(request).await;
                            match response {
                                Ok(_) => println!("Successfully sent push"),
                                Err(e) => println!("Error sending push notification: {}", e),
                            }
                        },
                        Err(e) => println!("Error sending push notification: {}", e),
                    }
                },
                None => println!("Plant has no identifier"),
            }
        }

        println!("Sleeping for 1 minute before making the next call...");
        sleep(Duration::from_secs(60)).await;
    }
}

         
async fn send_push_notification(name: &str, device: &str) -> Result<(), Box<dyn Error>> {
    // TODO: Include 'name' in the data payload for deep linking
    let payload = format!("navStack://{}", name);
    let _ = push::apns::run(&name, &device).await;
    Ok(())
}