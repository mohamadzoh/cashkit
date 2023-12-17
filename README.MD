
# CashKit

CashKit is a Rust library for currency exchange and currency information. It allows you to perform currency conversion and retrieve details about various currencies.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cashkit = "0.1.0"
```

## Currency Struct

```use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    pub name: &'static str,
    pub code: &'static str,
    pub symbol: &'static str,
    pub exponent: u8,
    pub numeric_code: &'static str,
    pub withdrawal_date: &'static str,
    pub next_update_date: &'static str,
    pub rates: Option<HashMap<String, f32>>,
}
```

## Usage Example

```extern crate cashkit;

use cashkit::{code_currency, exchange};

fn main() {
    let from_currency = "USD";
    let to_currency = "EUR";
    let amount = 100.0;

    let result = exchange(from_currency, to_currency, amount);

    println!("{} {} is {} {}", amount, from_currency, result, to_currency);
}
```

## Currency Information

```extern crate cashkit;

use cashkit::{code_currency, name_currency, symbol_currency, numeric_code_currency};

fn main() {
    let currency_code = "USD";
    let currency_name = "US Dollar";
    let currency_symbol = "$";
    let numeric_code = "840";

    let by_code = code_currency(currency_code);
    let by_name = name_currency(currency_name);
    let by_symbol = symbol_currency(currency_symbol);
    let by_numeric_code = numeric_code_currency(numeric_code);
}
```

## Currency Lists

```extern crate cashkit;

use cashkit::{active_currencies, withdrawn_currencies};

fn main() {
    let active_currencies_list = active_currencies();
    let withdrawn_currencies_list = withdrawn_currencies();

   ... (Handle Vec<Currency> as needed)
}
```


## Exchange Rate Provider
CashKit Depend while providing exchange rate on  the free api of "open.er-api.com".
Please note that we are not responsible for any inaccuracies or discrepancies in the provided data.

## Rusty Rails Project

  

Rusty Rails is a larger project aiming to bridge the gap between Rust and Ruby/Ruby on Rails. We are actively working on recreating ruby library into rust that seamlessly make working in rust more easy and fun for new developers.
## Contributing

Contributions to the CashKit library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.
