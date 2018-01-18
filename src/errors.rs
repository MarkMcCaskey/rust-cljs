error_chain!{
    errors {
        EndOfInput {
            description("Reached the end of the input")
        }
        UnrecognizedToken(line_number: usize, column_number: usize) {
            description("We found a token we don't know how to handle"),
            display("Unrecognized token at line {}, column {}", line_number, column_number)
        }
    }
}
