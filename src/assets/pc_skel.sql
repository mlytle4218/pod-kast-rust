PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE categories (
	category_id INTEGER NOT NULL, 
	category VARCHAR(250), 
	PRIMARY KEY (category_id)
);
CREATE TABLE podcasts (
	podcast_id INTEGER NOT NULL, 
	name VARCHAR(250), 
	url VARCHAR(250), 
	audio VARCHAR(250), 
	video VARCHAR(250),
	category_id INTEGER,
	collection_id INTEGER,
	viewed Integer,
	PRIMARY KEY (podcast_id), 
	FOREIGN KEY(category_id) REFERENCES categories (category_id)
);
CREATE TABLE episodes (
	episode_id INTEGER NOT NULL, 
	title VARCHAR(100), 
	published DATETIME, 
	summary VARCHAR(500), 
	length INTEGER, 
	audio INTEGER, 
	url VARCHAR(250), 
	downloaded INTEGER, 
	podcast_id INTEGER,
	viewed Integer, 
	PRIMARY KEY (episode_id), 
	FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
);
INSERT INTO categories (category) VALUES ("Uncategorized");
COMMIT;
