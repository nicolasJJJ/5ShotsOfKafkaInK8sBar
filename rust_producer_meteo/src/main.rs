use std::time::Duration;
use reqwest::Error;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json;
use std::fs;
mod api;


#[tokio::main]
async fn main() {
    println!("sel !");
    let config_api = load_config();

    match config_api{
        Ok(conf) => {

            for api in conf.apis {
                println!("Prepared URL for {}: {}", api.name, api.url);
                for param in api.params {
                    for tuple in param{
                        println!("gros param des familles : {}", tuple);
                    }
                    //let url = prepare_url(&api.url, &param_set);
                    
                    // Ici, tu pourrais faire ton appel d'API avec `url`
                }
            }
        }
        Err(e) => println!("Erreur lors de la récupération des configuations: {}", e),
    }


    let body_api = read_api().await;
    match body_api{
        Ok(body) => println!("Réponse reçue : {}", body), // Si on a un body, on l'affiche.
        Err(e) => println!("Erreur lors de la récupération de la réponse: {}", e), // Si on a une erreur, on l'affiche.
    }
}


fn load_config() -> serde_json::Result<api::ConfigAPI> {
    let data = fs::read_to_string("./config/apiparams.json").expect("Unable to read file");
    let config: api::ConfigAPI = serde_json::from_str(&data)?;
    Ok(config)
}


/* 
async fn produce() {
  /*//let brokers = "kafka:9092";
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
                  .headers(OwnedHeaders::new().add(
                      "header_key",
                      "header_value",
                  )),
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
  }*/
}
*/



async fn read_api() -> Result<String, reqwest::Error> {
    // L'URL de l'API que tu souhaites appeler
    //Le Vésinet [48.8928, 2.1331]
    //Clé API : 589f74f07d7072d01dd30f1a2dc96dd2
    let url = "http://api.openweathermap.org/data/2.5/air_pollution?lat=48.8928&lon=2.1331&appid=";

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
