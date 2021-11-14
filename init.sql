CREATE TABLE IF NOT EXISTS stores (
    id      SERIAL PRIMARY KEY,
    name    VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS items (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255),
    store_id    INTEGER REFERENCES stores (id) ON DELETE CASCADE,
    price       MONEY
);

INSERT INTO stores (name) VALUES 
    ('Amazon'),
    ('Natural Grocers'),
    ('Petco');

INSERT INTO items (name, store_id, price) VALUES 
    ('studio lights', (SELECT id FROM stores WHERE name = 'Amazon'), 150.00),
    ('celery', (SELECT id FROM stores WHERE name = 'Natural Grocers'), 2.50),
    ('bird feather toy', (SELECT id FROM stores WHERE name = 'Petco'), 10.50);