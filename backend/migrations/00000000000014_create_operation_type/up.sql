-- Your SQL goes here
CREATE TYPE "operation" AS ENUM (
    'create', 
    'update', 
    'delete', 
    'verify', 
    'register',
    'upload'
);