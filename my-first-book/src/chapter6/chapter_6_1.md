# chapter 6.1 列挙体

>このチャプターのコードは[The Rust Programming Language 6.1. Enumを定義する](https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html)から引用しています

enumは同じものとして扱うべきだが異なる一つの値を取る際に有効です。例えば、IPアドレスが挙げられます。現在のバージョンとしてバージョン4と6がありますが、この値を扱う際には同時に両方の値は使いません。しかしIPアドレス全体に何か処理を行う際には同じ型として扱う必要があります。この扱いができる点が構造体よりも良い点です。

列挙型を定義してみましょう。定義する際には`enum`キーワードを使います。
```rust
enum IpAddr{
    V4,
    V6,
}
```
ここでV4やV6を列挙子と言います。このそれぞれの列挙子の先頭は大文字にし、`-`などは使えないので注意しましょう。具体的に使えないものはenumを使用していきながら覚えていきましょう。

enumは構造体同様に独自の型(今回は`IpAddr`型)になり、関数の引数などにも使うことができます。上のコードの定義の仕方だとどんな種類かがわかるがデータを保持していないので構造体などを使用してenumの列挙子と値を紐づける必要があります。enumのみで値と紐付ける方法を下記に示します。
```rust
fn main(){
    enum IpAddr{
        V4(String),
        V6(String),  
    }

    let data1 = IpAddr::V4(String::from("127.0.0.1"));
    let data2 = IpAddr::V6(String::from("11・・11"));//アドレスを省略しています

}
```
`IpAddr::V4`のようにして列挙子のインスタンスを生成しています。このようにすることでenumの各列挙子と値を紐づけることができます。

また構造体と違う点としてenumは常に同じ型とデータの量を持つ必要はありません。
```rust
struct User{
        username: String,
        email: String,
        age: u32,
}

enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}
fn main(){
    let user1 = User{
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        age: 29,
    };//インスタンスを生成する際には必ずこの三つのフィールドを使用する
    
    let data1 = IpAddr::V4(127,0,0,1);
    let data2 = IpAddr::V6(String;;from("11・・11"));
    //data1とdata2は同じIpAddr型であるがインスタンスを生成する際に違う値を使用している
}
```
このコードに注目してください。`User`型のインスタンスは`username email age`の値をそれぞれ必ず持つ必要がありますが`IpAddr`型のインスタンスはV4とV6のどちらを扱うかによって紐づけるデータの型などが異なります。このようにenumでは同じ独自の型の中でも取る値に違いを出すことができます。 

構造体とenumを組み合わせることで有効的に使うことができます。
```rust
struct IpV4Addr{
    //省略
}

struct IpV6Addr{
    //省略
}

enum IpAddr{
    V4(IpV4Addr),
    V6(IpV6Addr),
}

fn main(){
    //省略
}
```
このようにすることで各列挙子に対して複数の種類のデータを格納することができます。実際にこれは標準のライブラリでこのように定義されています。標準のライブラリを自分のプログラム内で使えるようにすると自分自身で独自に標準ライブラリと同じもの(`IpAddr`など)を定義できないので注意しましょう。

enumの利点として複数の種類のデータを取れることで構造体よりも簡潔に書くことができます。複数の構造体を列挙子として一つにすることで独自の型が一つで済みますが構造体にすると列挙子一つに対して一つの構造体を作成する必要があります。
```rust
fn main() {
    enum Message {
        Quit, //何も値を取らない(正確には`()`をとる)
        Move { x: i32, y: i32 },//構造体
        Write(String),//String
        ChangeColor(i32, i32, i32),//タプル構造体
    } 

    struct QuitMessage; 
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); 
    struct ChangeColorMessage(i32, i32, i32);

}
```
このようにenumと構造体でそれぞれ同じものを作るとこのようになります。`Messege`型でいずれかの値を扱う関数を作成する際には`Messege`型を受け取るか構造体一つ一つを受け取るのではコードの簡単さが違います。

構造体とenumで似たところもあります。構造体のようにメソッドを定義することができます。
```rust,noplayground

fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // メソッド本体はここに定義される
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```
構造体と同様に`impl`キーワードを用いてメソッドを定義します。また引数も第一引数は`self`です。このコードでは`m`がメソッドを呼んでいるのでmの中身がselfが表す値になります。

<br>
<br>

enumに関連して少し便利なものについて紹介します。

enumの`Option`型というものについてです。
```rust,noplayground
enum Option<T> {
    Some(T),
    None,
}
```
このように定義されているもので有益すぎて最初から含まれており明示的に導入する必要がありません。また`Some`や`None`も明示的に導入する必要がなく、つまり`Option::`という接頭辞なしで使用することができます。

`Option`型は他言語でいう`null`と同じように使われるもので変数が`null`(値がないという状態)かそれ以外(1など値がある状態)のどちらかであると他言語では決められています。これは現在無効であったり存在しないを表現したいときに使います。しかしこの`null`の問題として`null`の値を何か値を持っているかのように使用するとエラーが出ることです。

先程の`Option`が`null`と似たような働きをします。`<T>`というのはこのコースでは扱わないものですが`<T>`は`Some`列挙子があらゆる型のデータを一つ持つことを意味します。

```rust,noplayground
let some_num = Some(5);
let some_str = Some("hello");

let no_num: Option<u32> = None;
```
このように使用することができます。`None`を使用する時はその変数が`Option` 型の何になるかを明示する必要があります。これは`null`と何が違うのでしょうか？

```rust
fn main(){
    let x: u32 = 5;
    let y: Option<u32> = Some(5);

    println!("x+yの値は{}",x+y);
}
```
これを実行するとわかるのですがxとyの型が異なるので計算をさしてもらえません。他言語のように`null`の際に計算さしてもらえず、`Option<u32>`を扱うと値が存在しない可能性があるのでコンパイラが弾くようになっています。なので実際に計算するためには`None`である場合の処理と`Some(T)`のT型の値を取り出し計算する処理が必要になります。

なので列挙子それぞれに対してコードの選択ができる`match`式が有効になってきます。この`match`式は普通のenumを扱う上でも重要になります。

enumは構造体と同じように`.`を使って列挙子に紐づけられた値を扱うことができません。`{:?}`のようにして`Debug`の出力の形にすればenumのインスタンスの中身を見ることはできます。enumのインスタンスに紐づけられた値を処理に使いたい時にも`match`式を扱うことでうまく扱うことができます。

構造体とenumの類似点
* メソッドの定義ができる
* 作成した独自の型になる
* mutをつけることで値は変更可能

構造体とenumの異なる点
* enumは列挙子ごとに値を設定する　構造体は全てのフィールドの値を設定する必要がある
* 構造体はドット記法で値を扱うことができる
