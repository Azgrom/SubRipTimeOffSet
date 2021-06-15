## How it works?

This code provides a container that accepts uploads of a `.srt` file, store its content in a *sqlite3* database and allow the user to download a given subtitle, contained in the database, with a given offset.


### The main routes are:

- `localhost:<port_number>/docs/upload_to_db/`

  It provides a form to upload a SubRip file. In the background it parses and stores the file contents in a database. Each filename field is unique.

- `localhost:<port_number>/docs/list_files_in_db/`

  Allows the user to visualize the list of already uploaded files. It is possible to skip and to limit a given amount of items returned, in case the system has too many files, allowing pagination. The list contains only the file names.

- `localhost:<port_number>/docs/download_subtitle_by_id/{id}/`

  Allows the user to insert the item index of the `/list_files_in_db/` route and a fixed offset to displace all dialogs timings, in seconds.

On docker `port_number` is `80`, so all ports above become accessible via `localhost/docs`

## How to use it?

On terminal, run: `./build-docker.sh` to build a docker container image of this solution; `./run-docker.sh` to execute it and go to `localhost/docs` on your browser to use it.

