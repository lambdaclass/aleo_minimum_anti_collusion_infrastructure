# Aleo Voting System

## How to use

To use the project you need to have [nix](https://nixos.org/download.html) installed.

Once installed on your system, run:

`nix-shell`

To start the server run in a second terminal:

`make run_server`

To use the cli run:

`./run_cli.sh`

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



