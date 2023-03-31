# Conventions 

### Structure Conventions
* Types used in [reqwest](https://crates.io/crates/reqwest) requests get moved into a child module named `reqwest_types`.
* As sometimes a request needs to repeat due to an invalid x-csrf-token, a "trampolining" approach is used as `async_recursion`
complicates the type signature. Due to this, all functions that require an x-csrf-token have their public method in an `external` module. This module can be, but does not need to be, in a separate file. The public method gets called, which calls `xxx_internal()` (xxx being the public method name)
up to two times.
    - (e.g., `external::item_details` is the public version of `item_details_internal`)


### Naming Conventions
* Structs used as a generic when calling [.json()](https://docs.rs/reqwest/latest/reqwest/struct.Response.html#method.json) are suffixed with   `-Response`.
    - (e.g., `ItemDetailsResponse`)
* Structs/Enums used inside structs suffixed with `-Response` are suffixed with `-Raw`.
    - (e.g., `ItemDetailsRaw`)
* Structs used as a body to a [reqwest](https://crates.io/crates/reqwest) request are suffixed with `-ReqBody`.
    - (e.g., `ItemDetailsReqBody`)
* Structs/Enums used inside structs sufficed with `-ReqBody` are suffixed with `-Req`.
    - (e.g., `ItemParametersReq`)