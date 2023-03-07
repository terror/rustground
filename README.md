## rustground 🛝

**rustground** is a rust web-playground on steroids.

### Development

Spawn the elastic search backend:

```bash
$ docker compose up -d
```

Run the server with a data source:

```bash
$ cargo run -- --source crates.json serve
```

Spawn the svelte frontend:

```bash
$ npm run dev
```
