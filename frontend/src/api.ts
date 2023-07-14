const API_URL = "http://127.0.0.1:8000"

export async function post(endpoint: String, body: Object) {
    let token   = sessionStorage.getItem('byotoken')
    let headers = token
        ? {
            "Content-type": "application/json; charset=UTF-8",
            "Authorization": `Bearer ${token}`
        }
        : { "Content-type": "application/json; charset=UTF-8" }

    return await fetch(`${API_URL}${endpoint}`, {
        method: 'POST',
        body: JSON.stringify(body),
        headers,
        credentials: 'include'
    })
}