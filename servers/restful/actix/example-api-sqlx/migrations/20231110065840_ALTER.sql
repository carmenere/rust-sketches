-- Add migration script here
ALTER TABLE persons ADD CONSTRAINT persons_unique_name UNIQUE (name, surname); 