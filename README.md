
# Prime Path - a math game

## Gameplay

1. Player1 chooses a random number (or it is automatically generated for fairness)
1. Player2 can then either add one to it or divide it by one of its prime factors
1. Then Player1 does the same
1. This repeats until the number 1 is reached (the one who get's the number 1 loses)


## Implementation

### Backend

- Rust with Actix-web

### Frontend

- Typescript with Svelte


## Development

### Frontend

Make sure npm is installed

The first time you need to install the dependencies:
```shell
cd ui
npm install
```

Start the development server:
```shell
cd ui
npm run dev -- --open
```

### Backend

Make sure rust is installed.

Build the UI first:
```shell
npm run build
```

To run the server:
```shell
cargo run
```


## Deploying

You can preview the production build with `cd ui && npm run preview`.

