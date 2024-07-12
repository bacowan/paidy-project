# Restaraunt API
## Running
- To start up the server, cd to the `server` directory and run `cargo run`.
- To run the simulation, cd to the `client` directory in a different terminal and run `cargo run`.
- To manually make requests to the server, using an existing client application such as Postman or curl is recommended. The format of the requests can be found in `openapi.yaml`. For a visual representation, go to https://editor.swagger.io/, select File -> Import URL and paste in "https://raw.githubusercontent.com/bacowan/paidy-project/main/openapi.yaml".

The server will run on `http://127.0.0.1:8000`.

### Example requests
#### New Orders
This creates 2 new orders for table 1 and the menu items with ids 1 and 2. It will return the created orders.
```http
POST /tables/1/orders HTTP/1.1
Host: http://127.0.0.1:8000
Content-Type: application/json

{
  "orders": [
    {
      "menu_item_id": 1
    },
    {
      "menu_item_id": 2
    }
  ]
}

```
#### Query All Orders for a Table
This gets all orders that have been made (and not yet deleted) for table 1.
```http
GET /tables/1/orders HTTP/1.1
Host: http://127.0.0.1:8000
```
#### Query Single Order for a Table
This gets a single order which has been added to a table by the order's ID (which can be retrieved by querying all orders for the table and is returned when the order is added).
```http
GET /tables/1/orders/1 HTTP/1.1
Host: http://127.0.0.1:8000
```
#### Delete an Order
This deletes an order from a given table with a given order ID.
```http
DELETE /tables/1/orders/1 HTTP/1.1
Host: http://127.0.0.1:8000
```
### Tests
- To run the client tests, cd to `client/tests` and run `cargo test`
- To run the server tests, cd to `server/tests` and run `cargo test`
## Design
### Assumptions Made
The assignment indicated that I should use my own judgement when any ambiguity is encountered in the instructions. The following are said assumptions. Please note that, while this was intended to be production ready, with a real product I would ask the client for clarification whenever ambiguity arises rather than making assumptions like I did here.
- The term "item" was used rather liberally in the instructions, and it was not always clear if it meant "menu item" or "order". I assumed that the intent was "order".
- Little guidance was given on client design. While I thought it would be useful to make a CLI or similar in order to manually run the clients, the wording make it sound more like an automatically run "simulation". I split the code such that creating a CLI would not be difficult, but for the sake of scope creep I refrained from doing so. Manual testing of the REST API can be done simply enough with tools like postman or curl.
### Rest API
The design of the Rest API can be seen in openapi.yaml, and can be viewed through https://editor.swagger.io/ by selecting File -> Import URL and pasting in "https://raw.githubusercontent.com/bacowan/paidy-project/main/openapi.yaml". A summary is as follows:
- The /menu-items GET endpoint is used to get all menu items with their names and ids. While this was not a requirement of the project, it is important to allow the client to be able to see menu item names and their associated IDs so that they can be added to orders.
- The `/tables/{table-number}/orders GET` endpoint lists all orders for a single table
- The `/tables/{table-number}/orders/{order-id} GET` endpoint gets a single order for a single table
- The `/tables/{table-number}/orders POST` enpoint allows for one or more orders to be added
- The `/tables/{table-number}/orders/{order-id} DELETE` endpoint deletes the given order from the table
### Database structure
The database is composed of 3 tables: menu_items, orders, and idempotent_requests. There is no table for "tables": table numbers are simply a property of orders.
- menu_items contains all items that can be ordered, and defaults are added on creation.
  - There is an autoincrementing ID column and a name column
- orders contains all orders that have been placed until they are deleted.
  - Each order has a unique autoincrementing ID, the ID of the menu item that was ordered, a table number, and minutes to cook.
- idempotent_requests lists unique POST requests that have been made.
  - The idempotency key is added to ensure that duplicate requests are not made. Since there is nothing unique about orders when they are sent (one table could order two hamburgers, for example), the client may provide a unique idempotency key when creating orders; if the request is sent twice for some reason (for example, if the client disconnects and makes the request a second time to ensure that it went through), the order will only be added once.
  - A potential new feature would be to have idempotentcy keys expire: a timestamp column could be added, and a cron job could periodically delete items which are older than a day, for example. This would need to be clearly documented for uses to know.
## Code Structure
### Server
- main.rs: this is the entry point for the server. The main function initializes the database, sets up the REST endpoints, and starts the server.
- endpoints.rs: this defines the behaviour of the individual endpoints. It is responsible for deserializing requests and serializing output, including outputting the correct error codes.
- server_functions.rs: this contains all of the database interaction logic.
  - The database engine used is Sqlite. Each call opens its own connection which is automatically closed once it goes out of scope.
  - Each function takes a `DatabaseConnector` as one of its input parameters. This allows for dependency injection during testing. The default implementation opens a database with a path defined on creation. The one used in testing opens a temporary file as managed by the operating system (and will thus have no conflicts with other tests and will be automatically cleaned up).
  - The orders table in the database handles idempotency: if two of the same order are sent, there will be a conflict and the second item will not be added
- Other files contain minor code, such as structs used elsewhere
### Client
- main.rs: this is the entry point for the client. 30 different threads representing "tablets" are spawned for 1 minute.
- sim.rs: this contains code for individual "tablets". Each "tablet" makes a series of time delayed random calls to the server. Each "tablet" keeps track of all orders that it has added, and that list is used when querying individual items and deleting items from the server. Other than tracking those orders, the bulk of the code in this file is dedicated to formatting output strings and calling code in the client_functions.rs file.
  - Functions in this file take a `SimInjectionParams` object for the sake of dependency injection. This object contains a `ClientFunctionInterface`, which wraps the functions in `client_functions.rs` so that they can be mocked in tests, and a `StdRng`, which allows tests to seed their RNG for the sake of having consistent tests.
- client_functions.rs: this contains functions for sending data to the server. If another client implementation were to bemade, such as a CLI, it could call the code in here.
  - Each function takes a `WebConnection` object, which is used for dependency injection in unit testing. The default implementation sends a real request to the server, and the mock implementation allows for responses to be mocked and keeps track of some information about what requests are made.
  - Since the client is responsible for creating the idempotency key, a UUID is created for each POST request. There is an option to retry the request on timeout; in this case, the same idempotency key is used. The simulation simply skips the request in this case, but its functionality is tested so it could be used in another implementation.
- Other files contain minor code, such as structs used elsewhere
