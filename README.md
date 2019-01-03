# rust_url_builder

An easy-to-use crate to construct URLs for the Rust Programming language

You can use this to build up context for a url over the course of execution and then
call the `.build()` method to generate the final url.

The mutating functions allow you to chain them to each other.

## Example

The following code will create a url similar to `http://localhost:8000?first=1&second=2&third=3`
The order of the query parameters is indeterminate as the parameters are internally stored in 
`std::collections::HashMap`.

```
let mut ub = URLBuilder::new();

ub.set_protocol("http")
    .set_host("localhost")
    .set_port(8000)
    .add_param("first", "1")
    .add_param("second", "2")
    .add_param("third", "3");

println!("{}", ub.build()); 
```
