-- Your SQL goes here
CREATE TABLE users(
	  id INT NOT NULL PRIMARY KEY,
	  first_name TEXT NOT NULL,
	  last_name TEXT NOT NULL,
	  email TEXT NOT NULL,
	  created_at TIMESTAMP NOT NULL
	);
	CREATE TABLE profiles(
	    ProfileId SERIAL NOT NULL PRIMARY KEY,
	    profileName TEXT NOT NULL,
	    created_at TIMESTAMP NOT NULL,
	    userr_id INT NOT NULL,
	  constraint fk_users
	     foreign key (userr_id)
	     REFERENCES users (id)
	    -- CONSTRAINT fk_customer
	    --   id SERIAL FOREIGN KEY(id)
		--   REFERENCES users(id)
		--   ON DELETE SET NULL
	);