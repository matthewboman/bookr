# Map of venues for booking tours

## Project info
- `/server` is a GraphQL enpoint written in Rust that connects to a Postgres database
- `/frontend` is a web application written in Svelte that renders a map of venues
- `/addr_to_geo` is a Node script that pulls contacts from the database and adds a latitude/longitude so they can be rendered on a map.

## Getting started
- Postgres Dockerfile provided as a convenience but not necessary. Example `.env` files use variables from image. Start the Docker service: `docker compose up`

Database migrations are under `/server`