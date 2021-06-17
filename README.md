# Subtitle time offset

Web backend API that Offsets SubRip (.srt) dialog timestamps.

What you are seeing is multiple solutions to the same backend. The point here is to compare performance and reliability between them.



## What functionalities I am supposed to find?

The project must be capable of receiving a SubRip (*.srt*) file, and then to return it with a given offset.

An extra is to have a upload history visualization route that allows downloading any previous file, with or without an offset. As of this commit, only `PythonSolution` provides this functionality.
