
# Rust error handling 

This is my attempt to build nice error type to use in library.

There are currently few ways of handling errors in Rust.

1. Using Box<dyn Error> - this has some drawbacks, like problems with send trait etc.
2. Using crate thiserror (to wrap easily multiple errors)
3. Using crate anyhow - this is powerful and easy to use crate that also enables backtraces etc. 
4. Native solution - like in this project.

My motivation is to have simple and easy to use pattern to have errors with
line info, and also multiple error types wrapped easly.

## Native solution

First create CustomError - you can add more custom errors to your library.
Look at custom.rs file.

Then ErrorBag that is enum collection of all errors that can occur in your library.

WrappedError enables use of macros to store where the error happen.




