/// A European country with associated market data.
#[derive(Debug, Clone, PartialEq)]
pub struct Country {
    pub name: &'static str,
    /// ISO 3166-1 alpha-2 code
    pub code: &'static str,
    /// Base market size index (1–100)
    pub market_size: u32,
    /// Regulatory complexity index (1–10, higher = harder)
    pub regulatory_complexity: u32,
    /// Labour cost index relative to EU average (100 = average)
    pub labour_cost_index: u32,
    /// Consumer purchasing power index (100 = EU average)
    pub purchasing_power: u32,
}

/// Returns all playable European countries.
pub fn all_countries() -> Vec<Country> {
    vec![
        Country {
            name: "Germany",
            code: "DE",
            market_size: 100,
            regulatory_complexity: 7,
            labour_cost_index: 110,
            purchasing_power: 120,
        },
        Country {
            name: "France",
            code: "FR",
            market_size: 90,
            regulatory_complexity: 8,
            labour_cost_index: 105,
            purchasing_power: 115,
        },
        Country {
            name: "United Kingdom",
            code: "GB",
            market_size: 95,
            regulatory_complexity: 6,
            labour_cost_index: 108,
            purchasing_power: 118,
        },
        Country {
            name: "Italy",
            code: "IT",
            market_size: 80,
            regulatory_complexity: 9,
            labour_cost_index: 95,
            purchasing_power: 100,
        },
        Country {
            name: "Spain",
            code: "ES",
            market_size: 75,
            regulatory_complexity: 7,
            labour_cost_index: 85,
            purchasing_power: 95,
        },
        Country {
            name: "Netherlands",
            code: "NL",
            market_size: 60,
            regulatory_complexity: 4,
            labour_cost_index: 108,
            purchasing_power: 125,
        },
        Country {
            name: "Poland",
            code: "PL",
            market_size: 65,
            regulatory_complexity: 6,
            labour_cost_index: 55,
            purchasing_power: 75,
        },
        Country {
            name: "Sweden",
            code: "SE",
            market_size: 55,
            regulatory_complexity: 3,
            labour_cost_index: 115,
            purchasing_power: 130,
        },
        Country {
            name: "Switzerland",
            code: "CH",
            market_size: 50,
            regulatory_complexity: 5,
            labour_cost_index: 145,
            purchasing_power: 155,
        },
        Country {
            name: "Portugal",
            code: "PT",
            market_size: 40,
            regulatory_complexity: 6,
            labour_cost_index: 65,
            purchasing_power: 80,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_countries_not_empty() {
        assert!(!all_countries().is_empty());
    }

    #[test]
    fn test_country_codes_are_two_chars() {
        for c in all_countries() {
            assert_eq!(c.code.len(), 2, "Country {} has invalid code", c.name);
        }
    }

    #[test]
    fn test_market_size_in_range() {
        for c in all_countries() {
            assert!(c.market_size >= 1 && c.market_size <= 100,
                "Country {} market_size out of range", c.name);
        }
    }

    #[test]
    fn test_regulatory_complexity_in_range() {
        for c in all_countries() {
            assert!(c.regulatory_complexity >= 1 && c.regulatory_complexity <= 10,
                "Country {} regulatory_complexity out of range", c.name);
        }
    }
}
