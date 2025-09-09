CREATE TABLE tasks (
    task_id BYTEA PRIMARY KEY,
    content TEXT NOT NULL ,
    completed BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ( octet_length(task_id) = 16 )
);
