pub mod plant;

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
        match &plant.identifier {
            Some(identifier) => {
                let sku = &identifier.sku;
                // Send push notification for the plant that needs to be watered
                match send_push_notification(&sku) {
                    Ok(_) => (),
                    Err(e) => println!("Error sending push notification: {}", e),  // Print out any errors
                }
            },
            None => println!("Plant has no identifier"),
        }
    }

    // Sleep for 1 minute before making the next call
    sleep(Duration::from_secs(30)).await;
}
}


fn send_push_notification(sku: &str) -> Result<(), Box<dyn Error>> {
// Code for sending push notification goes here
// Use the provided SKU and name to customize the notification
println!("push to {}!!", sku);
Ok(())
}
