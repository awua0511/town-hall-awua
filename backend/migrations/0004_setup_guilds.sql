CREATE TABLE guilds (
    guild_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE TABLE guild_members (
    guild_id BIGINT NOT NULL REFERENCES guilds (guild_id),
    user_id BIGINT NOT NULL REFERENCES users (user_id),
    role TEXT NOT NULL DEFAULT 'member',
    CONSTRAINT role_is_valid CHECK (
        role IN ('member', 'admin', 'owner')
    ),
    joined_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX idx_guild_members_guild_id ON guild_members (guild_id);
CREATE INDEX idx_guild_members_user_id ON guild_members (user_id);