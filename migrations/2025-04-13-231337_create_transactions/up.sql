CREATE TABLE transactions (
    uuid TEXT NOT NULL PRIMARY KEY,
    amount INTEGER NOT NULL,
    client_modified_at DATETIME NOT NULL,
    exponent INTEGER NOT NULL,
    merchant TEXT NOT NULL,
    occurred_at DATETIME NOT NULL,
    parent TEXT NOT NULL,
    server_modified_at DATETIME NOT NULL
);
