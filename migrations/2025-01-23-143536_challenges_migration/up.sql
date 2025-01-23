-- Your SQL goes here
CREATE TABLE challenges
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    start_time TIMESTAMP,
    end_time TIMESTAMP
);

CREATE TABLE leaderboard
(
    user_id INTEGER NOT NULL,
    challenge_id INTEGER NOT NULL,
    score INTEGER NOT NULL,
    CONSTRAINT fk_leaderboard_userid FOREIGN KEY (user_id) REFERENCES "user"(id),
    CONSTRAINT fk_leaderboard_challenge FOREIGN KEY (challenge_id) REFERENCES challenges(id),
    CONSTRAINT unique_user_challenge UNIQUE (user_id, challenge_id)
);
