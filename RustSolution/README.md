## How it works?

This code provides a container that accepts uploads of a `.srt` file, store its content in the OS temporary directory and allow the user to download it with a given offset..


### The main routes are:

- `localhost:<port_number>/file`

  It provides a form to upload a SubRip file. In the background it parses and stores the file contents in a database. Each filename field is unique.

- `localhost:<port_number>/file/offset/<offset_in_seconds>`

  Allows the user to insert a fixed offset to displace all dialogs timings, in seconds. After this route is consumed, the temporary file is deleted.


By default `port_number` is `1111`, so all ports above become accessible via `localhost:1111`

## How to use it?

On terminal, run: `./build-docker.sh` to build a docker container image of this solution; `./run-docker.sh` to execute it and go to routes with your favorite API client.

## Functionalities

- [x] SubRip file upload;
- [ ] Save to DataBase;
  - [ ] Allow Database list visualization;
- [x] Allow file download with fixed timestamp offset;
  - [ ] Allow download based on Database list index;

