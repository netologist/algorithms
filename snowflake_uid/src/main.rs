use snowflake_uid::SnowflakeGenerator;

fn main() {
    let mut generator = SnowflakeGenerator::new(1);
    for _ in 0..10 {
        let id = generator.generate();
        println!("Generated ID: {}", id);
    }
}
