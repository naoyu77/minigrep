use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        //      "ファイルを読み込むことができるはずでした"
        .expect("Should have been able to read the file");

    //       "テキスト:\n{contents}"
    println!("With text:\n{contents}");
}
