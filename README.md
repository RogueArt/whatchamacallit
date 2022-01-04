# About

Whatchamacallit is a web app that allows you to see and vote for geographic differences
in words. Whatchamacallit is being built with React on the frontend and Rust with
Rocket and MongoDB on the backend.

Currently, Whatchamacallit is still in development. Feel free to fork this repository
and make a pull request to make any changes you'd like!

# Getting Started

## Prerequisities

On the client side, install the latest versions of:

- [NodeJS](https://nodejs.org/en/download/)
- [ESLint](https://eslint.org/docs/user-guide/getting-started)
- [Prettier](https://prettier.io/docs/en/install.html)

On the server side, install the latest versions of:

- [Rust stable](https://www.rust-lang.org/tools/install)
- [MongoDB](https://docs.mongodb.com/manual/installation/).

## Installation

1. Clone this repository: `git clone https://github.com/RogueArt/whatchamacallit.git`

2. To install client-side dependencies:

```bash
cd whatchamacallit   # Go into the main folder
cd client            # Go to the client folder
npm install          # Install required dependencies
```

3. To install server-side dependencies:

```bash
cd whatchamacallit   # Go into the main folder
cd server            # Go to server folder
cargo install        # Install required dependencies
```

## Running

To start the client React server:

```bash
cd client
npm run start
```

To start the backend Rocket server:

```bash
cd server
cargo run
```
