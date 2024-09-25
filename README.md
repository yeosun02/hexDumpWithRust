# Screening Test for Educational OS in Rust
##### Name: Jason Yoo
##### Student ID: 921476726
___
## Content
* Summary
* Implementation
* Testing
  
## Summary
It takes command line arguments, parses the arguments, sets the member 
variables, `length` and `filename`, of the struct `Config` accordingly, and 
prints out the given file as hex limited by the given `length`.

## Implementation
### 1. Config
Config has members `length` and `filename` which both have `String` type.

#### new
    The constructor of Config. It takes an array of `String` and returns `Result<Config, &str>`.
    It sets `length` to "not specified" as default value and checks inputs. If the inputs are 
    not in valid format return `Err` with the appropriate error message. Otherwise, it returns
    `Ok(Config {})` with the given `length` and `filename` from the argument.

### 2. hex_dump
hex_dump takes `Config` and returns `Result<(), Box<dyn Error>>` to handle
different types of errors. It reads the contents of the file given by the `filename`
in the given `Config`. Also, it sets the length to the size of the file if `length`
is "not specified" or `length` is bigger than the file size. Otherwise, it sets to 
the `length`. Then it starts printing out the file as hex.

## Testing
### Using #[cfg(test)]
    Tests the Config construction handling error cases

### Using shell script
    Tests various hex_dump calls using builtin hexdump and diff command

## References
    The Rust Reference: Basic rust
