# webserver

Follow the example from [Section 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) of the official rust lang book.

This chapter leads you through making a very rudimentary webserver. The server is very, very simple but it is interesting to see how simple the HTTP protocol is.

The webserver is modified to have a thread-pool to allow it to handle simultaneous requests without crashing the host computer. This is still relatively simple, but it is a very educational and interesting example.
