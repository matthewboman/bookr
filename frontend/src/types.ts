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