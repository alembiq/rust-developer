Description:

You'll be tasked with implementing multi-threading in your Rust application. This will enhance the efficiency of your program by dividing tasks among separate threads.

    Set up Concurrency:
        Spin up two threads: one dedicated to receiving input and another for processing it.
        Make use of channels to transfer data between the two threads. You can employ Rust's native std::mpsc::channel or explore the flume library for this.

    Input-Receiving Thread:
        This thread should continuously read from stdin and parse the received input in the format <command> <input>. Remember to avoid "stringly-typed APIs" - the command can be an enum. For an enum, you can implement the FromStr trait for idiomatic parsing.

    Processing Thread:
        Analyze the command received from the input thread and execute the appropriate operation.
        If successful, print the output to stdout. If there's an error, print it to stderr.

    CSV File Reading:
        Instead of reading CSV from stdin, now adapt your application to read from a file using the read_to_string() function. Make sure you handle any potential errors gracefully.

    Bonus Challenge - Oneshot Functionality:
        If you're looking for an additional challenge, implement a mechanism where the program enters the interactive mode only when there are no CLI arguments provided. If arguments are given, the program should operate in the previous way, processing the command directly.
