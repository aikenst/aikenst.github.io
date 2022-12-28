# chapter 7.2 モジュールとuse

モジュールは関数、構造体や他のモジュールなどの要素の集合です。モジュールはクレート内のコードをグループ化し可読性と再利用性を上げるのに役立ちます。

モジュールは要素の**プライバシー**を制御することできます。プライバシーとは要素がコードの外側で使える**公開(public)** か内部の実装の詳細であり外部で使えない**非公開(private)** の二種類の状態のことです。Rustでは標準であらゆる要素(モジュールやその中の関数　構造体　モジュールなどの要素)は非公開の状態です。これはパスを説明する際に出てきます。

バレーボールの試合を例にして考えてみましょう。選手(`player`)と審判(`referee`)がいます。選手は攻撃と守備に別れ、それぞれレシーブやアタックなどをします。審判は得点の加算やアウトの判定などを行います。これをモジュールで表してみましょう。モジュールの定義は`mod 名前`で定義されます。
```rust,noplayground
mod player {
    mod attacker {
        fn receive() {
            //省略
        }

        fn toss (){
            //省略
        }

        fn attack (){
            //省略
        }
    }

    mod blocker {
        fn block() {
            //省略
        }
        
        fn through() {
            //省略
        }

        fn receive() {
            //省略
        }

    }
}

mod referee {
    //省略

}
```
モジュールの中に`attacker`や`blocker`のように他のモジュールを定義することができます。他にも関数や構造体などの要素を置くことができます。

`src/main.rs`や`src/lib.rs`はクレートルートと呼ばれると言いました。この名前は**モジュールツリー**と呼ばれるクレートのモジュール構造の根っこ(ルート)にこれら2つのファイルの中身が`crate`というモジュールを形成するためです。モジュールツリーを下に示します。

![モジュールツリー](../img/%E3%83%84%E3%83%AA%E3%83%BC.png)

このツリーを見るとどのモジュールが親子の関係なのかや兄弟の関係であるのかがすぐにわかります。例えば`crate`は全ての親であるし、`attacker`は`player`の子であることがわかります。また`referee`を省略していますが`player`と`referee`や`attacker`と`blocker`が兄弟の関係になります。この関係を理解することでこの後に紹介するパスについて理解しやすくなります。

Rustにモジュールツリー内の要素を見つけるためにどこを探せばいいのかというのを教えるために**パス**を使います。パスには二つの形があります。

* **絶対パス**：クレートの名前か`crate`という文字列を使用し、クレートルートからたどっていく
* **相対パス**：`self`や`super`または今のモジュールの識別子を使い、現在のモジュールからたどっていく

パスはそれぞれの識別子を`::`で区切って書きます。上のコードで新しい関数を作成しそこで`attack`関数を呼び出す例を見てみましょう。必要な部分をのみで他の部分は省略しています。ただ下のコードではパスの使い方はあっているのですがエラーになる理由は後ほど説明します。
```rust
mod player {
    mod attacker {
        fn attack(){
            println!("アタック！！");
        }
    }
}

mod referee {
    fn judge(){
        //絶対パス
        crate::player::attacker::attack();
        println!("イン！！");

        //相対パス
        super::player::attacker::attack();//superは後ほど説明
        println!("アウト！！");
    }
}
fn main() {
    referee::judge();//相対パス
}
```
このように絶対パスでは`crate`かクレート名で初め、そこから目的のものまでモジュールを`::`で分けて書いていきます。一方で相対パスでは呼び出している場所からスタートします。ここで`judge`関数は`referee`モジュール内なので一度`referee`モジュールと同じ階層に行く必要があります。そのために`super`を使用しています。そこから`player`モジュールの下を順番にパスで繋いで行きます。

