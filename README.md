# Lift

A simple concurrent program that imitate an elevateor system.

## Description

The program is composed of two elevators that communicate with each other to efficiently transport people from one floor to another. It is written in Rust and utilizes the concurrent functionalities of the language. The elevators can move up and down, stop at different floors, and pick up/drop off passengers. The program employs a simple algorithm to determine the next floor to visit.
Each elevator is a thread and they both communicate with the mscp system to get the next floor to visit.

## How to run

```bash
$ cargo run
$ cargo -- -h
```
