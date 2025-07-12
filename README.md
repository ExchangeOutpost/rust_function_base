# Rust Function Base for ExchangeOutpost

A baseline template for creating Rust-based financial functions that can be deployed and executed on [ExchangeOutpost](https://www.exchangeoutpost.com/).

## Overview

This project provides a foundational structure for developing financial analysis functions in Rust that compile to WebAssembly (WASM) and integrate seamlessly with the ExchangeOutpost platform. It includes pre-built data structures for handling financial market data, candlestick charts, and decimal precision calculations.

## Features

- **WebAssembly Compilation**: Functions compile to WASM for cross-platform execution
- **Financial Data Structures**: Built-in support for candlestick data, ticker information, and market data
- **Decimal Precision**: Integrated `rust_decimal` for accurate financial calculations
- **Extism Plugin Framework**: Uses Extism PDK for plugin development and execution
- **Type Safety**: Strongly typed financial data structures with serde serialization support

## Project Structure

```
src/
├── lib.rs                      # Main entry point and plugin function, edit this file to implement your function
└── exchange_outpost/           # Contains financial data structures and utility functions, you should not edit this directory
```

## Core Components

### FinData
The main data container that provides access to:
- **Ticker Data**: Market data for multiple trading symbols
- **Piped Data**: Additional data passed between functions
- **Call Arguments**: Runtime parameters for function execution

### Candle
Represents financial candlestick data with:
- Timestamp (Unix epoch)
- OHLCV data (Open, High, Low, Close, Volume)
- Support for both `f64` and `Decimal` precision

## Getting Started

### Prerequisites

- Rust 1.70+ with 2024 edition support
- `wasm32-unknown-unknown` target installed

### Installation

1. Clone this repository:
```bash
git clone https://github.com/ExchangeOutpost/rust_function_base.git
cd rust_function_base
```

2. Install the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Build the project:
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Development

### Creating Your Function

The main entry point is in `src/lib.rs`. Modify the `run` function to implement your financial analysis logic:

```rust
#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    // Access ticker data
    let labels = fin_data.get_labels();
    
    // Get candlestick data for a specific symbol
    let candles = fin_data.get_candles("BTCUSD")?;
    
    // Use decimal precision for calculations
    let decimal_candles = fin_data.get_candles_decimal("BTCUSD")?;
    
    // Your analysis logic here
    
    Ok(Output {
        // Return your results
    })
}
```

### Working with Financial Data

```rust
// Get available ticker symbols
let symbols = fin_data.get_labels();

// Access candlestick data
let candles = fin_data.get_candles("BTCUSD")?;
for candle in candles {
    println!("Close price: {}", candle.close);
}

// Use decimal precision for accurate calculations
let decimal_candles = fin_data.get_candles_decimal("BTCUSD")?;

// Access piped data from other functions
let pipe_sources = fin_data.get_pipe_sources();
let data = fin_data.get_data_from_pipe("previous_analysis")?;

// Get function call arguments
let args = fin_data.get_call_arguments();
```

### Output Structure

Define your output structure by modifying the `Output` struct:

```rust
#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    pub result: f64,
    pub signal: String,
    pub confidence: f64,
    // Add your fields here
}
```

## Building and Deployment

### Local Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be located at:
`target/wasm32-unknown-unknown/release/rust_function_base.wasm`

### Automated Releases

This project includes GitHub Actions for automated releases. When you push a tag, it will:
1. Build the WASM binary
2. Create a GitHub release
3. Upload the binary as `finfunc.wasm`

To create a release:
```bash
git tag v1.0.0
git push origin v1.0.0
```

### Testing Your Function
When pushing to the `master` branch, the CI will automatically build your function and create a preview release named `master`.
You can use this release to test your function on the ExchangeOutpost platform.

## Dependencies

- **extism-pdk** (1.4.0): Plugin development kit for WebAssembly functions
- **rust_decimal** (1.37.1): High-precision decimal arithmetic for financial calculations
- **serde** (1.0.219): Serialization/deserialization framework
- **serde_json** (1.0.140): JSON support for serde

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for more details.


## Related Links

- [ExchangeOutpost Platform](https://www.exchangeoutpost.com/)
- [Extism Documentation](https://extism.org/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
