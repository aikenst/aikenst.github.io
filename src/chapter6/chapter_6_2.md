# chapter6.2 match式 

`match`式は一度[3.1 条件式](../chapter3/chapter_3_1.md)で触れたと思います。基本的には変わりません。ここでは少し詳しく説明し、enumなどと組み合わせて使用する例を見ていきます。

少し復習をしましょう。下のコードは[3.1 条件式](../chapter3/chapter_3_1.md)で紹介したものです。
```rust
fn main(){
    let num = 3;

    match num{
        1 => println!("numの値は1"), //最後がカンマ(,)なのに注意
        2 => println!("numの値は2"),
        3 => println!("numの値は3"),
        4 => println!("numの値は4"),
        _ => println!("numの値は1-4ではありません"),
    }
}
```
numの値に対して左側のパターンと比較していき条件を満たしたものの右の処理を行います。このコードではnumの値が3なので3の部分の右の式が実行されます。

```rust
enum Money{
    One,
    Five,
    Ten,
    Fifty,
    Hundred,
    Fivehundred,
}

fn main(){
    let coin1 = Money::Five;

    println!("coin1の値は{}",coin_value(coin1));
}

fn coin_value (money: Money) -> u32 {
    match money {
        Money::One => 1,
        Money::Five => 5,
        Money::Ten => 10,
        Money::Fifty => 50,
        Money::Hundred => 100,
        Money::Fivehundred => 500,
        _ => 0,//硬貨ではないので0にしている
    }
}
```
enumと単純に組み合わせたものが上のコードのようになります。こちらも前に紹介したものと基本的に変わりません。`=>`の右の部分は式でなければなりません。またこの式の部分に`{}`を使い複数の処理を行うこともできます。
```rust,noplayground
fn coin_value (money: Money) -> u32 {
    match money {
        Money::One => 1,
        Money::Five => {
            println!("この硬貨は穴が空いています");
            5
        },
        Money::Ten => 10,
        Money::Fifty => {
            println!("この硬貨は穴が空いています");
            50
        },
        Money::Hundred => 100,
        Money::Fivehundred => 500,
        _ => {
            println!("硬貨ではありません"),
            0
        },
    }
}
```
このようにすることで`Money::Five`や`Money::fifty`に一致した時には`println!`とそれぞれの値を返す式にすることができます。

<br>

enumと`match`式を組み合わせて中の値を取り出せることを[6.1 enum](./chapter_6_1.md)の最後で軽く紹介したと思います。その例を見てみましょう。
```rust
#[derive(Debug)]
enum Name {
    Kitazato,
    Noguti,
    Natsume,
    Ito,
}
enum Money{
    One,
    Five,
    Ten,
    Fifty,
    Hundred,
    Fivehundred,
    Thousand(Name),
}

fn main(){
    let know_money = Money::Thousand(Name::Noguti);

    println!("このお金の金額は{}",coin_value(know_money));
}

fn coin_value (money: Money) -> u32 {
    match money {
        Money::One => 1,
        Money::Five => 5,
        Money::Ten => 10,
        Money::Fifty => 50,
        Money::Hundred => 100,
        Money::Fivehundred => 500,
        Money::Thousand(name) => {
            println!("この1000円札の人は{:?}",name);
            1000
        }
        _ => {
            println!("1000以下のお金ではありません");
            0
        }
    }
}
```
`match`式のパターンで1000円札の人物の名前の部分(今回は`Name::Noguti`)を`name`に束縛しています。このようにすることで列挙子から値を取り出すことができます。

これを使うことで前のchapterの`Option<T>`での問題を解決できるようになります。`Some(T)`の値を取り出したかったはずです。
```rust
fn main(){
    let x: u32 = 5;
    let y: Option<u32> = Some(5);
    let z: Option<u32> = None;

    println!("yの値をxの値分増やしたものは{}",plus(x,y));

    println!("zの値をxの値分増やしたものは{}",plus(x,z));
}

fn plus(x: u32, y: Option<u32>) -> u32 {
    match y {
        None => 0,
        Some(t) => t + x,
    }
}
```
yとzをそれぞれ`Some(5)`や`None`に設定していますがこれらももちろんパターンにマッチします。ここで`Option<T>`に対する`match`を行う際には`None`に対する処理がないとエラーになります。そのために`None`というパターンか`_`(ワイルドカード)を使用する必要があります。これは`match`式はどれかのパターンに一致しないといけないためです。

