> ⚠️ Moved to https://codeberg.org/FDL/economy_service

# Economy API Service
A main virtual currency payments gateway.


## Deploying guide

### Via Docker
1. Install the latest Docker version
2. Clone this repository
3. Run `docker build -t economy_service .` in the project root directory to build an image
4. Run the image with `docker run economy_service`

### Manually
1. Install the latest Rust version
2. Clone this repository
3. Run `cargo build --release --package economy_service` in the project root directory
4. Run the executable in `./target/release/economy_service`


## Environment variables
| Variable          | Purpose                  |
|-------------------|--------------------------|
| USERS_SERVICE_URL | Users service URL        |
| DATABASE_URL      | postgres:// database URL |

Note that the docker-compose.yml in this repo uses USERS_SERVICE_URL and POSTGRES_PASSWORD environment variables.


## License
The project is licensend under [GNU General Public License v3.0](https://github.com/fdl-mc/economy_service/blob/main/LICENSE)