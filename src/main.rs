fn process_string(s: &String) {
    println!("{}", s);
} // sのメモリはここで解放される

fn main() {
    let x = String::from("Hello Rust");
    process_string(&x); // ここで所有権が移動しprocess_string()が終了した時に自動的にメモリから解放される

    // ここでxは使えない
    // println!("{}", x); // => ??
    // 現在は参照を渡しているので所有権は移っていない

    // 文字列リテラルは不変である
    let _str = "Hello world"; // これは文字列リテラルを束縛しているので不変

    // ユーザーからの入力を待つために用意されている型がString型
    // String型はヒープメモリを確保するので、コンパイル時に長さが分かっていなくても、値を保持することができている
    // from関数を使用することで、文字列リテラルからString型を生成することが可能
    let mut string_type = String::from("これはString型の値です");
    // このString::fromから生成された文字列を変更することができます。
    string_type.push_str(": ほらね、コンパイルエラーが発生せず、無事に文字列を付け加えることができたでしょう？");
    // println!("string_type is {}", string_type);

    { // 新しくスコープを作成
        // 変数sはここから有効になる。この時点でsの所有権は新しく作成されたスコープにある
        let _s = String::from("Hello Rust");
    } // スコープが終了すると、自動的にメモリがOSに変換（解放）される
    // スコープから抜けると、Rustがdropという特別な関数を呼び、メモリが解放されています

    {
        // 変数xが5を束縛
        let x = 5;
        // 変数yがxのコピーを束縛
        let _y = x;

        // 変数s1がStringのhelloを束縛
        let s1 = String::from("hello");
        // 果たしてこれはコピーされる？
        let _s2 = s1; // されない
        // ここでs1がs2にコピーされたことで、s1が保持していた所有権がs2に移った
    }
    {
        // 参照と借用について
        // let s1 = String::from("hello");
        // let len = calculate_length(&s1);

        // println!("The length of '{}' is {}.", s1, len);
    }
    {

        // 借用したデータの変更
        let mut s1 = String::from("Hello Rust");
        let mut s2 = String::from("Hello React");
        change_string(&mut s1, &mut s2);
        println!("{}, {}", s1, s2);
        // let mut s = String::from("hello");

        // ミュータブル名参照は、同一スコープ内に1つまでしか作成できない
        // let r1 = &mut s;
        // let r2 = &mut s;

        // println!("{}, {}", r1, r2);
    }
    {
        // スライス型
        let s = String::from("Hello Rust");
        let word = first_word(&s); // sの最初のワードの長さを取得(5)

        // s.clear(); // sの文字がクリアされ""となる。
        // compile error

        println!("{}", word); // s.clear()で文字列が""になっているにも関わらず、取得した時の値である5が出力される
                               // 要は、文字と同期されていないのが問題
        println!("{}", word);
        // let hello = &s[0..5]; // &s[..5]と0から始まる場合は0を省略して記述することも可能
        // let rust = &s[6..11];
        // println!("{}, {}", hello, rust);

    }
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}
fn change_string(s1: &mut String, s2: &mut String) {
    s1.push_str("!");
    s2.push_str("!");
}
fn first_word(s: &String) -> &str {
    // この関数がやっていることは文字を1つ1つ確認し、空白が見つかったら空白のインデックスを返す。
    // 空白が見つからないなら文字列の長さを返す。

    // Stringをバイトの配列に変更
    let bytes = s.as_bytes();
    
    // iterメソッドでイテレータを生成
    // iterメソッドはコレクション内の各要素を返す
    // enumerateメソッドはインデックスと各要素の参照のペアをタプルとして返すメソッド
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s[..]
}