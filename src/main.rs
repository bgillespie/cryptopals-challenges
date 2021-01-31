mod challenges;
mod solutions;

type Rezult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Rezult<()> {
    solutions::set_1_challenge_3::main()?;
    solutions::set_1_challenge_4::main()?;
    solutions::set_1_challenge_5::main()?;
    // solutions::set_1_challenge_6::main()?;
    Ok(())
}
