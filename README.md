# Map of venues for booking tours

## Project info
- `/server` is a Rest enpoint written in Rust that connects to a Postgres database
- `/frontend` is a web application written in Svelte that renders a map of venues
- `/addr_to_geo` is a Node script that pulls contacts from the database and adds a latitude/longitude so they can be rendered on a map.

## Getting started
- Postgres Dockerfile provided as a convenience but not necessary. Example `.env` files use variables from image. Start the Docker service: `docker compose up`

Database migrations are under `/server`

## Tech improvements
- grouping map points
- postgis
    - need to implement: https://github.com/jmoiron/sqlx/issues/129 
- jwt bug where frontend thinks user is authenticated even though token is expired
- joining public and private contacts for authenticated users
- admin
    - approve pending contacts
- move node `addr_to_geo` script to rust backend
    - run on admin approval or as cron?
- JWT expiration
- reset_password token expiration
- consistent error handling in API
- UI theme, usability
- response messages for: signing up, logging in, logging out, adding contacts, errors

## Potential Features
- authenticated users should be able to....
    - users create public contacts--what's the verification process?
    - add notes/rating/feeback--public or private?
- filter by genre
- venue/contact type
    - filter by bar, venue, house, promoter, other
- create route && export to CSV