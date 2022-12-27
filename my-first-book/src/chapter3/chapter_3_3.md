# Chapter 3.3 関数

ここではよりコードを見やすくするために**関数**について説明していきます。

関数は今まで`main`関数のみを扱ってきて、この`main`関数内に全ての処理を書いてきました。しかし規模が大きく複雑なコードを書いていく際には`main`関数が長くなっていきコードが読みづらくなってしまいます。

そこで役割ごとに新しく関数を作成することでコードを読みやすくすることができます。Rustの関数と変数の命名の慣習として全ての文字を小文字にして単語ごとにアンダースコア(_)で区切ります。関数の定義されている場所は特に重要ではなく`main`関数の前でも後でも大丈夫です。

```rust
fn main (){
    println!("main関数で呼び出された");

    another_function(); //関数呼び出し

    println!("main関数で呼び出された2回目");
}

fn another_function(){ //新しく関数作成
    println!("別の関数で呼び出された");
}
```

関数の定義する際には`fn`キーワードを使います。`fn 名前 () {}`のように定義します。呼び出す際には`関数名 ()`のように呼び出してその関数内の処理を行うことができます。呼び出した関数の処理が終わると基本的に元の関数の残りの処理を行っていきます。

`main`関数の途中で`another_function`関数が呼び出されて実行されています。`another_function`関数の処理が終わると`main`関数に戻り処理を続けます。

関数にはその中の処理に使うための引数を渡すこともできます。そのためには引数を取るように関数を定義する必要があります。引数を取るようにするには関数定義の際に`()`内に変数と型(`x: i32`のように)を描く必要があります。この変数の前にmutをつけることで可変にすることもできます。これは後ほど出てきます。定義した関数に値を渡す際には呼び出す際の`()`内に値を書いて渡すことができます。
```rust
fn main (){
    println!("二つの値を足します");
    sum(3,5);
}

fn sum (x:i32, y:i32){ //xが3 yが5の値になる
    println!("{}+{}={}",x,y,x + y);
}
```
上の例では`sum`関数に3と5の二つの値を与えて合計を計算させています。`sum`関数内では引数に設定したxとyの値に渡された3と5が入りこの変数を使い計算することができます。
```rust
fn main() {
    time_print(12,'h');
}

fn time_print(x: i32, time: char){
    println!("時間は{}{}",x,time);
}
```
上の例のように引数は同じ型である必要もありません。

### 文と式
今まであまり気にしてきませんでしたがRustのコードには**文**と**式**という違いがあります。文は基本的に(`;`)で終了しており、何か動作をするが値を返さないもので、式は値として評価されるものです。具体的に見ていきましょう。
```rust
fn main(){
    let x = 1; //let x = 式
    let y = 3+5; //let y = 式
    //let z = (let a = 1;); //let z = 文
    
}
```
このコードのコメントの部分を普通に書いて実行しようとするとエラーになります。`let a = 1;`というのは文で値がないので`let z =`の束縛がうまくいきません。逆に今までよく使ってきた`x`や`y`の定義に使用している`1`や`3+5`は式なので値が評価され`x`や`y`への束縛がうまくいきます(`let x = 1;`というのは一行で見ると文です)。他にも式になるものは関数呼び出し、マクロ呼び出し(println!()など)、`{}`で囲まれたものなどがあります。
```rust
fn main(){
    let x = {
        let y = 1;
        y + 1
    };

    let z = println!("マクロ呼び出し");//zの値は空のタプル"()"になる。

    let a = sum(3,5);

    let b = difference(7,2);

    println!("xの値{}　aの値は{} bの値は{}",x,a,b);
}

fn sum (x: i32,y: i32 ) -> i32 {
    x+y
}

fn difference (x: i32,y: i32) -> i32 {
    return x - y;
    println!("Hello world");//return以降は実行されない 警告が出るので注意
}
```
xの定義では`{}`で囲まれたコードの結果が定義に使われています。`y + 1`の後ろに`;`がついていないことに注意してください。式になるものは`;`がついていません。なのでyを定義した後、`y + 1`をした結果が式の値となりxの定義に使用されます。

マクロ呼び出しは`println!()`など`!`がついたものになります。この本では詳しくは扱いませんが値を表示させるなど複雑なコードを書く必要がなく、コードを見やすくしてくれる便利なものという感じでいいと思います。このマクロの結果は`()`空のタプルとなります。これは普通には表示させることはできないので注意してください(今はそこまで重要でないのでスキップします)。

`sum`関数の呼び出しも式となります。なので関数を呼び出した結果をaの定義に使用しています。

関数の呼び出した結果がどのように帰ってくるか見ましょう。関数を呼び出した結果を扱いたい時には関数の定義の際に注意する必要があります。呼び出したところへ返す値(**戻り値**)の設定をします。この戻り値が関数呼び出しの結果となります。この戻り値の設定は`sum (x: i32,y: i32 ) -> i32`の`-> i32`の部分になります。ここで戻り値の型を決めています。関数の結果の返し方は2通りあります。

* `return` <br>
`return`を扱うとその場で関数から値を返すことができます。それ以降のコードは実行されません。`return`は(`;`)を忘れないようにしましょう。
* 式 <br>
Rustでは関数の最後の式の値が戻り値と勝手になるので計算などの式にすることで明示的に`ruturn`を書かなくても値を返してくれます。(xの定義で利用した`{}`内でも同じ)

<br>

また戻り値はタプルで複数の値を返すこともできます。
```rust
fn main(){
    let tar = 5;

    let (x, count) = cal(tar);

    println!("{}までの値の合計は{}そして計算回数は{}回",tar,x,count);
}

fn cal(mut target: i32) -> (i32, i32){
    let mut count = 0;
    let mut sum = 0;
    while 0 < target{
        sum += target;
        count += 1;
        target -= 1;
    }
    (sum, count)
}
```

関数を役割ごとに分けて書きそれを呼び出すだけにすることでコードがかなり見やすくなります。
これまでの内容で複雑なコードを書くことができるようになりました。これからの章ではRustで重要な所有権についてまたさらに複雑なコードを書くために必要な構造体などについて学んでいきましょう。

### 問題
ある値の2乗を計算する関数と二つの値の差を計算する関数の二つを作成し2つの2乗の差を計算しましょう
```rust,editable
fn main(){

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    let x = 5;
#    let y = 8;

#    println!("xの値{}とyの値{}の2乗の差は{}",x,y,diff(x,y)); 
}

#fn sqr(a: i32) -> i32{
#    a * a
#}

#fn diff (a: i32,b: i32) -> i32{
#    if a >= b {
#        sqr(a) - sqr(b)
#    }else{
#        sqr(b) - sqr(a)
#    }
#}
```

配列の中の3の倍数の数を数える関数を作成しましょう。
```rust,editable
fn main(){
    let index = [1,3,5,4,6,8,4,3,18,32,11,12];

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
    let index = [1,3,5,4,6,8,4,3,18,32,11,12];
#    let mut count = 0;

#    for element in index {
#        count += judge(element);
#    }

#    println!("indexの中の3の倍数の数は{}",count);
}

#fn judge(element: u32) -> u32 {
#    if element % 3 == 0 {
#        1
#    }else{
#        0
#    }
#}
```