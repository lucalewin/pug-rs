use pug::prelude::*;

fn main() -> Result<()> {
    let compiled = pug::compile("./examples/basic.pug")?;

    println!("compiled:\n{compiled:#?}");

    Ok(())
}
