# Aleo MACI docs

This system is divided in 3 main components: a [client](../client), a [server](../server), and a [shared](../libs) library.

## Architecture Diagram

![maci_aleo_Architecture](aleo_maci_overview.png)

## Sequence Diagrams

You can find detailed sequence diagrams for every key operation [here](aleo_maci_sequence_diagram.pdf).

## Components Overview

### The Client

The [client](../client) is the program that run locally for each voter, it's responsible of:
- Ask the server to create an election
- Signing up an user to an election
- Sending user messages to the election
- Ask the server to start the tally
- Verifies that the users votes were properly counted after the tally ends.

### The Server

The [server](../server) is a rust written http server implemented with the [warp framework](https://github.com/seanmonstar/warp) and makes use of a [Rocksdb](https://github.com/facebook/rocksdb/) instance to store transactions_ids.

The server is responsible of:
- Creating and election
- Do the tally
- Storing every needed transaction_id to the db.

### Shared Library

The [shared library](../libs) is a rust lib that contains all the necessary code in order to make the client and server communication possible with ease.

It also contains common modules to interact with the blockchain from the client or the server, and shared circuits for basic operations, like storing data. 