では先程のコードのエラーの原因を見てみましょう。これはモジュールの説明の際に出てきたプライバシーに関するエラーです。エラーを読むと`attacker`が`private`(非公開)の状態なのでエラーになっています。これを公開にするためには`pub`キーワードを使用することで公開することができます。
```rust
mod player {
    pub mod attacker {
        fn attack(){
            println!("アタック！！");
        }
    }
}

mod referee {
    fn judge(){
        //絶対パス
        crate::player::attacker::attack();
        println!("イン！！");

        //相対パス
        super::player::attacker::attack();
        println!("アウト！！");
    }
}

fn main() {
    referee::judge();
}
```
エラーの内容通り`pub`を`attacker`につけ公開しました。しかしこれもエラーになってしまいます。このコードは`player`モジュールの中身の`attacker`と`blocker`の内、`attacker`モジュールが公開されただけなのです。つまりまだ`attacker`モジュールの中身を公開していないのです。エラーの内容も`attack`が`private`というエラーです。同様に`judge`にも`pub`をつける必要があります。
```rust
mod player {
    pub mod attacker {
        pub fn attack(){
            println!("アタック！！");
        }
    }
}

mod referee {
    pub fn judge(){
        //絶対パス
        crate::player::attacker::attack();
        println!("イン！！");

        //相対パス
        super::player::attacker::attack();
        println!("アウト！！");
    }
}

fn main() {
    referee::judge();
}
```
相対パスにおいて`player`モジュールを公開していないのに`referee`モジュールから参照できるのは何故なんでしょうか？これは同じクレート内で`player`モジュールと`referee`モジュールが兄弟の関係なので`referee`から`player`を参照することができます(`player`を公開する必要がない)。それ以降は今まで説明してきたものになります。


### 相対パスをsuperで始める
相対パスにおいて親モジュールからスタートしたい際に`super`というキーワードを使います。
```rust,noplayground
mod player {
    pub mod attacker {
        pub fn attack(){
            println!("アタック！！");
        }
    }
}

mod referee {
    pub fn judge(){
        //絶対パス
        crate::player::attacker::attack();
        println!("イン！！");

        //相対パス
        super::player::attacker::attack();
        println!("アウト！！");
    }
}

```
このコードでは`judge`関数で`attack`関数を呼び出そうとしています。相対パスで呼び出そうとするためには`judge`関数の親モジュール`referee`モジュールから兄弟の関係の`player`モジュールに行き`attacker`モジュール `attack`関数へ行く必要があります。この`judge`関数の親モジュール`referee`モジュールを指定する際に`super`を使います。

### 構造体とenum
構造体やenumも`pub`を使うことで公開されますがそれぞれ少し異なる動きをします。構造体を公開するとモジュールと同様に中身のフィールドは公開されません。なので公開したいフィールドはそれぞれ設定する必要があります。一方でenumは公開すると中の列挙子も全て公開されます。
```rust
mod menu {
    pub struct Seasoncakeset {
        pub drink: String,
        season_cake: String,
    }

    impl Seasoncakeset {
        pub fn winter(drink: &str) -> Seasoncakeset {
            Seasoncakeset{
                drink: String::from(drink),
                season_cake: String::from("orangecake"),
            }
        }
    }
}

fn main(){
    let mut order = menu::Seasoncakeset::winter("coffee");
    
    order.drink = String::from("tea");
    //order.season_cake = String::from("strawberrycake");コメントを外すとエラーになる
    //order.season_cakeを表示することもprivateなのでできない
    println!("orderのドリンクは{}",order.drink);

}
```

### useキーワード
これまでのパスは長く繰り返し書くには面倒なものでした。それを簡単にするのが`use`というキーワードです。`use`キーワードを使い、パスを一度スコープに持ち込むことでパス内の要素がローカルにあるかのように呼び出すことができます。このパスは絶対パスでも相対パスでも構いません。
```rust
mod player {
    pub mod attacker {
        pub fn attack(){
            println!("アタック！！");
        }
    }
}

mod referee {
    pub fn judge(){
        //絶対パス
        crate::player::attacker::attack();
        println!("イン！！");

        //相対パス
        super::player::attacker::attack();
        println!("アウト！！");
    }
}

fn main() {
    referee::judge();
}
```
`use`を使いこれを簡単にしましょう。
```rust
mod player {
    pub mod attacker {
        pub fn attack() {
            println!("アタック！！");
        }
    }
}

use crate::player::attacker;

mod referee {
    pub fn judge(){
    
        super::attacker::attack();
        println!("イン！！");
    }
}

fn main() {
    referee::judge();
}
```
`judge`関数内で`attack`関数を呼び出す際の違いを見てください。書く量が減っていると思います。これは`crate::player::attacker`モジュールをスコープに持ち込むことで`use`が書かれた階層と同じ階層に`attacker`モジュールがあるかのように扱うことができます。なので`super::attacker::attack`と指定することで`attack`関数を呼び出すことができます。スコープに`use`で持ち込まれたパスの要素も他のパスと同様にプライバシーがチェックされるので注意してください。

