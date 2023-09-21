import { writable } from 'svelte/store'

export const admin           = writable(false)
export const adminReviews    = writable([])
export const authenticated   = writable(false)
export const contactReviews  = writable([])
export const genres          = writable([])
export const selectedContact = writable(null)
export const selectedReview  = writable(null)
export const userId          = writable(null)