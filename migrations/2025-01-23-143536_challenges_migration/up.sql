-- Your SQL goes here
CREATE TABLE challenges
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    CONSTRAINT fk_challenges_user FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_challenges_mapid FOREIGN KEY (map_id) REFERENCES map_layout(id)
);

CREATE TABLE challenges_responses
(
    id SERIAL PRIMARY KEY,
    attacker_id INTEGER NOT NULL,
    challenge_id INTEGER NOT NULL,
    map_id INTEGER NOT NULL,
    score INTEGER NOT NULL,
    CONSTRAINT chall_response_userid FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT chall_response_challenge FOREIGN KEY (challenge_id) REFERENCES challenges(id),
    CONSTRAINT chall_response_mapid FOREIGN KEY (map_id) REFERENCES map_layout(id),
    CONSTRAINT unique_user_challenge UNIQUE (user_id, challenge_id)
);