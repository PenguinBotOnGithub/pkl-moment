-- Your SQL goes here
CREATE TYPE "operation" AS ENUM (
    'create', 
    'update', 
    'delete', 
    'verify', 
    'unverify',
    'register',
    'upload'
);