# Restaraunt API
## Running
### Simulation
1. cd to server and run `cargo run` in order to start up the server.
2. In a separate terminal, cd to client and run `cargo run`. This will run a simulation of the client.
### Tests
- To run the client tests, cd to client/tests and run `cargo test`
- To run the server tests, cd to server/tests and run `cargo test`
## Design
### Rest API
The design of the Rest API can be seen in openapi.yaml.
### Database structure
The database is composed of 2 tables: menu_items and orders. There is no table for "tables"; table numbers are simply a property of orders.
- menu_items contains all items that can be ordered, and defaults are added on creation.
  - There is an autoincrementing ID column and a name column
- orders contains all orders that have been placed.
  - Each order has a unique ID, the menu item that was ordered, a table number, minutes to cook, and a unique idempotency key.
  - The idempotency key is added to ensure that duplicate orders are not placed. Since there is nothing unique about orders when they are sent (one table could order two hamburgers, for example), the client provides a unique idempotency key when creating an order; if the order is sent twice for some reason, the order will only by added once.
## Code Structure
### Server
- main.rs: this is the entry point for the server. The main function sets up the REST endpoints and starts the server
- endpoints.rs: this defines the behaviour of the individual endpoints. It is responsible for deserializing requests and serializing output, including outputting the correct error codes.
- server_functions.rs: this contains all of the database interaction logic.
  - The database engine used is Sqlite. Each call opens its own connection which is automatically closed once it goes out of scope.
  - Each function takes a `DatabaseConnector` as one of its input parameters. This allows for dependency injection during testing. The default implementation opens a database with a path defined on creation. The one used in testing opens a temporary file as managed by the operating system (and will thus have no conflicts with other tests and will be automatically cleaned up).
  - The orders table in the database handles idempotency: if two of the same order are sent, there will be a conflict and the second item will not be added
- Other files contain minor code, such as structs used elsewhere
### Client
- main.rs: this is the entry point for the client. The 30 different threads are spawned for 1 minute. Each thead acts as a single "waiter" at the restaraunt, making random calls to the server. Each waiter keeps track of all orders that it has added, and that list is used when querying individual items from the server. Note that a querried order may have been deleted by another waiter, in which case a 404 error will be returned and the process will continue.
- client_functions.rs: this contains functions for sending data to the server. Each function takes a WebConnection object, which is used for dependency injection in unit testing. The default implementation sends a real request to the server, and the mock implementation allows for responses to be mocked and keeps track of some information about what requests are made.
  - Since the client is responsible for creating the idempotency key, a UUID is created for each order on POST requests. There is an option to retry the request on timeout; in this case, the same idempotency key is used.
