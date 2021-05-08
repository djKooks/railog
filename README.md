# Traili

## Run via source
```sh
$ cargo run -- --config [config-file-path]
```

### Config example
[config.yml]
```yml
- input_type: kafka
  brokers: 'localhost:9092'
  topics: [test-topic]
  group_id: group
  document: traili
  log_level: debug

```
