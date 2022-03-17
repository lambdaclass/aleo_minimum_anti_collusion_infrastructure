# Aleo Voting System

## How to use


### Alternative 1: Nix

To use the project you need to have [nix](https://nixos.org/download.html) installed.

Once installed on your system, run:

`nix-shell`

To start the server run in a second terminal:

`make run_server`

To use the cli run:

`./run_cli.sh`


### Alternative 2: Docker

You can also run the server with docker

`make build_images`

`make run_docker`

## Overview

Electronic voting system application on Aleo that implements MACI (Minimal anti-collusion infrastructure) with a simple CLI and a server.

To learn more about how it works, you can read the [docs](docs)

## The Problem

Electronic voting is a controversial topic that awakens different opinions regarding its implementation and vulnerability. While it is currently being used by several organizations and governments, those that criticize it claim that electronic voting is not secure since it compromises the anonymity of the voter and the safety of the election. 

If the information of each vote becomes public, or if the voter can prove he has voted for one party, it’s easy for the voters to collude. An actor that wants to manipulate the election, may offer bribes or find how to incentivize or penalize the voters in exchange for them acting as he wants.

For this reason, it’s of utter importance that there’s no easy mechanism to verify how someone has voted. And even more, the existence of mechanisms capable of nullifying and changing votes without anybody knowing, in order to render any kind of proof useless. 
 
We believe that an electronic voting system developed on Aleo, which uses zero knowledge cryptography to achieve both privacy and programmability, secures the voter’s anonymity and the election’s outcome.

## Our Approach

As it says on [aleo.org](https://aleo.org): “Zero knowledge cryptography and zero knowledge proofs allow third parties to verify the truth of a piece of information without us needing to reveal it directly”. This is the very essence of a secret ballot system and the backbone paradigm behind Aleo. Aleo provides both the privacy and programmability needed to develop a private but still dynamic application. With this in mind we will develop an electronic voting system application using Leo, Aleo’s programming language, and Rust when required. 

## Why privacy?

Privacy is the backbone of a secret ballot system and it is why Aleo is the right technology for this project. Ethereum provides limited privacy while Aleo excels at providing privacy for the user and in our use case, the voter.

## Why implement MACI specifically?

"Minimal Anti-Collusion Infrastructure (MACI) is a set of smart contracts and zero-knowledge circuits upon which developers can build collusion-resistant applications, such as voting systems or quadratic funding platforms. MACI grants resistance to collusion for decentralized applications and it is important because crypto communities are increasingly adopting Decentralised Autonomous Organisations (DAOs) which govern through token voting"[*](https://medium.com/privacy-scaling-explorations/release-announcement-maci-1-0-c032bddd2157). MACI has a use for several other applications such as gaming, gambling and finance.

# Aleo MACI technical docs

This system is divided in 3 main components: a [client](../client), a [server](../server), and a [shared](../libs) library.

## Architecture Diagram

![maci_aleo_Architecture](docs/aleo_maci_overview.png)

## Sequence Diagrams

You can find detailed sequence diagrams for every key operation [here](docs/aleo_maci_sequence_diagram.pdf).

## Components Overview

### The Client

The [client](client) is the program that run locally for each voter, it's responsible of:
- Ask the server to create an election
- Signing up an user to an election
- Sending user messages to the election
- Ask the server to start the tally
- Verifies that the users votes were properly counted after the tally ends.

### The Server

The [server](server) is a rust written http server implemented with the [warp framework](https://github.com/seanmonstar/warp) and makes use of a [Rocksdb](https://github.com/facebook/rocksdb/) instance to store transactions_ids.

The server is responsible of:
- Creating and election
- Do the tally
- Storing every needed transaction_id to the db.

### Shared Library

The [shared library](libs) is a rust lib that contains all the necessary code in order to make the client and server communication possible with ease.

It also contains common modules to interact with the blockchain from the client or the server, and shared circuits for basic operations, like storing data. 
