# rust-fullstack-example

Following [this tutorial](https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/).

## Setup

Clone the repo:  

    git clone https://github.com/jwolff52/rust-fullstack-example

Install postgresql:
```
$ sudo apt-get update && sudo apt-get install postgresql
```

Edit `/etc/postgres/15/main/postgresql.conf` to bind to port `7878`:
```
# snip

port=7878

# snip
```

Create a user called `dev`:
```
sudo -u postgres createuser dev
```

Set Password and Permissions for `dev` user:
```
$ su postgres
postgres $ psql
postgres=# ALTER USER dev WITH ENCRYPTED PASSWORD '1234';
postgres=# GRANT ALL PRIVILEGES ON DATABASE 'postgres' TO dev;
postgres=# GRANT ALL ON SCHEMA public TO dev;
```

Make sure the rust environment is set up to target wasm:
```
$ rustup target add wasm32-unknown-unknown
```

Install trunk with Cargo:
```
$ cargo install --locked trunk
```

## Run
Build and Run the Backend, this serves the backend api at `http://localhost:8000`:
```
cargo run
```

Build the frontend:
```
trunk build frontend/index.html
```

Serve the frontend at `http://localhost:8080`:
```
trunk serve frontend/index.html
```
