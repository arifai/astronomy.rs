CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE astronauts (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  full_name VARCHAR NOT NULL,
  avatar TEXT,
  email VARCHAR NOT NULL,
  active BOOLEAN NOT NULL DEFAULT FALSE,
  registered_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_astronauts_email ON astronauts (
    (lower(email))
);

CREATE TABLE astronauts_pass_hashed (
  astronaut_id UUID NOT NULL PRIMARY KEY,
  pass_hashed TEXT,

  CONSTRAINT fk_astronaut FOREIGN KEY (astronaut_id) REFERENCES astronauts (id) ON DELETE CASCADE
);

CREATE INDEX idx_astronauts_pass_hashed_astronaut_id ON astronauts_pass_hashed (
    (astronaut_id)
);

CREATE TABLE access_tokens (
  astronaut_id UUID NOT NULL,
  token TEXT NOT NULL PRIMARY KEY,
  valid_thru TIMESTAMP NOT NULL,
  created_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT fk_astronaut FOREIGN KEY (astronaut_id) REFERENCES astronauts (id) ON DELETE CASCADE
);

CREATE INDEX idx_access_tokens_astronaut_id ON access_tokens (
    (astronaut_id)
);
CREATE UNIQUE INDEX idx_access_tokens_token ON access_tokens (
    (token)
);

INSERT INTO astronauts (full_name, avatar, email, active) VALUES ('Astro Bot', 'https://api.dicebear.com/8.x/identicon/svg?seed=Astro%20Bot', 'bot@astronaut.dev', TRUE);

INSERT INTO astronauts_pass_hashed (astronaut_id, pass_hashed) VALUES ((SELECT id FROM astronauts ORDER BY registered_time DESC LIMIT 1), '$argon2id$v=19$m=16,t=2,p=1$MU1nRTlycUZ2T0dCcTRhRg$CiKW/JjxjDQR9B/6TsN4MA');