また`use crate::player::attacker::attack`のようにすると呼び出す際には`attack()`だけで呼び出すことができますが自分がローカルで定義した関数かどうかがわからなくなるので基本的に親のモジュールまで`use`で持ち込みます。構造体やenumその他の要素は全て書いて持ち込みます。

### 二つの同じ名前の要素の持ち込み方

同じ名前の二つの要素を`use`でスコープに持ち込むのはRustでは許されません。そこでそれを解消する方法が二つあります。一つ目は親モジュールを使う方法です。
```rust,editable
use std::fmt;
use std::io;

fn function1() -> fmt::Result{
    //省略
    Ok(())//Result型の戻り値です
}

fn function2() -> io::Result<()> {
    //省略
    Ok(())
}

fn main(){

}
```
同じ`Result`型を持ち込んできた場合区別するために親モジュール(`fmt`や`io`)を使い区別します。これを消すとエラーになるので消して確認してみてください。

二つ目の方法は`as`キーワードを使うことです。この`as`キーワードは新しいローカルの名前をつけることができます。
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result{
    //省略
    Ok(())//Result型の戻り値です
}

fn function2() -> IoResult<()> {
    //省略
    Ok(())
}

fn main(){

}
```
今回は`std::io::Result`に`IoResult`という新しい名前をつけています。このようにすることで名前が衝突することを防ぎます。

### 外部のパッケージを使う
プログラムを書く上で乱数の生成するパッケージなど便利なものがすでに用意されています。このパッケージはモジュールなどを持ち込むように`use`キーワードを使い自分のスコープに持ち込めば使用することができます。自分のPC上にRustを構築し動かす場合は`Cargo.toml`というファイルに使いたいパッケージの情報を書き込み`use`で持ち込むことで使うことができます。このコース上で外部のパッケージを使いたい場合は`extern crate 名前`という行を追加する必要があります。これはこのコースの実行環境の問題で使っていますが実際にはほぼ使うことがなくなった記法です。このようにすることで外部クレートをスコープに持ってくることができます。
```rust
extern crate rand;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..101);
    println!("{}",secret);
}
```
このコードを複数回実行してみてください。実行結果が毎回異なると思います。`rand`クレートの具体的な実装内容は必要があれば調べてみてください。

今まで問題を解いていく際に自分で変数の値をコードに直接書き込んでいましたがライブラリを持ち込むことでユーザの入力をそのまま変数の値にすることができます。しかしこれはこのドキュメント上では扱えないので自分のPC上で行う際に行ってみてください。
```rust,noplayground
use std::io;
fn main(){
    let mut inport = String::new();

    io::stdin().read_line(&mut inport).expect("failed");

    println!("入力は{}",inport);
}
```
`std`ライブラリは標準ライブラリに含まれており、その中の`io`(入出力)ライブラリをスコープに持ち込むことでユーザの入出力を受け取ることができます。実際に`read_line`メソッドを使用し、変数(今回は`inport`)に格納しています。この`read_line`メソッドは文字列を追加する動きをするので可変な参照を渡す必要があります。`expect`メソッドは`read_line`メソッドが成功したかどうかを判断するために使用します。失敗した場合に中の文字列を表示させプログラムをそこで終了させます。

### useする際の便利な記法
`use`をする際に同じモジュールから複数の要素を持ち込むときに一行一行書いていくのが面倒だと思います。これの代わりにネストしたパスを使うことで一行で複数の要素を持ち込むことができます。
```rust,noplayground
/*
今までの記法
use std::cmp::Ordering;
use std::io;
use std::fmt;
*/
use std::{cmp::Ordering, io, fmt};

```
このようにすることで`use`文の数を減らすことができます。

また公開されている要素全てをスコープに持ち込みたいときに`glob`演算子(`*`)を使うことで全て持ち込めます。しかしプログラムで使われている名前がどこで定義されたのかわかりづらくなるので注意してください。
```rust
mod player {
    pub mod attacker {
        pub fn attack(){
            println!("アタック！！");
        }
    }

    pub mod blocker {
        pub fn block() {
            println!("ブロック！！");
        }
    }
}

use crate::player::*;

fn main(){
    attacker::attack();
    blocker::block();
}
```
このように`player`モジュールの下の公開されている要素全て扱うことができるようになります。

