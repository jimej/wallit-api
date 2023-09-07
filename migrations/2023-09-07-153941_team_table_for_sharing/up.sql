ALTER TABLE groups ADD COLUMN team BOOLEAN DEFAULT FALSE; --to indicate a shared group

CREATE TABLE IF NOT EXISTS users_teams (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    team_id uuid NOT NULL, -- team refers to a group that's shared
    permission VARCHAR(10) NOT NULL, -- permission: READ/WRITE/DELETE, preferrable enum
    PRIMARY KEY (user_id, team_id),
    CONSTRAINT fk_x_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_x_team_id FOREIGN KEY (team_id) REFERENCES groups (id) ON DELETE CASCADE
)