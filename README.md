# Map of venues for booking tours

## Project info
- `/server` is a Rest enpoint written in Rust that connects to a Postgres database
- `/frontend` is a web application written in Svelte that renders a map of venues
- `/addr_to_geo` is a Node script that pulls contacts from the database and adds a latitude/longitude so they can be rendered on a map. Maybe deprecated now that this happens on admin approval

## Getting started
- Postgres Dockerfile provided as a convenience but not necessary. Example `.env` files use variables from image. Start the Docker service: `docker compose up`

Database migrations are under `/server`

## Tech improvements
- grouping map points
- postgis
    - need to implement: https://github.com/jmoiron/sqlx/issues/129
- UI theme, usability
- response messages for: signing up, logging in, logging out, adding contacts, errors

## Potential Features
- authenticated users should be able to....
    - add notes/rating/feeback--public or private?
- filter by genre
- venue/contact type
    - filter by bar, venue, house, promoter, other
- create route && export to CSV
- editing contacts
    - admin edit pending contacts
    - admin edit public contacts
    - user edit own contacts