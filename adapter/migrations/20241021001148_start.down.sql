-- Add down migration script here
DROP TRIGGER IF EXISTS books_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

DROP TRIGGER IF EXISTS users_update_at_trigger ON users;
DROP TABLE IF EXISTS usres;
DROP TABLE IF EXISTS roles;

DROP FUNCTION set_updated_at;

DROP TABLE IF EXISTS returned_checkouts;
DROP TABLE IF EXISTS checkouts;