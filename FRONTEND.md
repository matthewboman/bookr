# Development without a backend
You can replace the following methods to do front-end development without the API

## ./src/routes/+page.svelte

        async  function  getPublicContacts() {
	        let contacts = [
		        {
			        'contactId':  1,
			        'userId':  'qwerty',
			        'displayName':  'Static Age',
			        'address':  '110 N Lexington Ave', 
			        'city':  'Asheville',
			        'state':  'NC',
			        'zipCode':  '28801,
			        'country':  'USA',
			        'capacity':  49,
			        'latitude': 35.5951,
			        'longitude':  -82.5515,
			        'contactForm':  'https://www.facebook.com/static.agerecords.7',
			        'ageRange':  'allAges',
			        'isPrivate':  false,
			        'contactType':  'venue',
			        'averageRating':  4.5,
			        'genres':  [
				        {'genreId':  1,  'genreName':  'alternative/rock'},
				        {'genreId':  3,  'genreName':  'punk/hardcore'},
				        {'genreId':  2,  'genreName':  'indie'}
			        ]
		        }
	        ]
        }

        async function getGenres() {
	        $genres = [
		        {'genreId': 1, 'genreName': 'alternative/rock'},
		        {'genreId': 3, 'genreName': 'punk/hardcore'},
		        {'genreId': 2, 'genreName': 'indie'},
		        {'genreId': 4, 'genreName': 'metal}
			 ]
        }


## ./src/routes/contact/+page.svelte

    async function getContact(contactId: string) {
	    $selectedContact = {
			        'contactId':  1,
			        'userId':  'qwerty',
			        'displayName':  'Static Age',
			        'address':  '110 N Lexington Ave', 
			        'city':  'Asheville',
			        'state':  'NC',
			        'zipCode':  '28801,
			        'country':  'USA',
			        'capacity':  49,
			        'latitude': 35.5951,
			        'longitude':  -82.5515,
			        'contactForm':  'https://www.facebook.com/static.agerecords.7',
			        'ageRange':  'allAges',
			        'isPrivate':  false,
			        'contactType':  'venue',
			        'averageRating':  4.5,
			        'genres':  [
				        {'genreId':  1,  'genreName':  'alternative/rock'},
				        {'genreId':  3,  'genreName':  'punk/hardcore'},
				        {'genreId':  2,  'genreName':  'indie'}
			        ]
		        }
    }
    async function getReviews(contactId: string) {
	    $contactReviews = [
		    {
			    'body': "this might have a lot or a little text",
			    'contactId': 1,
			    'rating': 4,
			    'title': 'overall vibe',
			    'userId': 'qwerty'
			},
			{
			    'body': "this can say a lot more details about the space",
			    'contactId': 1,
			    'rating': 5,
			    'title': 'titles are short',
			    'userId': 'asdfg'
			}
		]
    }