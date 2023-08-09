import type { Jwt } from './types'

/*
 * Public
 */
export function isAuthenticated(): boolean {
    let token = sessionStorage.getItem('byotoken')

    if (token != null) {
        let jwt     = parseJwt(token)
        let expired = jwtIsExpired(jwt)

        if (!expired) {
            return true
        }
    }

    return false
}

/*
 * Private
 */
function jwtIsExpired(jwt: Jwt): boolean {
    const now = new Date()

    return now.getTime() < jwt.exp
}

function parseJwt(token: string): Jwt {
    const base64Url   = token.split('.')[1]
    const base64      = base64Url.replace(/-/g, '+').replace(/_/g, '/')
    const jsonPayload = decodeURIComponent(
        window
            .atob(base64)
            .split('')
            .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
            .join('')
    )

    return JSON.parse(jsonPayload)
}



