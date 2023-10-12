# Week7-miniProj-zg105

This is a simple command-line tool for SQLite database operations. With it, you can:

- Create a user table
- Insert some sample users
- Delete specific users
- Display current users in the database

## Prerequisites

- Python 3.x
- `click` library
- `sqlite3` (comes with the standard Python library)

## Features

1. **Create a User Table**: If the table doesn't exist, the script will create a new user table in the database with fields for `id` and `username`.

2. **Insert Users**: The script will insert three users into the user table.

3. **Delete User**: The script will delete some users.

4. **Display Users**: The script will then display all current users in the user table.

## Demo
run python3 main.py zg105.db to start

![](pic1.png)

## User Guide
run python3 main.py --help to see a user guide

![](pic2.png)