しかしある値の時のみ処理を行いたい際に`match`式を使うと`None`に一致するコードなど無駄なコードを書くことになってしまいます。そこで`if let`を使うことで短く書くことができます。
```rust
fn main(){
    let some_value = Some(1);
    if let Some(1) = some_value {
        println!("One");
    }
}
```
このように`if let`では`=`で区切り左側がパターン右側が評価する変数や値になります。`some_value`の値は`Some(1)`なので左側のパターンと一致するのでブロック内の`println!("One");`が実行されます。
```rust
fn main(){
    let some_value = Some(1);
    if let Some(mut i) = some_value {
        println!("iの値は{}",i);
        i= i+1;
        println!("iの値は{}",i);
    }
    println!("some_valueの値は{:?}",some_value);
}
```
また上のコードのように値を束縛することができます。`match`式のように全てのパターンに対応するかのチェックはないので注意してください。

`if let`では`if`式のように`else`を追加することができます。
```rust
fn main(){
    let some_value = Some(5);
    if let Some(1) = some_value {
        println!("1です");
    }else{
        println!("1以外の値です");
    }
}
```
`if`式のように`else`が使えるので`if else`で繋げていくこともできます。

### 問題
春夏秋冬を判断するプログラムを作成しましょう。

コードの条件<br>
* 季節の列挙型を作成する
* match式を利用して季節を判断する

```rust,editable
fn main(){

}
```
答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押して確認しましょう。
```rust
#enum Season {
#    Spring,
#    Summer,
#    Autumn,
#    Winter,
#}
#
fn main(){
#    let season = Season::Summer;

#    match season {
#        Season::Spring => println!("季節は春です"),
#        Season::Summer => println!("季節は夏です"),
#        Season::Autumn => println!("季節は秋です"),
#        Season::Winter => println!("季節は冬です"),
#}

}
```

お店で服(シャツ　コート　スウェットシャツ　カーディガン)を買う時を想定し合計点数と合計金額を計算するコードを作成しましょう<br>

コードの条件<br>
* 服の列挙型を作成する
* 買う服の種類とサイズを一つの配列で管理する
* match式を使用して服の種類を判断する

服の種類とサイズによる金額は以下の通り

|                               | S    | M    | L     | 
| ----------------------------- | :--: | :--: | :---: | 
| シャツ(Shirt)                 | 1000 | 2000 | 3000  | 
| コート(Coat)                  | 8000 | 9000 | 10000 | 
| スウェットシャツ(Sweatshirts) | 2500 | 3500 | 4500  | 
| カーディガン(Cardigan)        | 3000 | 4000 | 5000  | 

```rust,editable
fn main() {

}
```

答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押して確認しましょう。

```rust
#enum Wear{
#    Shirt(char),
#    Coat(char),
#    Sweatshirts(char),
#    Cardigan(char),
#}

#fn main(){
#    let order = [Wear::Shirt('M'), Wear::Coat('L'), Wear::Cardigan('M'), Wear::Shirt('S'), Wear::Sweatshirts('L')];

#    let mut sum = 0;
#    let mut count_order = 0;

#    for element in order { //今回はWear型の何かにはなるので(_)は利用していない
#        match element {
#            Wear::Shirt(s) => {
#                sum += shirt_money(s);
#                count_order += 1;
#            },
#            Wear::Coat(s) => {
#                sum += coat_money(s);
#                count_order += 1;
#            },
#            Wear::Sweatshirts(s) => {
#                sum += sweatshirts_money(s);
#                count_order += 1;
#            },
#            Wear::Cardigan(s) => {
#                sum += cardigan_money(s);
#                count_order += 1;
#            },
#        }
#    }
#    println!("合計{}点で金額は{}になります",count_order,sum);
#}

#fn shirt_money(s: char) -> u32 {
#    match s {
#        'S' => 1000,
#        'M' => 2000,
#        'L' => 3000,
#         _  => 0,
#    }
#}

#fn coat_money(s: char) -> u32 {
#    match s {
#        'S' => 8000,
#        'M' => 9000,
#        'L' => 10000,
#         _  => 0,
#    }
#}

#fn sweatshirts_money(s: char) -> u32 {
#    match s {
#        'S' => 2500,
#        'M' => 3500,
#        'L' => 4500,
#         _  => 0,
#    }
#}

#fn cardigan_money(s: char) -> u32 {
#    match s {
#        'S' => 3000,
#        'M' => 4000,
#        'L' => 5000,
#         _  => 0,
#    }
#}
```