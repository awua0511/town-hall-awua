CREATE TABLE solutions (
    solution_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    quest_id BIGINT NOT NULL REFERENCES quests (quest_id),
    adventurer_id BIGINT NOT NULL REFERENCES users (user_id),
    github_link TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'submitted',
    CONSTRAINT status_is_valid CHECK (
        status IN ('submitted', 'approved', 'rejected')
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE INDEX idx_solutions_quest_id ON solutions (quest_id);
CREATE INDEX idx_solutions_adventurer_id ON solutions (adventurer_id);