DROP INDEX idx_access_tokens_token;
DROP INDEX idx_access_tokens_astronaut_id;
DROP INDEX idx_astronauts_pass_hashed_astronaut_id;
DROP INDEX idx_astronauts_email;

DROP TABLE IF EXISTS astronauts_pass_hashed;
DROP TABLE IF EXISTS astronauts;