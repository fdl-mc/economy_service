CREATE TABLE IF NOT EXISTS economy_states (
    id INT GENERATED ALWAYS AS IDENTITY,
    user_id INT UNIQUE,
    balance INT,
    banker BOOL
);

CREATE TABLE IF NOT EXISTS transactions (
    id INT GENERATED ALWAYS AS IDENTITY,
    payer_id INT,
    payee_id INT,
    amount INT,
    comment TEXT,
    at INT
);