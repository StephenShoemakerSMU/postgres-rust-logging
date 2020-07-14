CREATE TYPE LEVEL AS ENUM ('debug', 'info', 'error');

CREATE TABLE logs(
    log_id TEXT PRIMARY KEY,
    level LEVEL,
    status_code TEXT,
    header TEXT,
    message TEXT,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    complete_time TIMESTAMP
)