fn main() {
    println!("🔍 NeoDB CLI");

    neodb_core::hello_core();
    neodb_parser::hello_parser();

    // Example usage stub
    // let query = "select a from t where x > 10";
    // let parsed = neodb_parser::parse(query).unwrap();
    // println!("{:#?}", parsed);
}