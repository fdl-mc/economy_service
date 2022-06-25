fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/fdl/users/v1/users.proto"], &["proto"])?;
    tonic_build::configure().compile(&["proto/fdl/economy/v1/economy.proto"], &["proto"])?;

    Ok(())
}
