#[macro_use] extern crate lazy_static;
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json;
use std::fs;
mod api;
use tokio;
use futures;
use rdkafka::error::KafkaError;
use std::sync::Mutex;


lazy_static! {
    static ref PRODUCER: Mutex<FutureProducer> = Mutex::new(
        ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .create()
            .expect("Failed to create producer")
    );
}

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
    let mut handles = Vec::new();

    //Producer here, used later
    

        //let messages_to_send = vec!["Message 1", "Message 2", "Message 3"];

        /*for message in messages_to_send {
            // Prépare le message. Ici, tu ajustes selon ton topic et tes besoins
            let record = FutureRecord::to("mon_topic")
                .payload(message)
                .key("ma_clef");
    
            // Envoie le message
            match producer.send(record, Duration::from_secs(0)).await {
                Ok(delivery) => println!("Message envoyé : {:?}", delivery),
                Err((e, _)) => println!("Erreur d'envoi : {}", e),
            }
        }*/

    for api in config_api.apis {
        if let Some(key) = api_keys.find_value_by_key(&api.api) {
            for param in api.params.clone() {
                let mut url = api.url.clone();
                let api_n = api.name.clone();
                for (key, valeur) in param{
                    url = url.replace(&format!("{{{}}}", key), &valeur);
                }
                url = url.replace(&format!("{{{}}}", "KEY"), &key);
                let handle = tokio::spawn(async move {
                    match read_api(url).await {
                        Ok(body) => {
                            println!("Réponse reçue : {}", body);
                            
                            match produce(body.clone(), &api_n).await {
                                Ok(_) => println!("Message envoyé"),
                                Err(e) => println!("Erreur lors de l'envoi du message: {:?}", e),
                            }
                        },
                        Err(e) => println!("Erreur lors de la récupération de la réponse: {}", e),
                    }
                });
                handles.push(handle);
            }
        } else {
            println!("Key not found for : {}. Check the apikey.json file or the apiparams.json one.", &api.api);
        }
    }
    futures::future::join_all(handles).await;

    /*for result in results {
        match result {
            Ok(body) => println!("Réponse reçue),
            Ok(Err(e)) => println!("Erreur lors de la récupération de la réponse: {}", e),
            Err(e) => println!("Une tâche a paniqué : {:?}", e),
        }
    }*/
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


//: &FutureProducer = &ClientConfig::new()
async fn produce(body: String, topic : &String) -> Result<(), KafkaError> {

  let producer_guard = PRODUCER.lock().unwrap().clone();

  let record = FutureRecord::to(topic)
  .key(&topic)
  .payload(&body);

  producer_guard.send(record, Duration::from_secs(0)).await
        .map(|delivery| {
            println!("Message envoyé : {:?}", delivery);
            ()
        })
        .map_err(|(e, _)| {
            println!("Erreur d'envoi : {}", e);
            e
        })
    
      // This will be executed when the result is received.

// This loop will wait until all delivery statuses have been received.

}

async fn read_api(url : String) -> Result<String, reqwest::Error> {
    let result = reqwest::get(url).await?.text().await; 
    match result {
        Ok(body) => {
            Ok(body)
        },
        Err(e) => {
            Err(e)
        },
    }
}
