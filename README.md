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


### Administrators

Administrators are people with deep insight into the system. They need as much information as possible to track down and fix the error promptly.


### Machines

We can use errors for control flow too. Sometimes, operations fail but might succeed when retried at a later point in time. 

Machines should therefore only get information that influences the control flow. It is not relevant, why the error is unrecoverable since all unrecoverable errors should bubble up until some caller knows how to handle these errors. In this context, handling an error could mean notifying both the user and an administrator about the problem.

These errors are mainly used in libraries, while errors in applications are often targeted primarily to end-users and administrators.