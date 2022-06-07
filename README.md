# Rust Error Handling

This repository functions as a playground for Rust error handling. I would like to test out how errors can be handled elegantly within the code and displayed correctly to different audiences.

Error and exceptions can serve multiple purposes. Depending on the audience, different information has to be printed.

## Audiences

### End-user

End users are the normal users of your system. They do not have a deep insight into your system. Therefore, they should not be bothered by the exact cause of the error. No stack trace or system state should be exposed to them, also to protect your code.

They only have to know, how they should react to get back to a functioning system. Either

- they have used the system wrong, e.g., by inputting invalid information or by misconfiguring the system
- they should try at a later time
- there is an unrecoverable error and an administrator has to be contacted

The standard library already provides the `Display` trait which signalizes, that an object can be printed in a friendly way. In the context of error handling, we use this trait to display errors to the user.


### Administrators

Administrators are people with deep insight into the system. They need as much information as possible to track down and fix the error promptly. An administrator will benefit from further information in addition to a plain error message. Error chains and stack traces are such information.

Furthermore, we will use the `Debug` trait from the standard library to note that an error can be printed in a more informative and still human-friendly way for an administrator.


### Machines

We can use errors for control flow too. Sometimes, operations fail but might succeed when retried at a later point in time. 

Machines should therefore only get information that influences the control flow. It is not relevant, why the error is unrecoverable since all unrecoverable errors should bubble up until some caller knows how to handle these errors. In this context, handling an error could mean notifying both the user and an administrator about the problem.

These errors are mainly used in libraries, while errors in applications are often targeted primarily to end-users and administrators.


# Crates

While the native error handling in Rust is advanced, it can be cumbersome to write all the necessary boilerplate. Fortunately, many users and the [Rust Error Handling Group](https://github.com/rust-lang/project-error-handling) have created crates that make handling errors more compact and ergonomic.

- [thiserror](https://docs.rs/thiserror/latest/thiserror/): Derive macro for the `std::error::Error` trait
- [snafu](https://docs.rs/snafu/latest/snafu/): Similar to `thiserror` but with the ability to add context. 
- [anyhow](https://docs.rs/anyhow/latest/anyhow/): Opaque error type
- [fehler](https://docs.rs/fehler/latest/fehler/): Macro for `throw`ing errors like in python or java

Over time, there were multiple attempts to improve upon the native error handling. Those attempts are mostly deprecated in favor of `thiserror` and `anyhow`.

- [failure](https://docs.rs/failure/0.1.8/failure/)
- [error-chain](https://docs.rs/error-chain/0.12.4/error_chain/)