
pub mod apns {
    use serde::Serialize;
    use std::fs::File;
    use a2::{NotificationOptions, DefaultNotificationBuilder, NotificationBuilder, Client, Endpoint,};

    #[derive(Serialize, Debug)]
    pub struct PayloadURL {
        url: String,
    }
    
    impl PayloadURL {
        pub fn new(url: String) -> Self {
            PayloadURL { url }
        }
    }

    pub async fn run(name: &str, device: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Pushing him...");
        
        let topic: Option<String> = Some("com.alek.SucculentAndPlantApp".to_owned());
    
        let options = NotificationOptions {
            apns_topic: topic.as_deref(),
            ..Default::default()
        };
    
        let builder = DefaultNotificationBuilder::new()
            .set_body(&name)
            .set_badge(420)
            .set_category("cat1")            
            .set_mutable_content()
            .set_sound("ping.flac");
            
        // 47c3d1239a3242d1a7768ae81daa9cde5c133d9b13d13e5b30520c7b4b0a9170    
        let mut payload = builder.build(device, options);
        let url = format!("navStack\\{}", name);

        let payload_url = PayloadURL {
            url: url,
        };

        payload.add_custom_data("url", &payload_url)?;
        
        let mut file = File::open("private_key.p8")?;
    
        let client = Client::token(
            &mut file,
            "RNL77KG567",
            "7W5ZSCHZ2W",
            Endpoint::Sandbox)?;
    
        let response = client.send(payload).await?;
        println!("Sent: {:?}", response);
    
        Ok(())
    }
    
}

