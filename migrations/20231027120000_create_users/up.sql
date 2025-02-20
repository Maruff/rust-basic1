CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE, -- Add unique constraint for email
    password_hash VARCHAR(255) NOT NULL,  -- Store the password hash
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
