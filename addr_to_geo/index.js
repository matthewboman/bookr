require('dotenv').config()

const GoogleMapsAPI    = require('googlemaps')
const { Pool } = require("pg")

const mapsConfig = {
    key:              process.env.GOOGLE_API_KEY,
    stagger_time:     1000,
    encode_polylines: false,
    secure:           true
}

const dbCredentials = {
    user:     process.env.DB_USER,
    host:     process.env.DB_HOST,
    database: process.env.DB_NAME,
    password: process.env.DB_PASSWORD,
    port:     process.env.DB_PORT,
}

// fetchGeoLocation :: String -> Promise (Err [{}])
const fetchGeoLocation = address => new Promise((resolve, reject) => {
    const gmAPI         = new GoogleMapsAPI(mapsConfig)
    const geocodeParams = { address, language: 'en' }

    gmAPI.geocode(geocodeParams, (err, res) => {
        if (err) {
            reject(err)
        }
        if (res && res.results[0]) {
            resolve(res.results[0].geometry.location)
        }
        resolve(null)
    })
})

// addGeolocation :: Pool, Object, Object -> Result
async function addGeolocation(pool, contact, latLng) {
    const text   = `UPDATE contacts 
                    SET latitude = ${latLng.lat}, longitude = ${latLng.lng} 
                    WHERE contact_id = ${contact.contact_id}`
    const result = await pool.query(text)

    return result
}

async function findGeolocations(pool) {
    const text     = `SELECT contact_id, address, state, country, zip_code 
                      FROM contacts WHERE latitude IS NULL`
    const result   = await pool.query(text)

    // Counter to close pool after all updates have run
    let updates    = result.rows.length

    result.rows.forEach(async r => {
        const address = `${r.address} ${r.state} ${r.zip_code}`
        const latLng  = await fetchGeoLocation(address)

        if (latLng != null) {
            await addGeolocation(pool, r, latLng)
            updates --
        }

        if (updates < 1) {
            await pool.end()
        }
    }) 
}

(async () => {
    const pool = new Pool(dbCredentials)
  
    await findGeolocations(pool)
})()