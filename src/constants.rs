use crate::definitions;
use definitions::Currency;

pub const PROVIDER_IP: &str = "open.er-api.com";
pub const PROVIDER_BASE_URL: &str = "/v6/latest/";

pub const CURRENCIES: [Currency; 147] = [
    Currency {
        name: "Afghani",
        code: "AFN",
        numeric_code: "971",
        exponent: 2,
        withdrawal_date: "",
        symbol: "AFN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lek",
        code: "ALL",
        numeric_code: "008",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ALL",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Algerian Dinar",
        code: "DZD",
        numeric_code: "012",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "US Dollar",
        code: "USD",
        numeric_code: "840",
        exponent: 2,
        withdrawal_date: "",
        symbol: "USD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kwanza",
        code: "AOA",
        numeric_code: "973",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "East Caribbean Dollar",
        code: "XCD",
        numeric_code: "951",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "No universal ",
        code: "",
        numeric_code: "",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Argentine Peso",
        code: "ARS",
        numeric_code: "032",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ARS",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Armenian Dram",
        code: "AMD",
        numeric_code: "051",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Aruban Florin",
        code: "AWG",
        numeric_code: "533",
        exponent: 2,
        withdrawal_date: "",
        symbol: "AWG",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Australian Dollar",
        code: "AUD",
        numeric_code: "036",
        exponent: 2,
        withdrawal_date: "",
        symbol: "AUD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Azerbaijan Manat",
        code: "AZN",
        numeric_code: "944",
        exponent: 2,
        withdrawal_date: "",
        symbol: "AZN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Bahamian Dollar",
        code: "BSD",
        numeric_code: "044",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BSD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Bahraini Dinar",
        code: "BHD",
        numeric_code: "048",
        exponent: 3,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Taka",
        code: "BDT",
        numeric_code: "050",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Barbados Dollar",
        code: "BBD",
        numeric_code: "052",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BBD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Belarusian Ruble",
        code: "BYN",
        numeric_code: "933",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Belize Dollar",
        code: "BZD",
        numeric_code: "084",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BZD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Bermudian Dollar",
        code: "BMD",
        numeric_code: "060",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BMD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Indian Rupee",
        code: "INR",
        numeric_code: "356",
        exponent: 2,
        withdrawal_date: "",
        symbol: "INR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Ngultrum",
        code: "BTN",
        numeric_code: "064",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Boliviano",
        code: "BOB",
        numeric_code: "068",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BOB",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Mvdol",
        code: "BOV",
        numeric_code: "984",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Convertible Mark",
        code: "BAM",
        numeric_code: "977",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BAM",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pula",
        code: "BWP",
        numeric_code: "072",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BWP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Norwegian Krone",
        code: "NOK",
        numeric_code: "578",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NOK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Brazilian Real",
        code: "BRL",
        numeric_code: "986",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BRL",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Brunei Dollar",
        code: "BND",
        numeric_code: "096",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BND",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Bulgarian Lev",
        code: "BGN",
        numeric_code: "975",
        exponent: 2,
        withdrawal_date: "",
        symbol: "BGN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Burundi Franc",
        code: "BIF",
        numeric_code: "108",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Cabo Verde Escudo",
        code: "CVE",
        numeric_code: "132",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Riel",
        code: "KHR",
        numeric_code: "116",
        exponent: 2,
        withdrawal_date: "",
        symbol: "KHR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Canadian Dollar",
        code: "CAD",
        numeric_code: "124",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CAD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Cayman Islands Dollar",
        code: "KYD",
        numeric_code: "136",
        exponent: 2,
        withdrawal_date: "",
        symbol: "KYD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Chilean Peso",
        code: "CLP",
        numeric_code: "152",
        exponent: 0,
        withdrawal_date: "",
        symbol: "CLP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Unidad de Fomento",
        code: "CLF",
        numeric_code: "990",
        exponent: 4,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Yuan Renminbi",
        code: "CNY",
        numeric_code: "156",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CNY",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Colombian Peso",
        code: "COP",
        numeric_code: "170",
        exponent: 2,
        withdrawal_date: "",
        symbol: "COP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Unidad de Valor Real",
        code: "COU",
        numeric_code: "970",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Comorian Franc",
        code: "KMF",
        numeric_code: "174",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Congolese Franc",
        code: "CDF",
        numeric_code: "976",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Costa Rican Colon",
        code: "CRC",
        numeric_code: "188",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CRC",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kuna",
        code: "HRK",
        numeric_code: "191",
        exponent: 2,
        withdrawal_date: "",
        symbol: "HRK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Cuban Peso",
        code: "CUP",
        numeric_code: "192",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CUP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Peso Convertible",
        code: "CUC",
        numeric_code: "931",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Netherlands Antillean Guilder",
        code: "ANG",
        numeric_code: "532",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ANG",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Czech Koruna",
        code: "CZK",
        numeric_code: "203",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CZK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Danish Krone",
        code: "DKK",
        numeric_code: "208",
        exponent: 2,
        withdrawal_date: "",
        symbol: "DKK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Djibouti Franc",
        code: "DJF",
        numeric_code: "262",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Dominican Peso",
        code: "DOP",
        numeric_code: "214",
        exponent: 2,
        withdrawal_date: "",
        symbol: "DOP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Egyptian Pound",
        code: "EGP",
        numeric_code: "818",
        exponent: 2,
        withdrawal_date: "",
        symbol: "EGP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "El Salvador Colon",
        code: "SVC",
        numeric_code: "222",
        exponent: 2,
        withdrawal_date: "",
        symbol: "SVC",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Nakfa",
        code: "ERN",
        numeric_code: "232",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lilangeni",
        code: "SZL",
        numeric_code: "748",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Ethiopian Birr",
        code: "ETB",
        numeric_code: "230",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Euro",
        code: "EUR",
        numeric_code: "978",
        exponent: 2,
        withdrawal_date: "",
        symbol: "EUR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Falkland Islands Pound",
        code: "FKP",
        numeric_code: "238",
        exponent: 2,
        withdrawal_date: "",
        symbol: "FKP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Danish Krone",
        code: "DKK",
        numeric_code: "208",
        exponent: 2,
        withdrawal_date: "",
        symbol: "DKK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Fiji Dollar",
        code: "FJD",
        numeric_code: "242",
        exponent: 2,
        withdrawal_date: "",
        symbol: "FJD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFP Franc",
        code: "XPF",
        numeric_code: "953",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BEAC",
        code: "XAF",
        numeric_code: "950",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Dalasi",
        code: "GMD",
        numeric_code: "270",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lari",
        code: "GEL",
        numeric_code: "981",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GEL",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Ghana Cedi",
        code: "GHS",
        numeric_code: "936",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Gibraltar Pound",
        code: "GIP",
        numeric_code: "292",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GIP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Danish Krone",
        code: "DKK",
        numeric_code: "208",
        exponent: 2,
        withdrawal_date: "",
        symbol: "DKK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Quetzal",
        code: "GTQ",
        numeric_code: "320",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GTQ",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pound Sterling",
        code: "GBP",
        numeric_code: "826",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GBP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Guinean Franc",
        code: "GNF",
        numeric_code: "324",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Guyana Dollar",
        code: "GYD",
        numeric_code: "328",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GYD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Gourde",
        code: "HTG",
        numeric_code: "332",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lempira",
        code: "HNL",
        numeric_code: "340",
        exponent: 2,
        withdrawal_date: "",
        symbol: "HNL",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Hong Kong Dollar",
        code: "HKD",
        numeric_code: "344",
        exponent: 2,
        withdrawal_date: "",
        symbol: "HKD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Forint",
        code: "HUF",
        numeric_code: "348",
        exponent: 2,
        withdrawal_date: "",
        symbol: "HUF",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Iceland Krona",
        code: "ISK",
        numeric_code: "352",
        exponent: 0,
        withdrawal_date: "",
        symbol: "ISK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Indian Rupee",
        code: "INR",
        numeric_code: "356",
        exponent: 2,
        withdrawal_date: "",
        symbol: "INR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rupiah",
        code: "IDR",
        numeric_code: "360",
        exponent: 2,
        withdrawal_date: "",
        symbol: "IDR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "SDR (Special Drawing Right)",
        code: "XDR",
        numeric_code: "960",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Iranian Rial",
        code: "IRR",
        numeric_code: "364",
        exponent: 2,
        withdrawal_date: "",
        symbol: "IRR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Iraqi Dinar",
        code: "IQD",
        numeric_code: "368",
        exponent: 3,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pound Sterling",
        code: "GBP",
        numeric_code: "826",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GBP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "New Israeli Sheqel",
        code: "ILS",
        numeric_code: "376",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ILS",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Jamaican Dollar",
        code: "JMD",
        numeric_code: "388",
        exponent: 2,
        withdrawal_date: "",
        symbol: "JMD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Yen",
        code: "JPY",
        numeric_code: "392",
        exponent: 0,
        withdrawal_date: "",
        symbol: "JPY",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pound Sterling",
        code: "GBP",
        numeric_code: "826",
        exponent: 2,
        withdrawal_date: "",
        symbol: "GBP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Jordanian Dinar",
        code: "JOD",
        numeric_code: "400",
        exponent: 3,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Tenge",
        code: "KZT",
        numeric_code: "398",
        exponent: 2,
        withdrawal_date: "",
        symbol: "KZT",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kenyan Shilling",
        code: "KES",
        numeric_code: "404",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "North Korean Won",
        code: "KPW",
        numeric_code: "408",
        exponent: 2,
        withdrawal_date: "",
        symbol: "KPW",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Won",
        code: "KRW",
        numeric_code: "410",
        exponent: 0,
        withdrawal_date: "",
        symbol: "KRW",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kuwaiti Dinar",
        code: "KWD",
        numeric_code: "414",
        exponent: 3,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Som",
        code: "KGS",
        numeric_code: "417",
        exponent: 2,
        withdrawal_date: "",
        symbol: "KGS",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lao Kip",
        code: "LAK",
        numeric_code: "418",
        exponent: 2,
        withdrawal_date: "",
        symbol: "LAK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Lebanese Pound",
        code: "LBP",
        numeric_code: "422",
        exponent: 2,
        withdrawal_date: "",
        symbol: "LBP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Loti",
        code: "LSL",
        numeric_code: "426",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rand",
        code: "ZAR",
        numeric_code: "710",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ZAR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Liberian Dollar",
        code: "LRD",
        numeric_code: "430",
        exponent: 2,
        withdrawal_date: "",
        symbol: "LRD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Libyan Dinar",
        code: "LYD",
        numeric_code: "434",
        exponent: 3,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Swiss Franc",
        code: "CHF",
        numeric_code: "756",
        exponent: 2,
        withdrawal_date: "",
        symbol: "CHF",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pataca",
        code: "MOP",
        numeric_code: "446",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Denar",
        code: "MKD",
        numeric_code: "807",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MKD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Malagasy Ariary",
        code: "MGA",
        numeric_code: "969",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Malawi Kwacha",
        code: "MWK",
        numeric_code: "454",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Malaysian Ringgit",
        code: "MYR",
        numeric_code: "458",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MYR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rufiyaa",
        code: "MVR",
        numeric_code: "462",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Ouguiya",
        code: "MRU",
        numeric_code: "929",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Mauritius Rupee",
        code: "MUR",
        numeric_code: "480",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MUR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "ADB Unit of Account",
        code: "XUA",
        numeric_code: "965",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Mexican Peso",
        code: "MXN",
        numeric_code: "484",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MXN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Mexican Unidad de Inversion (UDI)",
        code: "MXV",
        numeric_code: "979",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Moldovan Leu",
        code: "MDL",
        numeric_code: "498",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Tugrik",
        code: "MNT",
        numeric_code: "496",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MNT",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Moroccan Dirham",
        code: "MAD",
        numeric_code: "504",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Mozambique Metical",
        code: "MZN",
        numeric_code: "943",
        exponent: 2,
        withdrawal_date: "",
        symbol: "MZN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kyat",
        code: "MMK",
        numeric_code: "104",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Namibia Dollar",
        code: "NAD",
        numeric_code: "516",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NAD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rand",
        code: "ZAR",
        numeric_code: "710",
        exponent: 2,
        withdrawal_date: "",
        symbol: "ZAR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Nepalese Rupee",
        code: "NPR",
        numeric_code: "524",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NPR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFP Franc",
        code: "XPF",
        numeric_code: "953",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Cordoba Oro",
        code: "NIO",
        numeric_code: "558",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NIO",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "CFA Franc BCEAO",
        code: "XOF",
        numeric_code: "952",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Naira",
        code: "NGN",
        numeric_code: "566",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NGN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "New Zealand Dollar",
        code: "NZD",
        numeric_code: "554",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NZD",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Norwegian Krone",
        code: "NOK",
        numeric_code: "578",
        exponent: 2,
        withdrawal_date: "",
        symbol: "NOK",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rial Omani",
        code: "OMR",
        numeric_code: "512",
        exponent: 3,
        withdrawal_date: "",
        symbol: "OMR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Pakistan Rupee",
        code: "PKR",
        numeric_code: "586",
        exponent: 2,
        withdrawal_date: "",
        symbol: "PKR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "No universal ",
        code: "",
        numeric_code: "",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Balboa",
        code: "PAB",
        numeric_code: "590",
        exponent: 2,
        withdrawal_date: "",
        symbol: "PAB",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Kina",
        code: "PGK",
        numeric_code: "598",
        exponent: 2,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Guarani",
        code: "PYG",
        numeric_code: "600",
        exponent: 0,
        withdrawal_date: "",
        symbol: "PYG",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Sol",
        code: "PEN",
        numeric_code: "604",
        exponent: 2,
        withdrawal_date: "",
        symbol: "PEN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Philippine Peso",
        code: "PHP",
        numeric_code: "608",
        exponent: 2,
        withdrawal_date: "",
        symbol: "PHP",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Zloty",
        code: "PLN",
        numeric_code: "985",
        exponent: 2,
        withdrawal_date: "",
        symbol: "PLN",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Qatari Rial",
        code: "QAR",
        numeric_code: "634",
        exponent: 2,
        withdrawal_date: "",
        symbol: "QAR",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Romanian Leu",
        code: "RON",
        numeric_code: "946",
        exponent: 2,
        withdrawal_date: "",
        symbol: "RON",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Russian Ruble",
        code: "RUB",
        numeric_code: "643",
        exponent: 2,
        withdrawal_date: "",
        symbol: "RUB",
        rates: None,
        next_update_date: "",
    },
    Currency {
        name: "Rwanda Franc",
        code: "RWF",
        numeric_code: "646",
        exponent: 0,
        withdrawal_date: "",
        symbol: "",
        rates: None,
        next_update_date: "",
    },
];
