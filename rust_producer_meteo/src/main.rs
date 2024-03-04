use std::time::Duration;
use reqwest::Error;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json;
use std::fs;
mod api;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("sel !");

    //let mut handles = vec![];

    //Loading Apikey objects.
    let api_keys = match load_keys() {
        Ok(keys) => keys,
        Err(e) => {
            println!("Erreur lors de la récupération des clés: {}", e);
            return;
        }
    };


    //Loading ConfigAPI object
    let config_api = match load_config() {
        Ok(conf) => conf,
        Err(e) => {
            println!("Erreur lors de la récupération des configurations: {}", e);
            return;
        }
    };

    for api in config_api.apis {
        if let Some(key) = api_keys.find_value_by_key(&api.api) {
            for param in api.params.clone() {
                let mut url = api.url.clone();
                for (key, valeur) in param{
                    url = url.replace(&format!("{{{}}}", key), &valeur);
                }
                url = url.replace(&format!("{{{}}}", "KEY"), &key);
                let body_api = read_api(url.clone()).await;
                match body_api{
                    Ok(body) => println!("Réponse reçue : {}", body), 
                    Err(e) => {
                        println!("Erreur lors de la récupération de la réponse: {}", e);
                        return;
                     } 
                }
            }
        } else {
            println!("Key not found for : {}. Check the apikey.json file or the apiparams.json one.", &api.api);
        }

  
    }
}


fn load_config() -> serde_json::Result<api::ConfigAPI> {
    let data = fs::read_to_string("./config/apiparams.json").expect("Impossible de récupérer les paramètres d'APIs");
    let config: api::ConfigAPI = serde_json::from_str(&data)?;
    Ok(config)
}

fn load_keys() -> serde_json::Result<api::Apikey> {
    let data = fs::read_to_string("./config/apikey.json").expect("Impossible de récupérer les clés d'APIs");
    let config: api::Apikey = serde_json::from_str(&data)?;
    Ok(config)
}



async fn produce() {
  //let brokers = "kafka:9092";
  let topic = "sample_topic";
  let producer: &FutureProducer = &ClientConfig::new()
  .set("bootstrap.servers", "kafka:9092")  // Remplacement par ton adresse de serveur Kafka
  .set("message.timeout.ms", "5000")
  .create()
  .expect("Producer creation error");

  let futures = (0..5)
  .map(|i| async move {
      // The send operation on the topic returns a future, which will be
      // completed once the result or failure from Kafka is received.
      let delivery_status = producer
          .send(
              FutureRecord::to(topic)
                  .payload(&format!("Message {}", i))
                  .key(&format!("Key {}", i))
                  /* .headers(OwnedHeaders::new().add(
                      "header_key",
                      "header_value",
                  ))*/,
              Duration::from_secs(0),
          )
          .await;

      // This will be executed when the result is received.
      println!("Delivery status for message {} received", i);
      delivery_status
  })
  .collect::<Vec<_>>();

// This loop will wait until all delivery statuses have been received.
  for future in futures {
    println!("Future completed. Result: {:?}", future.await);
  }
}

async fn read_api(url : String) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await; // On tente d'obtenir le body directement.
    match result {
        Ok(body) => {
            Ok(body)
        },
        Err(e) => {
            Err(e)
        },
    }
}
