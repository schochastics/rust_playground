use test_data_generation::data_sample_parser::DataSampleParser;

fn main() {
    let mut dsp = DataSampleParser::new();

    // Use the default delimiter (comma)
    dsp.analyze_csv_file(&String::from("tracking.csv"), None)
        .unwrap();
    dsp.generate_csv(100, &String::from("generated-01.csv"), None)
        .unwrap();
}
