## rustground 🛝

**rustground** is a rust web-playground on steroids.

### Development

You'll need docker, cargo and npm installed on your machine to spawn the various
components the project needs to run locally.

First, mount a local elastic search and judge0 instance using docker:

```bash
$ docker compose up -d
```

Spawn the server with a data source:

```bash
$ cargo run -- --source crates.json serve
```

n.b. the server cli provides a load subcommand for fetching all crates from
cargo and building a json data source.

Spawn the svelte frontend:

```bash
$ npm install
$ npm run dev
```
