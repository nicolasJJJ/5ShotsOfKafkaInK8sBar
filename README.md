# goal
The main goal is to publish some recreatives projects for me to learn, exchange with people knowing more than me and maybe offer me new opportunities for data engineering projects.




# LittleMeteoProject
A Kafka project with K8s. 

K8S : 
<img src="https://github.com/nicolasJJJ/5ShotsOfKafkaInK8sBar/assets/104780543/aa50ace3-3372-49eb-91ad-4d53faaf0d1a" width="10%">

https://github.com/apache-spark-on-k8s/kubernetes-HDFS/tree/master

## rust producer meteo

The cargo directory containing the Rust producer.
Adjust the config files in rust_producer_meteo/config/apiparams.json
Create rust_producer_meteo/config/apikey.json

TODO : Dynamically get the Minikube IP.
For cloud and K8s : definite a DNS for a probable change of IP ?

```
{
    "keys": [
        {"OPENWEATHERMAP": "KEY ..." },
        {"OTHER_API ": "OTHER KEY ..."}
    ]
}

```

## Configs folder
Contains configs read by minikube to start the services, pdbs, deployments and pvcs.

This cluster would contain : 
- HDFS
- Kafka + Schema Registry & Zookeeper
- Spark consumer with Scala

```
sh src/minikube/start-env.sh
```

TODO : In Kafka Statefulset, Dynamically get the minikube IP for External gateway

## Spark

Not created yet. Would be a script made to "normalize" different sources.

## Data sources / dashboard

Graphs comparing air quality and weather in quiet zones like Le Vésinet vs a urban zone like Saint-Denis

openweathermaps.com
