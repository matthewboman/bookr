export type Contact = {
    contactId:   number,
    displayName: string,
    address:     string,
    city:        string,
    state:       string,
    zipCode:     string,
    country:     string | null,
    capacity:    number | null,
    latitude:    number | null,
    longitude:   number | null,
    email:       string | null,
    contactForm: string | null,
    ageRange:    string | null,
    isPrivate:   boolean
}

export type Jwt = {
    sub:  string,
    iat:  number,
    exp:  number,
    role: string,
}

export type NewContact = {
    displayName: string,
    address:     string | null,
    city:        string,
    state:       string,
    zipCode:     string,
    // country:     string,
    capacity:    number | null,
    email:       string | null,
    contactForm: string | null,
    ageRange:    string,
    isPrivate:   boolean
}

export type NewReview = {
    contactId: number,
    userId:    string,
    title:     string,
    body:      string,
    rating:    number,
}

export type Review = {
    reviewId:  string,
    contactId: number,
    userId:    string,
    title:     string,
    body:      string,
    rating:    number,
}

export type User = {
    email:    string,
    password: string
}
