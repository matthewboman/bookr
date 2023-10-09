# Map of venues for booking tours

  

## Project info

-  `/server`
	- REST API written in Rust
	- Handles JWT-based authentication/authorization
	- Client to retrieve GeoJSON data from Google Maps API
	- Postgres && Redis
-  `/frontend` 
	- SPA written with Sveltekit that renders a filterable map of venues
	- Flowbite/Tailwind-based CSS
	- Leaflet map library

-  `/addr_to_geo` 
	- DEPRECATED: Node script that pulls contacts from the database and adds a latitude/longitude so they can be rendered on a map. 
	- Can be used to bulk-update `Contacts` with lat/lng
  

## Getting started

- Postgres Dockerfile provided as a convenience but not necessary. Example `.env` files use variables from image. Start the Docker service: `docker compose up`
- Run database migrations under `/server/migrations` with `sqlx migrate run`
- Start the server (port 8000 default): `cargo run`
	- Uncomment lines to create `TestUser` in `server/src/startup.rs` first time running app 
	- OR signup and change `status` column in `users` to `confirmed`
	- Comment out `send_confirmation_email` in `server/src/routes/auth/signup.rs` if email client not configured locally.
- Start the SPA: `npm run dev`
- For frontend/UI development without setting up a Rust backend, checkout [FRONTEND.md](https://github.com/matthewboman/bookr/blob/main/FRONTEND.md)

## Testing

-  `cargo test` || `cargo test -- --nocapture`
- `main.rs` tests the different features. You may need to run one group of tests at a time.
- OR run serial `RUST_TEST_THREADS=1`
- OR or increase `ulimit`
- Temporary databases are created with hashes for names. If you need to clean them up run `psql -Atqc "SELECT 'DROP DATABASE ' || quote_ident(datname) || ';' FROM pg_database WHERE datname like '%7%';" | psql` replacing `7` with any number that's not in the name of one of your existing databases

## Tech improvements

- grouping map points
- postgis
	- need to implement: https://github.com/jmoiron/sqlx/issues/129
- UI theme, usability
- response messages for: signing up, logging in, logging out, adding contacts, errors
- validate `contactForm` is link when submitted
- state management
	- a lot in UI needs updated when user signs in/out
	- endpoints should return newly added/edited data and update UI
- make API CRUD (currently only GET/POST)
- GraphQL?

## Potential Features

- filter by contact type: promoter, venue, DIY/house, band
- create tour
	- select date range
	- add multiple contacts to each date, set to: contacted, pending, confirmed, n/a
	- adjust date of events
	- export to CSV formatted for Songkick import
	- calculate drive times for routes
- mass-import contacts from CSV
- login w/ google
- ability to share private contacts w/ specific users