const API_URL = "http://127.0.0.1:8000"

export async function get(endpoint: String) {
    const headers = setHeaders()

    return await fetch(`${API_URL}${endpoint}`, {
        method: 'GET',
        headers,
        credentials: 'include'
    })
}

export async function post(endpoint: String, body: Object) {
    let headers = setHeaders()

    return await fetch(`${API_URL}${endpoint}`, {
        method: 'POST',
        body: JSON.stringify(body),
        headers,
        credentials: 'include'
    })
}

const setHeaders = () => {
    const token = sessionStorage.getItem('byotoken')
    return token
        ? {
            "Content-type": "application/json; charset=UTF-8",
            "Authorization": `Bearer ${token}`
        }
        : { "Content-type": "application/json; charset=UTF-8" }
}