use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
// use tokenizers::pre_tokenizers::whitespace::Whitespace;
use tokenizers::tokenizer::{Result, Tokenizer};
use tokenizers::{OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer};

fn main() -> Result<()> {
    let tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
    let input_text = "Hey there!\t This is a {(real-time) test}.\n [This] is a second testing for Tokenizers library in Rust.   This bird is a Tokan.";
    let encoding = tokenizer.encode(input_text, false)?;
    println!("{:?}", encoding.get_tokens());
    println!("{:?}", encoding.get_ids());

    // let pre_tokenizer = Whitespace {};
    let pre_tokenizer: BertPreTokenizer = BertPreTokenizer {};
    let mut pre_tokenized = PreTokenizedString::from(input_text);
    match pre_tokenizer.pre_tokenize(&mut pre_tokenized) {
        Ok(_) => {
            let splitted = pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte);
            for split in splitted {
                println!("{:?}", split);
            }
            // println!(
            //     "{:?}",
            //     pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte)
            // );
        }

        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    Ok(())
}

// fn main() {
//     let pre_tokenizer = Whitespace {};
//     let mut pre_tokenized =
//         PreTokenizedString::from("Hey there! This is a real-time test. This is a second test for Tokenizers library in Rust. This bird is a Tokan.");
//     match pre_tokenizer.pre_tokenize(&mut pre_tokenized) {
//         Ok(_) => {
//             println!(
//                 "{:?}",
//                 pre_tokenized.get_splits(OffsetReferential::Original, OffsetType::Byte)
//             );
//         }
//         Err(err) => {
//             eprintln!("Error: {}", err);
//         }
//     }
// }
