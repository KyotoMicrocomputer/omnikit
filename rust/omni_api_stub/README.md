# API stub

A stub library that exports all C functions referenced by the plugin API GIR metadata.

> [!WARNING]
> This is a testing tool for OmniKit itself and not intended to be used by plugin authors.

<!-- [ref:api_stub_called] -->
This library provides stub functions that print the called function's symbol name to stdout and exit the process with zero exit status.

It can be used to test the loading process of a plugin without requiring an official build of `libomnikit.so`.
Since the stub functions do not implement the normal behavior, it cannot be used for functional testing.
