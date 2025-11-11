use chrono::NaiveDate;
use std::error::Error;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
struct ConsumptionLineItem {
    start_date: String,
    end_date: String,
    start_read_kwh: String,
    end_read_kwh: String,
    tariff_pence: String,
    standing_charge_pence: String,
}

pub fn calculate_charges(filepath: String) -> i64 {
    let file = File::open(filepath).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut line_items = Vec::new();

    for result in rdr.deserialize() {
        let record: ConsumptionLineItem = result.unwrap();
        line_items.push(record);
    }

    let mut total_charges = 0;

    for item in line_items {
        let start_date = NaiveDate::parse_from_str(&item.start_date, "%Y-%m-%d").unwrap();
        let end_date = NaiveDate::parse_from_str(&item.end_date, "%Y-%m-%d").unwrap();
        let consumption =
            item.end_read_kwh.parse::<i64>().unwrap() - item.start_read_kwh.parse::<i64>().unwrap();
        let total_standing_charge = ((end_date - start_date).num_days())
            * item.standing_charge_pence.parse::<i64>().unwrap();
        let cost = consumption * item.tariff_pence.parse::<i64>().unwrap() + total_standing_charge;
        total_charges += cost;
    }

    total_charges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_bill_returns_aggregate_of_single_line_csv() {
        let result = calculate_charges(String::from("data/single_line.csv"));
        assert_eq!(result, 570);
    }

    #[test]
    fn generate_bill_returns_aggregate_of_multiple_line_csv() {
        let result = calculate_charges(String::from("data/multiple_line.csv"));
        assert_eq!(result, 1020);
    }

    #[test]
    fn generate_bill_handles_bad_data() {
        let result = calculate_charges(String::from("data/bad_line.csv"));
        assert_eq!(result, 1020);
    }
}
