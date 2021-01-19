CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE projects (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL
);

CREATE TABLE tags (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE files (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL
);

CREATE TABLE images (
  id SERIAL NOT NULL PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  modified_at TIMESTAMP NOT NULL
);

CREATE TABLE user_projects (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id) NOT NULL,
  project_id INTEGER REFERENCES projects(id) NOT NULL,
  UNIQUE(user_id, project_id)
);

CREATE TABLE project_tags (
  id SERIAL NOT NULL PRIMARY KEY,
  project_id INTEGER REFERENCES projects(id) NOT NULL,
  tag_id INTEGER REFERENCES tags(id) NOT NULL,
  UNIQUE(project_id, tag_id)
);

CREATE TABLE project_files (
  id SERIAL NOT NULL PRIMARY KEY,
  project_id INTEGER REFERENCES projects(id) NOT NULL,
  file_id INTEGER REFERENCES files(id) NOT NULL,
  UNIQUE(project_id, file_id)
);

CREATE TABLE project_images (
  id SERIAL NOT NULL PRIMARY KEY,
  project_id INTEGER REFERENCES projects(id) NOT NULL,
  image_id INTEGER REFERENCES images(id) NOT NULL,
  UNIQUE(project_id, image_id)
);