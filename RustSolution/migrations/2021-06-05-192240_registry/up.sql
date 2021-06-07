CREATE TABLE subrip_reg (
  id INTEGER PRIMARY KEY,
  filename VARCHAR NOT NULL,
  content TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
