mod constants;
mod definitions;
use constants::CURRENCIES;
use definitions::Currency;

pub fn exchange(from_currency: &str, to_currency: &str, amount: f64) -> f64 {
    let from_currency = code_currency(from_currency).unwrap();
    let to_currency = code_currency(to_currency).unwrap();
    from_currency.exchange_to(amount, &to_currency)
}

pub fn code_currency(code: &str) -> Option<Currency> {
    for currency in CURRENCIES {
        if currency.code == code {
            return Some(currency.clone());
        }
    }
    None
}

pub fn name_currency(name: &str) -> Option<Currency> {
    for currency in CURRENCIES {
        if currency.name == name {
            return Some(currency.clone());
        }
    }
    None
}

pub fn symbol_currency(symbol: &str) -> Option<Currency> {
    for currency in CURRENCIES {
        if currency.symbol == symbol {
            return Some(currency.clone());
        }
    }
    None
}

pub fn numeric_code_currency(numeric_code: &str) -> Option<Currency> {
    for currency in CURRENCIES {
        if currency.numeric_code == numeric_code {
            return Some(currency.clone());
        }
    }
    None
}

pub fn withdrawal_date_currency(withdrawal_date: &str) -> Option<Currency> {
    for currency in CURRENCIES {
        if currency.withdrawal_date == withdrawal_date {
            return Some(currency.clone());
        }
    }
    None
}

pub fn active_currencies() -> Vec<Currency> {
    let mut active_currencies: Vec<Currency> = Vec::new();
    for currency in CURRENCIES {
        if currency.withdrawal_date == "" {
            active_currencies.push(currency.clone());
        }
    }
    active_currencies
}

pub fn withdrawn_currencies() -> Vec<Currency> {
    let mut withdrawn_currencies: Vec<Currency> = Vec::new();
    for currency in CURRENCIES {
        if currency.withdrawal_date != "" {
            withdrawn_currencies.push(currency.clone());
        }
    }
    withdrawn_currencies
}
