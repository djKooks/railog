# Traili
// TODO: Unprepared status


## Run via source
### Prepare...
1. Run MS in local
2. Run Kafka server
3. Run project with command:
```sh
$ cargo run -- --config [config-file-path]
```

### Config file example
[config.yml]
```yml
- input_type: kafka
  brokers: localhost:9092
  topics: [test-topic]
  group_id: group
  document: traili
  log_level: debug
```
