# Educate
Custom search engine built for academic research, educational content & learning, built with Rust, Python and React.

## Goal
The goal of this project is to built a search application which is primarily focused around educational resources. It is built around two primary features, a fully local search engine & a meta search engine. Results are aggregated from all of these sources, and final outputs are provided as links to these sources (browser re-direct) & summarisations (with a locally sourced LLM). 

## Requirements
I assume the user has the following installed. The project is not being externally hosted so treat it as if it was your own project still in development:

- Python >3.11 & listed dependencies (within root/src/scripts).
- Rust & Rust Toolchain
- Redis (https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/)
- Node.js 
- NPM
- Vite
- Terminal emulator (i.e. Kitty) (preferable)

## Usage
Recommended to use a multiplexed terminal or tabs in terminal emulators such as Kitty or Alacritty to run services in parallel given below.

### Search Services
Run search services, this is by running `cargo run main` within the 'src' directory from the root. Seed domains for local search are already linked via another repository publically available on GitHub. Explanations for how to setup necessities for meta search are given below.

### Frontend
Run development or production server for React front-end, this is by running `npm run dev` (for development server) in **searchinterface/src** from the root.

### Authentication & Configurations Service
Run Redis cache server, simple to call `redis_server` in a separate terminal instance. Ensure the default connection string is not modified (for the current version).

### Summarisation Service
Run summarisation service, simply access 'src/microservices' from the root, call `python/python3 search.py`. 

### Autosuggestions
Autosuggestions are supported by the use of a Trie (more detail at the bottom of this section), which relies on an existing sentence database. I recommend running the following command to obtain a required sentence database: `wget https://raw.githubusercontent.com/dwyl/english-words/master/words_dictionary.json`.  

### Google Engine Setup & Environment Variables
Environment variables can be used to modify Google programmable search engine key. This can be set up properly via: (). It is left open to the user to customise this, but it is recommended to select only 'edu' domains as allowable to fit the purpose of the application.

### DuckDuckGo CLI Setup
As part of the meta search service, the engine parses outputs from the DDGR command line tool. This can be installed with most default package managers, i.e. for Debian/Ubuntu sytems `sudo apt-get install ddgr` or through pip `sudo pip3 install ddgr`.

### The Application
Once all the above is complete, you can run the default IP address (i.e. using development server). This will bring up a search page (image below). 

![Alt text](SearchHomePage.png)


#### Home Page & Account Setup
Enter into the main search input in the center of the screen. Autosuggestions (single-word and full-sentence) will appear below, select as needed. In the top right includes configurations, search history & profile (a sign-up or sign-in page). The text beneath the profile will displayed as 'Login or Register' when the user is not currently signed in. Search results will appear after typing in the input after some time (approx. 10 seconds although more accurate details are given per search method in configurations) in a list. Summaries are awaited on until completed by the 'TinyGPT' microservice.

#### Search History
Search history can be filtered and deleted as the user requires, which will be automatically dropped from the Redis cache. Of course this is not externally hosted so the user need not worry about access or security issues unless they host their own instance. 

#### Configurations
Configurations include 'index type', 'search method', 'engines'. Index type can specify document-term, which will allow you to use its associated search methods. It can also specify inverted index, and its associated search methods. The methods associated with document-term are slightly more accurate but slower to compute (without GPU acceleration) in contrast to inverted index methods, which can take as little as 15s to rank 1000's of web documents. The engines selection allows you to filter which engines you would like to include in 'meta-search', in this case pulling from your personalised Google search engine (setup as above) and parsed results from the DuckDuckGo search CLI, this will be prioritised over local results.

I'm hoping to write a wrapper gateway at some point which deals with managing the lifecycle of the system and ensuring everything runs without as much user input.

## System In-Depth
More detail about how the project works internally can be found in the following article: (). If you enjoy the article and the rest of the page, it would be much appreciated if you subscribe (it is completely free and I don't intend to change this into the near future). 
