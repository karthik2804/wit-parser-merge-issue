# Wit-parser merge_worlds issue

This is a simple repro of not being able to merge two worlds that have the same export. The operation fails with the following error 

```bash
error: failed to merge worlds

Caused by:
    export `wasi:http/incoming-handler@0.2.0` conflicts with previous name of `wasi:http/incoming-handler@0.2.0`
```

the `wit` definition can be found at `wit/main.wit`,

The expected behavior would be for the worlds to merge successfully sine the exports are of the same type.

## Running the example 

```bash 
cargo run 
```