import type { Jwt } from './types'

/*
 * Public
 */
export function clearExpiredToken() {
    sessionStorage.removeItem('byotoken')
}

export function getUserId(): string | null {
    const token = sessionStorage.getItem('byotoken')

    if (token != null) {
        const jwt = parseJwt(token)

        return jwt.sub
    }

    return null
}

export function isAdmin(): boolean {
    const token = sessionStorage.getItem('byotoken')

    if (token != null) {
        const jwt = parseJwt(token)

        return jwt.role === 'admin'
    }

    return false
}

export function isAuthenticated(): boolean {
    const token = sessionStorage.getItem('byotoken')

    if (token != null) {
        const jwt     = parseJwt(token)
        const expired = jwtIsExpired(jwt)

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
    const exp = jwt.exp * 1000 // convert milliseconds to seconds

    return now.getTime() > exp
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