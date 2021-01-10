-- Your SQL goes here
CREATE TABLE project (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL
);

CREATE TABLE tag (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE file (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL
);

CREATE TABLE user_project (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  project_id INTEGER REFERENCES project(id) NOT NULL
);

CREATE TABLE project_tag (
  id SERIAL NOT NULL PRIMARY KEY,
  project_id INTEGER REFERENCES project(id) NOT NULL,
  tag_id INTEGER REFERENCES tag(id) NOT NULL
);

CREATE TABLE project_file (
  id SERIAL NOT NULL PRIMARY KEY,
  project_id INTEGER REFERENCES project(id) NOT NULL,
  file_id INTEGER REFERENCES file(id) NOT NULL
);


