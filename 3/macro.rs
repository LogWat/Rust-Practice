use std::io::Write;

fn main() {
    // プログラムの異常終了
    // panic!("It will panic");

    println!("hello");
    eprint!("hello {}", "error");

    let mut w = Vec::new();

    // バッファーにバイト列書き込み
    write!(&mut w, "{}", "ABC"); // UTF-8byte列
    writeln!(&mut w, " is 123"); // UTF-8byte列
    dbg!(w);

    println!("filename: {}", file!());
    println!("linenum: {}", line!());
    println!("is test {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}