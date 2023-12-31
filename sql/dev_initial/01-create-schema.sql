---- Base app schema

-- User
CREATE TABLE "user" (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(128) NOT NULL UNIQUE,
  pwd varchar(256),
  pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
  token_salt uuid NOT NULL DEFAULT gen_random_uuid(),

  cid BIGINT NOT NULL,
  ctime TIMESTAMP WITH time zone NOT NULL,
  mid BIGINT NOT NULL,
  mtime TIMESTAMP WITH time zone NOT NULL
);

-- Project
CREATE TABLE project (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  owner_id BIGINT NOT NULL,
  name varchar(256) NOT NULL,

  cid BIGINT NOT NULL,
  ctime TIMESTAMP WITH time zone NOT NULL,
  mid BIGINT NOT NULL,
  mtime TIMESTAMP WITH time zone NOT NULL
);

-- Task
CREATE TABLE task (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  project_id BIGINT NOT NULL,

  title varchar(256) NOT NULL,
  done bool NOT NULL DEFAULT false,

  cid BIGINT NOT NULL,
  ctime TIMESTAMP WITH time zone NOT NULL,
  mid BIGINT NOT NULL,
  mtime TIMESTAMP WITH time zone NOT NULL
);

ALTER TABLE task ADD CONSTRAINT fk_project
  FOREIGN KEY (project_id) REFERENCES project(id)
  ON DELETE CASCADE;