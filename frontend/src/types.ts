export type Contact = {
    contactId:   number,
    displayName: string,
    address:     string,
    city:        string,
    state:       string,
    zipCode:     string,
    capacity:    number | null,
    latitude:    number | null,
    longitude:   number | null,
    email:       string | null,
    contactForm: string | null,
    ageRange:    string | null
}

export type NewContact = {
    displayName: string,
    address:     string | null,
    city:        string,
    state:       string,
    zipCode:     string,
    capacity:    number | null,
    email:       string | null,
    contactForm: string | null,
    ageRange:    string | null
}

