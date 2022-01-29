-- Add migration script here
CREATE TABLE messages (
  name character varying (16) NOT NULL,
  message text NOT NULL,
  create_at timestamp with time zone NOT NULL
);

CREATE INDEX IF NOT EXISTS msg_time
  ON messages USING btree
  (create_at DESC);
