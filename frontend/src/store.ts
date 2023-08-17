import { writable } from 'svelte/store'

export const admin         = writable(false)
export const authenticated = writable(false)