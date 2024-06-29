# Educate
Custom search engine built for academic research, educational content & learning, built with Rust and React.

## Goal
The goal of this project is to built a search application which is primarily focused around educational resources, learning materials & informative ( as well as legitimately sourced ) content. It is built around two primary features, a fully local search engine & a meta search engine. Results are aggregated from all of these sources, and final outputs are provided as links to these sources ( browser re-direct ) & summarisations ( with a locally sourced LLM ). 

## Requirements


## Usage

These are quick notes. Full documentation to be provided at a later date.

The application uses a Redis instance to store authentication details, manage users & search histories locally. This process is not managed, so users should ensure Redis is installed on their system, and use the command 'redis-server' to spawn a Redis instance. It is also up to the user how disk persistence is managed, and if they want to store user details at all (it is possible, as per the original documentation, to implement a non-graceful shutdown, clearing session data from disk). 
