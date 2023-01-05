# Chapter 3.2 ループ文

ここでは**ループ**について説明していきたいと思います。ループはコードを書くうえで今まで複数回書いていたもの(同じ条件式を使う場合など)を省略できるようにしてくれます。

Rustには3種類のループが存在します。`loop` `while` `for`の三種類です。

### loop
loopは中のコードを自分で明示的に終了するまで永遠に繰り返します。

```rust
fn main(){
    loop{
        println!("ヤッホー");
    }
}
```
この例を実行するとヤッホーがずっと表示されることになります。このドキュメント上で行おうとするとタイムアウト(実行が終わらない)エラーになります。自分のPC上でこのコードを動かす際には`ctrl + c`を押して終了させましょう。ループを終わらせるためには`break`というキーワードを使います。

`loop`は`break`と条件式を一緒に使うことでうまく扱うことができます。
```rust
fn main(){
    let mut x = 1;

    loop{
        if x == 15 {
            break;
        }else if (x % 6)==0 {
            println!("{}は6の倍数です",x);
        }else if (x % 3)==0 {
            println!("{}は3の倍数です",x);
        }else if (x % 2)==0 {
            println!("{}は2の倍数です",x);
        }
        x += 1;
    }
}
```
このようにすることで1~15までの値の中で6の倍数 3の倍数 2の倍数のものを表示することができます。

ループを扱う際にもう一つ便利なキーワードがあります。それが`continue`です。これは`continue`以降の処理をスキップしすぐに次のループに移ります。
```rust
fn main(){
    let mut x = 1;

    loop{
        if x == 3 {
            x += 1;
            continue;
        }
        println!("x={}の時です",x);//xの値が3の時のみスキップされる
        
        if x == 5{
            break;//xの値が5になったらループ終了
        }
        x += 1;
    }
}
```
x == 3の時に`continue`というキーワードを用いることで`continue`以降の処理を行わずに進みます。カウントをする際にはカウントの数を増やす処理を`continue`の前に行うのを忘れないようにしましょう。ループが終わらずエラーになってしまいます。

ループ(`loop` `while` `for`)を扱う際にループにラベル(名前)をつけることができます。これはループ内にループが存在する場合に有効的です。通常では`break`や`continue`は一番内側の**ループ**に適用されます。ここでループラベルを用いることで`break`は`continue`を適用するループを指定することができます。ループラベルは`'name: `というような形でつけることができます。
```rust
fn main(){
    let mut count=0;

    'counting_up: loop{
        let x = 3;

        loop{
            if count < 3 { //countが3未満の時はスキップ
                count += 1;
                continue;
            }

            if  count == 7 {//countの値が7の時にcounting_upというラベルのついたループを抜ける
                break 'counting_up;
            }
            println!("{}/{}の値は{}",count,x,count/x);
            count += 1;
        }

    }
}
```
このコードでは1~7までの値を3で割った商を求めています。今回は整数型なので小数点以降は表示されません。また3より小さい場合は`continue`を使うことで処理をスキップしています。そして`count`が7になった際に`counting_up` という一番外側のラベルのついたループを抜けています。

### while
`while`はループ内で条件式を評価します。つまり条件が真の時にループが始まり、条件が偽になった時に自動的に`break`を呼び出し、ループを終了します。
```rust
fn main(){
    let a = [1,2,3,4,5];
    let mut count = 0;

    while count < 5{
        println!("aの値の{}番目の値は{}",count,a[count]);
        count += 1;
    }
}
```
このコードではcountが5になった際に条件が偽になるのでそこでループが終了します。配列を使っているので添字に注意しましょう。配列の長さよりの長い場所にアクセスしようとするとパニックしてしまいます。このように`while`を使うことで先程の`loop`や`if`などを組み合わせる必要がなく、よりコードが見やすくなります。

### for
`for`は`while`よりも高速で安全にループを実行することができます。特に配列にアクセスする際に有効的です。
```rust
fn main(){
    let a = [1,2,3,4,5];
    let mut count = 0;

    for element in a { //elementは一時的に用意したものなので名前はelementでなくても大丈夫
        println!("配列aの{}番目の値は{}",count,element);
        count += 1;
    }
}
```
`in`の後に書かれた要素を一つずつ値を取り出し、`element`に束縛され値がなくなるまでループを続けます。なので配列の要素数を超えることなくアクセスしてくれます。タプルで行うことはできないので注意してください。
```rust
fn main(){
    for element in (1..6) {
        println!("値は{}",element);
    }
}
```
1から5までの値を扱うなど配列などを使わずループを扱いたい時には上記のように書くことができます。`(1..6)`は**レンジ**と言います。これは1から6未満の数字を表しています。`(1..=6)`のようにすると1から6以下までが対象となります。

このように`for`文を使うことで配列の要素数が変わったときなどにも取り出せる要素がなくなったらループを終了するので`for`文のコードを変える必要がありません。

### 問題
loopを使用して0からの自分が決めた整数までの合計値を求めましょう。
```rust,editable
fn main(){
    let tar =    //合計値を計算する範囲の設定
}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    let tar = -5;
#    let mut sum = 0;
#    let mut count = 0;
#    
#    loop {
#       if tar < 0 {
#            break;
#        }

#        if count <= tar {
#            sum += count;
#            count += 1;
#        }else{
#            break;
#        }
#    }
#
#    loop {
#        if count >= tar {
#            sum += count;
#            count -= 1;
#        }else{
#            break;
#        }
#    }
#    println!("{}までの値の合計値は{}",tar,sum);
}
```

whileを使って配列の要素の合計と平均(小数点は切り捨て)を求めましょう
```rust,editable
fn main(){
    let list = [10,5,7,63,3];

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    let list = [10,5,7,63,3];
#    let mut count = 0;
#    let mut sum = 0;
#    print!("list:");
#    while count < 5 {
#        sum += list[count];
#        print!("{} ",list[count]);
#        count += 1;
#    }
#    print!("の合計は{} 平均は{}",sum,sum/5);

}
```
forを使って上の問題を書いてみましょう
```rust,editable
fn main(){
    let list = [10,5,7,63,3];

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    let list = [10,5,7,63,3];
#    let mut sum = 0;
#    print!("list:");
#    for element in list {
#        sum += element;
#        print!("{} ",element);
#    }
#    print!("の合計は{} 平均は{}",sum,sum/5);

}
```
九九の表を表示しましょう(二重ループを使いましょう)
```rust,editable
fn main(){

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    for i in (1..=9){
#        for j in (1..=9){
#            print!("{} ", i*j);
#        }
#        println!("");
#    }

}
```
九九で25より大きい値が出た時はそれ以降をスキップして25が出た時に九九の表示を終了するようにしましょう
```rust,editable
fn main(){

}
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
fn main(){
#    'out: for i in (1..=9){
#        for j in (1..=9){
#            if i * j > 25{
#                break;
#            }else if i * j == 25{
#                break 'out;
#            }
#            print!("{} ", i*j);
#        }
#        println!("");
#    }

}
```