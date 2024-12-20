CREATE TABLE IF NOT EXISTS diets (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO diets (name) VALUES ('Ketogenic');
INSERT INTO diets (name) VALUES ('LowCarb');
INSERT INTO diets (name) VALUES ('CalorieRestricted');
INSERT INTO diets (name) VALUES ('Vegetarian');
INSERT INTO diets (name) VALUES ('Mediterranean');


CREATE TABLE IF NOT EXISTS user_settings (
    user_id VARCHAR(250) PRIMARY KEY,
    diet_id INTEGER,
    no_of_persons SMALLINT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (diet_id) REFERENCES diets(id)
);

