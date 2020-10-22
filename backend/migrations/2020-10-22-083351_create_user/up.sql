CREATE TABLE users(
       id INTEGER PRIMARY KEY NOT NULL,
       username TEXT NOT NULL,
       password TEXT NOT NULL,
       apikey TEXT NOT NULL,
       permission TEXT CHECK(permission IN ('student', 'teacher', 'admin')) NOT NULL
)
