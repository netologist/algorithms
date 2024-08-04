use snowflake_uid::{SnowflakeGenerator, MACHINE_ID_MAX};


#[test]
fn test_generate_unique_ids() {
    let mut generator = SnowflakeGenerator::new(1);
    let id1 = generator.generate();
    let id2 = generator.generate();
    assert_ne!(id1, id2, "IDs should be unique");
}

#[test]
fn test_machine_id_limit() {
    let machine_id = MACHINE_ID_MAX;
    let generator = SnowflakeGenerator::new(machine_id);
    assert_eq!(generator.machine_id, machine_id);
}

#[test]
#[should_panic(expected = "Machine ID out of range")]
fn test_machine_id_out_of_range() {
    SnowflakeGenerator::new(MACHINE_ID_MAX + 1);
}