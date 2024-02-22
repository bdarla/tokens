use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;

    let encoding = tokenizer.encode("Hey there! This is a test. This is a second test. ", false)?;
    println!("{:?}", encoding.get_tokens());
    println!("{:?}", encoding.get_ids());

    Ok(())
}
