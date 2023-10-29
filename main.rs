pub mod plant;
mod push;

mod plant_proto {
        include!("plant.rs");

        pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
                tonic::include_file_descriptor_set!("plant_descriptor");
}

use std::error::Error;
use tonic::transport::Server;
use dotenv::dotenv;
use plant::plant_service_client::PlantServiceClient;
use tonic::transport::Channel;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let channel = Channel::from_static("http://127.0.0.1:9001")
        .connect()
        .await?;

    let mut client = PlantServiceClient::new(channel);


    loop {
        // Call the get_watered function
        let response = client.get_watered(()).await?;

        println!("Got watered");

        // Process the response and send push notifications
        let plants = &response.get_ref().plants;

        println!("Number of plants: {}", plants.len());  // Print out the number of plants

        for plant in plants {
            println!("plant");
            match &plant.information {
                Some(information) => {
                    let name = &information.name();
                    // Send push notification for the plant that needs to be watered
                    match send_push_notification(&name).await {
                        Ok(_) => (),
                        Err(e) => println!("Error sending push notification: {}", e),  // Print out any errors
                    }
                },
                None => println!("Plant has no identifier"),
            }
        }
        
        

        // Sleep for 1 minute before making the next call
        sleep(Duration::from_secs(60)).await;
    }
}


         
async fn send_push_notification(name: &str) -> Result<(), Box<dyn Error>> {
    let push_string = format!("Remember to water {}!", name);
    // Include 'name' in the data payload for deep linking
    let payload = format!("navStack://petunia"); //, name
    let _ = push::apns::run(&push_string).await;
    Ok(())
}