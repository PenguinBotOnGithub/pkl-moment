-- Your SQL goes here
INSERT INTO
    "user" ("username", "password", "role")
VALUES
    (
        'admin',
        '$argon2id$v=19$m=19456,t=2,p=1$mKs0XLiszCvEFoQf4Yc9gQ$gycogumU7zrw/hPTTm1xdeQY4tYL1fWtuiTQgJLEnw0',
        'admin'
    );