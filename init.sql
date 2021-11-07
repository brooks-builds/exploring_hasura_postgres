CREATE TABLE IF NOT EXISTS players (
    id      SERIAL,
    name    text,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS levels (
    id      SERIAL,
    name    TEXT,
    level   JSON,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS scores (
    id          SERIAL,
    score       int,
    player_id   int,
    level_id    int,
    PRIMARY KEY (id),
    CONSTRAINT player_id 
        FOREIGN KEY (player_id)
            REFERENCES players (id)
                ON DELETE CASCADE,
    CONSTRAINT level_id 
        FOREIGN KEY (level_id)
            REFERENCES levels (id)
                ON DELETE CASCADE
);

INSERT INTO players (name) VALUES 
    ('dota2attitude'),
    ('ko2fan'),
    ('codinggardenfan');

INSERT INTO levels (name, level) VALUES
    ('sudoku 1', '[[1,2,3],[4,5,6],[7,9,9]]');

INSERT INTO scores (score, player_id, level_id) VALUES
    (450, (SELECT id FROM players WHERE name = 'dota2attitude'), (SELECT id FROM levels WHERE name = 'sudoku 1'));