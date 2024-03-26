/*
  kong HTTP client
  written in vanilla Javascript
*/

/// Kong errors
const KongError = {
    InvalidInput: Error('Invalid user input'),
    InternalServer: Error('Internal server error'),
    InvalidPassword: Error('Invalid Password. Length cannot be less than 10 characters long'),
    InvalidUsername: Error('Invalid Username. Length cannot be greater than 15 characters'),
    InvalidEmail: Error('Invalid email'),
    NotFound: Error('Resource does not exist'),
    Unauthorized: Error('Unauthorized, please login')
};

/// User input validator
const Validate = {
    /// username validation
    username(username){
	if (username.length === 0){
	    throw KongError.InvalidUsername;
	}

	// TODO: more username validation
    },

    email(email){
	if (email.length === 0){
	    throw KongError.InvalidEmail;
	}

	// TODO: more username validation
    },

    /// password validation
    password(password){
	// check password length
	if (password.length < 10){
	    throw KongError.InvalidPassword;
	}
    }
};
/// Account creation input
class KongAccountCreationInput {
    constructor(username, email, password){
	this.username = username;
	this.email = email;
	this.password = password;
    }

    /// Validate input
    validate(){
	Validate.username(this.username);

	if (this.email){
	    Validate.email(this.email);
	}

	Validate.password(this.password)
    }
}

const KongAccountsAPI = {
    address: "/accounts",
    /// Create a new kong account
    async create_account(account_creation_input) {
	if (!account_creation_input instanceof KongAccountCreationInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	account_creation_input.validate();

	return fetch(this.address,{
	    method: "POST",
	    headers: {
		"Content-Type": "application/json",
	    },
	    body: JSON.stringify(account_creation_input),
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
}
/// Account authenticatation input
class KongAccountAuthInput{
    constructor(username, password){
	this.username = username;
	this.password = password;
    }

    /// Validate input
    validate(){
	Validate.username(this.username);
	Validate.password(this.password)
    }
}

const KongLoginAPI = {
    address: "/login",
    /// Authenticate (login) user
    async authenticate(account_auth_input){
	if (!account_auth_input instanceof KongAccountAuthInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	account_auth_input.validate();

	return fetch(this.address, {
	    method: 'POST',
	    headers: {
		'Content-Type': 'application/json',
	    },
	    body: JSON.stringify(account_auth_input),
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
}
class KongContactMessageInput{
    constructor(name, email, message){
	this.name = name;
	this.email = email;
	this.message = message;
    }

    /// Validate input
    validate(){
	Validate.email(this.email);
    }
}

const KongContactMessageAPI = {
    address: "/contact",

    /// send message
    async send_message(contact_message_input){
	if (!contact_message_input instanceof KongContactMessageInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	contact_message_input.validate();

	return fetch(this.address, {
	    method: 'POST',
	    headers: {
		'Content-Type': 'application/json',
	    },
	    body: JSON.stringify(contact_message_input),
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
}
class KongNewsletterSubcriberInput{
    constructor(email){
	this.email = email;
    }

    /// Validate input
    validate(){
	Validate.email(this.email);
    }
}

const KongNewsletterAPI = {
    address: "/newsletter",

    /// subscribe to newsletter
    async subscribe(subscription_input){
	if (!subscription_input instanceof KongNewsletterSubcriberInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	subscription_input.validate();

	return fetch(this.address, {
	    method: 'POST',
	    headers: {
		'Content-Type': 'application/json',
	    },
	    body: JSON.stringify(subscription_input),
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.InvalidInput;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
}
/// Account creation input
class KongBlogInput {
    constructor(title, subtitle, overview, author, cover, content){
	this.title = title;
	this.subtitle = subtitle;
	this.overview = overview;
	this.author = author;
	this.cover = cover;
	this.content = content;
    }

    /// Validate input
    validate(){
	// TODO:
    }
}

const KongBlogAPI = {
    address: "/blog",

    /// Submit blog
    async post_blog(input) {
	if (!input instanceof KongBlogInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	input.validate();

	const formData = new FormData();

	formData.append("title", input.title);
	formData.append("content", input.content);
	formData.append("subtitle", input.subtitle);
	formData.append("overview", input.overview);
	formData.append("author", input.author);
	formData.append("cover", input.cover);

	return fetch(this.address, {
	    method: "POST",
	    body: formData,
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.Unauthorized;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },

    /// Get all blog posts
    async get_all() {
	return fetch(this.address, {
	    method: "GET",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.Unauthorized;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
    /// Get single blog post by id
    async get_by_id(id) {
	return fetch(this.address+"/"+id, {
	    method: "GET",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.Unauthorized;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },
    /// Get single blog post by id
    async delete_by_id(id) {
	return fetch(this.address+"/"+id, {
	    method: "DELETE",
	})
	    .then((response) => {
		switch (response.status){
		case 200:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.Unauthorized;
		case 404:
		    throw KongError.NotFound;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    },

    /// Submit blog
    async update(id, input) {
	if (!input instanceof KongBlogInput){
	    throw KongError.InvalidInput;
	}

	// validate input
	input.validate();

	const formData = new FormData();

	formData.append("title", input.title);
	formData.append("content", input.content);
	formData.append("subtitle", input.subtitle);
	formData.append("overview", input.overview);
	formData.append("author", input.author);
	formData.append("cover", input.cover);

	return fetch(this.address+"/"+id, {
	    method: "PUT",
	    body: formData,
	})
	    .then((response) => {
		switch (response.status){
		case 201:
		    return response.json();
		case 400:
		    throw KongError.InvalidInput;
		case 401:
		    throw KongError.Unauthorized;
		case 500:
		    throw KongError.InternalServer;
		}
	    })
	    .catch((error) => {
		throw error;
	    });
    }
};
