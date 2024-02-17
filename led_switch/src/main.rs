use rainmaker::{Rainmaker, error::RMakerError};

fn main() -> Result<(), RMakerError>{
    std::env::set_var("RUST_BACKTRACE", "1"); // for debugging

    let mut rmaker = Rainmaker::new()?;
    rmaker.init();
    rmaker.init_wifi();
    rainmaker::prevent_drop();
    
    Ok(())
}
