# Jonah: The Prophet Named "Dove" 🕊️

**Forecasting the currents of the market with the wisdom of Prophet, wrapped in the speed of Rust.**

## Overview

Jonah is a high-performance financial forecasting tool designed to predict Open, High, Low, Close, and Volume (OHLCV) stock data. The project is named after the biblical prophet Jonah, whose Hebrew name (יוֹנָה - Yonah) translates to "dove." In many traditions, the dove is a symbol of peace, hope, and the bearer of good news or significant messages—much like the dove that returned to Noah with an olive branch, signaling new beginnings. Similarly, this project, "Jonah," aims to act as a messenger, bringing clarity, foresight, and a sense of guidance to the often turbulent and unpredictable seas of the stock market through its forecasting capabilities.

This project leverages the powerful forecasting capabilities of Facebook's Prophet library, traditionally used in Python, by creating a Rust-based application that interfaces with Prophet via the `pyo3` crate. This approach combines the ease and robustness of Prophet's forecasting algorithms with the performance, safety, and concurrency features of Rust.

## Key Features

* **OHLCV Forecasting:** Specifically tailored for predicting all key aspects of stock price movements.
* **Prophet Integration:** Utilizes Facebook Prophet for robust time series forecasting.
* **Rust Core:** Built with Rust for optimal performance, memory safety, and concurrency.
* **Python Interoperability:** Seamlessly calls Python Prophet functions using `pyo3`.
* **Extensible Design:** Planned to be modular and extensible for future enhancements.

## Technical Stack

* **Primary Language:** [Rust](https://www.rust-lang.org/)
* **Python Bridge:** [PyO3](https://pyo3.rs/)
* **Forecasting Engine:** [Prophet (via Python)](https://facebook.github.io/prophet/)
* **Data Handling:** (To be determined - e.g., Polars, ndarray)

## Project Status

🚧 **Work in Progress / Planned** 🚧

Jonah is currently in the planning and early development stages. The core architecture and integration strategy are being defined.

## How It Works

Jonah will operate by:

1.  **Data Ingestion (Rust):** Loading and preprocessing OHLCV stock data.
2.  **Python Call (Rust via `pyo3`):** Preparing the data and invoking the Prophet forecasting model within an embedded Python interpreter or by calling a Python script.
3.  **Forecasting (Python/Prophet):** Prophet performs the time series analysis and generates predictions.
4.  **Result Handling (Rust):** Receiving the forecast results back from Python and processing/presenting them.

The `pyo3` crate is crucial for enabling this communication, allowing Rust to call Python functions and exchange data structures efficiently.

## Getting Started

*(This section will be updated once the project has a runnable version.)*

Prerequisites:
* Rust toolchain
* Python environment
* Prophet library installed in the Python environment

Build & Run:
```bash
# Clone the repository (once available)
# git clone <repository-url>
# cd jonah

# Build the project
# cargo build

# Run the application
# cargo run -- <args>
Future GoalsIntegration with various data sources (APIs, local files).Batch processing for multiple stock symbols.Backtesting capabilities.API for programmatic access to forecasts.Potential for a simple UI or visualization output.ContributingContributions are welcome! If you're interested in helping with Jonah, please feel free to fork the repository, make your changes, and submit a pull request. You can also open an issue to discuss potential features or report bugs.(Contribution guidelines will be detailed further as the project matures.)LicenseThis project is planned to be licensed under the MIT License. *(A `LICENSE
