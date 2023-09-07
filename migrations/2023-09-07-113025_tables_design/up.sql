CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; 

CREATE TABLE IF NOT EXISTS users (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    username VARCHAR(20) NOT NULL DEFAULT '',
    email VARCHAR(40) UNIQUE NOT NULL,
    first_name VARCHAR(20) NOT NULL,
    last_name VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS groups ( -- there should be a nogroup, and a sharing group
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    group_name VARCHAR(20) NOT NULL,
    user_id uuid NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL,
    -- nogroup BOOLEAN NOT NULL,
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS logins (
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    group_id uuid NOT NULL,
    cname VARCHAR(20) UNIQUE NOT NULL DEFAULT '', -- entity/company name
    url TEXT,
    login TEXT NOT NULL, -- username
    password TEXT NOT NULL,
    email VARCHAR(40) UNIQUE NOT NULL,
    description TEXT,
    -- sharing BOOLEAN NOT NULL DEFAULT FALSE, -- following, follower type relationship
    created_at TIMESTAMP DEFAULT NOW(),
    last_modified TIMESTAMP NOT NULL,
    CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);
-- no chained sharing, or reverse sharing, i.e only the owner can share; can ownership be moved
-- request for sharing - no
-- graph database may be more appropriate for sharing; social network graph
-- use cases: team in company, family
-- simplify - a user_team table b/c smaller scale coompared to social network,
-- to share just create a team and add user and login to the table

CREATE TABLE IF NOT EXISTS history ( -- a larger table compared to logins; TTL or only last 5
    id uuid UNIQUE DEFAULT uuid_generate_v4(),
    group_id uuid NOT NULL,
    cname VARCHAR(20) UNIQUE NOT NULL DEFAULT '', -- entity/company name
    url TEXT,
    login TEXT NOT NULL, -- username
    password TEXT NOT NULL,
    email VARCHAR(40) UNIQUE NOT NULL,
    description TEXT,
    -- sharing BOOLEAN NOT NULL DEFAULT FALSE, -- following, follower type relationship
    mode VARCHAR(10) NOT NULL, -- update, delete, moved group
    created_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);
-- possibly groups_history, users_history

