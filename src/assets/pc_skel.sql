PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE podcasts (
	podcast_id INTEGER NOT NULL, 
	name VARCHAR(250), 
	url VARCHAR(250), 
	audio VARCHAR(250), 
	video VARCHAR(250), category varchar(250), 
	PRIMARY KEY (podcast_id)
);
CREATE TABLE episodes (
	episode_id INTEGER NOT NULL, 
	title VARCHAR(100), 
	published DATETIME, 
	summary VARCHAR(500), 
	length INTEGER, 
	audio INTEGER, 
	href VARCHAR(250), 
	downloaded INTEGER, 
	podcast_id INTEGER, veiwed Integer, 
	PRIMARY KEY (episode_id), 
	FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
);
CREATE TABLE categories (
	category_id INTEGER NOT NULL, 
	category VARCHAR(250), 
	PRIMARY KEY (category_id)
);
COMMIT;
