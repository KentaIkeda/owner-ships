fn main() {
    // stack memory, heap memory
    // コンパイル時に長さが分かっているものはスタックメモリ
    // 逆に分からない、変更されるであろうものはヒープメモリへと格納され
    // そのメモリのポインタ（参照）が返される
    
    // 関数が呼び出されると、スタックには
    // 引数として渡した値
    // 関数内で定義されている変数
    // ヒープへのポインタ
    // が積まれ、関数の処理が終わると、スタックから取り除かれる
    let x = &String::from("Hello Rust");
    process_string(x); // ここで所有権が移動しprocess_string()が終了した時に自動的にメモリから解放される

    // ここでxは使えない
    println!("{}", x); // => ??
    // 現在は参照を渡しているので所有権は移っていない
    // https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html
}

fn process_string(s: &String) {
    println!("{}", s);
} // sのメモリはここで解放される