//! Stub library for `libomnikit.so` that exits the process on call.

#![expect(unsafe_code, reason = "defines C symbols to mock `libomnikit.so`")]

fn called(symbol: &str) -> ! {
    // [tag:api_stub_called]
    println!("stub function {symbol} was called");

    // Since we do not provide correct implementations for the plugin API
    // functions, it is impossible to return to the caller safely.
    std::process::exit(0);
}

include!(concat!(env!("OUT_DIR"), "/stubs.rs"));
