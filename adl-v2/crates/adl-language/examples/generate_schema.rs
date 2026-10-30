fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&adl_language::json_schema()).expect("serialize schema")
    );
}
