Description:

This exercise emphasizes proper error propagation, eliminating the use of unwraps and expects, and utilizing Option and Result types for comprehensive error representation.

    Refactor main() Function:
        Restructure the main() function to solely examine the first argument to identify the required operation, and execute its function.
        Display either the operation's output or an error if the output is invalid.

    Function creation for Operations:
        For each operation from the previous assignment, create a dedicated function.
        These functions should validate arguments, parse, and subsequently return the output as a String.
        Return Result<String, Box<dyn Error>> from each function. This facilitates the conversion of a variety of error types using the ? operator. You will need to import std::errror::Error to be able to use this.
        Use the format!() macro to construct strings, mirroring the use of println!().

    Error Handling in main():
        Present the selected operation and any errors encountered. Print both to stderr via the eprintln!() macro.
        Successful operation outputs should be relayed to stdout (println!()).

    Implement the CSV Operation:
        Incorporate an additional operation labeled csv.
        This operation should interpret the input string as CSV (reading the entire input, not merely one line), treating the inaugural row as headers.
        Exhibit the parsed content in an orderly table layout.
        For ease, you can assume that neither header names nor values will span over 16 characters. There is a bonus point in it for you, if you can handle any length of values and headers.
        If you want, you can create a Csv struct, which will store the headers and values, and implement the Display trait on per standard library documentation (https://doc.rust-lang.org/std/fmt/trait.Display.html#examples). This will make your final csv() function much cleaner. Remember that everything that implements Display gets a .to_string() method for free.
        You can opt to manually parse the CSV or employ the csv crate. Feel free to explore and test other crates that might be beneficial.
        Your application should ideally remain stable and not panic, even when fed with nonsensical input.
