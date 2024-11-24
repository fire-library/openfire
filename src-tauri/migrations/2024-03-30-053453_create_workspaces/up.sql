-- Your SQL goes here
CREATE TABLE workspaces (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  directory TEXT NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
)