-- Add up migration script here
CREATE TABLE "user" (
  id Uuid PRIMARY KEY,
  clerkId Text UNIQUE NOT NULL
);
