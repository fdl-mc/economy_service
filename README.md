# Economy API Service
A main virtual currency payments gateway.


## Deploying guide

### Via Docker
1. Install the latest Docker version
2. Clone this repository
3. Run `make build` (or `docker build -t fdl-mc/api/economy .`) in the project root directory to build an image
4. Run `make build` and then `make run` (or `make deploy` if you wanna run it as a daemon)

### Manually
1. Install the latest Rust version
2. Clone this repository
3. Run `cargo build --release` in the project root directory
4. Run the executable in `./target/release/Economy_service`


## Environment variables
| Variable          | Purpose                                           |
|-------------------|---------------------------------------------------|
| USERS_SERVICE_URL | URL to the users service                          |
| DATABASE_URL      | Database URL (supports Postgres, MySQL or SQLite) |


## License
The project is licensend under [GNU General Public License v3.0](https://github.com/fdl-mc/economy_service/blob/main/LICENSE)