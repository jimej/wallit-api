ALTER TABLE users ALTER COLUMN last_modified DROP NOT NULL;
ALTER TABLE groups ALTER COLUMN last_modified DROP NOT NULL;
ALTER TABLE logins ALTER COLUMN last_modified DROP NOT NULL;
-- ALTER TABLE user_teams ALTER COLUMN last_modified DROP NOT NULL; -- no last_modified
-- ALTER TABLE history ALTER COLUMN last_modified DROP NOT NULL; -- this is ok because of no user interaction; actually no last_modified