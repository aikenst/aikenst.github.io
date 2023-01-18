# Chapter 5.1 構造体

構造体はタプルのように複数の型の値を使用して形成することができます。また、構造体の各データに名前をつけれるのでタプルよりも各データの意味が明確になっています。またこの名前を使用することでアクセスを容易にします。
```rust,noplayground
# fn main(){
    struct User{
        username: String,
        email: String,
        age: i32,
    }
# }
```
構造体を定義する時には`struct`キーワードを使用し、構造体全体に名前をつけます。この名前は構造体の意義を表すものを基本的に使用します。構造体の型としては構造体名が独自の型になります(今回は`User`型)。構造体の中には**フィールド**と呼ばれる意味を持つデータの名前と型を決め定義することができます。各フィールドは`フィールド名:型`のようにすることで定義できます。上のコードではユーザの情報を保持する構造体を作成しています。
```rust,noplayground
fn main(){
    struct User{
        username: String,
        email: String,
        age: i32,
    }

    let user1 = User{
        username: String::from("username1"),
        email: String::from("username1@example.com"),
        age: 20,
    };

}
```
構造体を使用するために各フィールドに対して具体的な値を指定し、構造体のインスタンス(実際に処理に使うもの `user1`など)を生成します。またインスタンスを生成する際、各フィールドの値は構造体が宣言した順番で指定する必要はありません。

構造体のインスタンスから特定の値を得る際には`user1.age`のようにドット記法を用います。また誕生日を迎え、年齢(`user1.age`)を変更する必要があるなど特定の値を変更したい際にはインスタンスを生成する際に可変で定義する必要があります。ただ、一部のフィールドのみを可変にすることはできず、全てのフィールドの値が可変になります。
```rust
fn main(){
    struct User{
        username: String,
        email: String,
        age: i32,
    }

    let mut user1 = User{
        username: String::from("username1"),
        email: String::from("username1@example.com"),
        age: 20,
    };

    println!("user1の名前は{}",user1.username);
    
    user1.username=String::from("helloworld");//user1のusernameを変更

    println!("user1の名前は{}",user1.username);
}
```

また構造体を関数の最後の式として生成することで構造体のインスタンスを返すこともできます。

構造体を定義する場所を変えることで構造体に関連する操作が行いやすくなります。
```rust
struct User{
    username: String,
    email: String,
    age: i32,
}

fn main(){
    
    let user1 = init_user(String::from("username1"),String::from("username1@example.com"));

    println!("user1の名前は{}",user1.username);
}   

fn init_user (username: String, email: String) -> User{
    User{
        username: username,
        email: email,
        age: 20,
    }
}
```
上のコードでは構造体を`main`関数の外側で定義しているのでどこでも構造体のインスタンスの受け渡しや生成を行うことができます。構造体を`main`関数内のみで扱い、外の他の関数では扱えないようにしたい際には今までのように構造体の定義を`main`関数内に入れる方がいいでしょう。

`init_user`関数において構造体のフィールドの数が多い場合、構造体のフィールド名と関数の仮引数名をフィールドの数分書くのは少し面倒だと思います。そこでRustでは省略して書く方法があります。
```rust
struct User{
    username: String,
    email: String,
    age: i32,
}

fn main(){
    
    let user1 = init_user(String::from("username1"),String::from("username1@example.com"));

    println!("user1の名前は{}",user1.username);

}

fn init_user (username: String, email: String) -> User{
    User{
        username,
        email,
        age: 20,
    }    
}
```
このようにフィールド名と仮引数名を同じにすることで値の束縛を省略して書くことができ、フィールドの数が多くなるとより便利さがわかると思います。

<br>

新しくインスタンスを生成する際にすでにある他のインスタンスの値を使用することができ、変更したい箇所のみを書くことで省略して書くことができます。
```rust
fn main(){
    struct Set{
        ramen: String,
        sub:String,
        soup: String,
        value: i32,
    }

    let user1 = order(String::from("醤油ラーメン"),String::from("餃子"));

    let user2 = Set{
        ramen: String::from("味噌ラーメン"),
        sub: String::from("チャーハン"),
        soup:String::from("中華スープ"),
        ..user1
    };

    println!("user1のラーメンは{} サブは{} スープは{} 価格は{}円",user1.ramen,user1.sub,user1.soup,user1.value);
    println!("user2のラーメンは{} サブは{} スープは{} 価格は{}円",user2.ramen,user2.sub,user2.soup,user2.value);

    fn order (ramen: String, sub: String) -> Set{
        Set{
            ramen,
            sub,
            soup:String::from("中華スープ"),
            value:1000
        }
    }
} 
```
`user2`を生成する際に`..user1`を使い自分が更新する`ramen`と`sub`以外を`user1`と同じ値にしています。この時`soup`も同じなので省略しようとすると所有権が移動してしまい`user1`の方が使えなくなってしまうので注意しましょう。`..user1`は実際には`soup:user1.soup`という動作であり`soup`は文字列なのでコピーされず所有権が移動してしまいます。

<br>

構造体に似たものとして構造体名があり意義を含むがフィールドに名前が付いてなく型だけのタプルを**タプル構造体**と言います。タプル構造体はタプル全体に名前をつけそのタプルを他のタプルと異なる型にしたい場合に有用です。

タプル構造体を定義する際には構造体のように`struct`キーワードを使い、構造体名とタプルに含まれる型を書いていきます。
```rust
fn main(){
    struct Color(i32,i32,i32);
    struct LightColor(i32,i32,i32);

    let black = Color(0,0,0);
    let white = LightColor(0,0,0);


}
```

ここで`black`と`white`は違う型なので注意してください。異なるタプル構造体のインスタンスであるため、それぞれ`Color`型と`LightColor`型になります。タプル構造体の要素にアクセスする際にはタプルと同様に`.`を用いてアクセスすることができ、また分配などもできます。

### 問題

身長と体重のフィールドを持つ構造体を用いてBMI {体重(kg)}/{身長(m)^2} と適正体重{身長(m)^2 * 22}を計算するプログラムを作成しましょう。これをもとに構造体を作り計算するコードを書きましょう。

コードの条件
* 身長と体重のフィールドを持つ構造体を作成する
* BMIと適正体重を計算する関数を作成する

```rust,editable
fn main(){


}
```
<br>

入出力例

身長 175.2 体重 68.0<br> 
BMIは22.153418 適正体重は67.52908<br>


答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
#struct Data {
#    height: f32,  //身長
#    weight: f32,  //体重
#}
#
fn main(){

#    let man1 = Data {
#        height: 175.2, //最初から身長をmで設定するなら1.752
#        weight: 68.0,
#    };
#    println!("man1のBMIは{} 適正体重は{}",bmi_cal(&man1),app_wei(&man1));//この後値を使う可能性があるので&をつけて参照を渡している
#    //&をつけなくても計算することができるが所有権が移動してしまうのでman1が使えなくなる   

}
#
#fn bmi_cal(data: &Data) -> f32{


#    data.weight / (data.height/100.0 * data.height/100.0)

#}

#fn app_wei(data: &Data) -> f32{

#    (data.height/100.0 * data.height/100.0) * 22.0    

#}
```
<br>

ここでデバッグ(コードのバグを直す)時に使えるものを簡単に紹介します。自分が生成したインスタンスの中身をちゃんと想定通りになっているかコードの途中で確認したい時があると思います。その際に`println!()`を使うと思いますが全て`{}`と`変数名.フィールド名`を組み合わせて書くのは面倒だと思います。これは直接結果を見たいというユーザ向けの出力になっています。そこで`{}`を`{:?}`にしましょう。これは`Debug`と呼ばれる出力の形でこの状態であれば配列なども要素数が多くなければ一度に見ることができます。
しかし構造体ではまだ使うことができません。実際に見てましょう。ついでに少しエラーを見て直す練習をしてみましょう。エラーの表示の中のhelpの表示付近を見るといいでしょう。

```rust,editable
fn main(){
    struct Data{
        height: f32,  //身長
        weight: f32,  //体重
    }

    let man1 = Data{
        height: 175.2, //最初から身長をmで設定するなら1.752
        weight: 68.0,
    };

    println!("man1の中身は{:?}",man1);
}
```
答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
#fn main(){
#//note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
#//#[derive(Debug)]をRectangleに追加または自分で実装してください
#//help: consider annotating `Rectangle` with `#[derive(Debug)]
#//Rectangleに#[derive(Debug)]という注釈を考えましょう
#//これらの表示がエラー表示の中にあると思います
#//このようにエラーの表示がかなり丁寧に教えてくれます
#    #[derive(Debug)]
#    struct Rectangle{
#        height: f32,  //身長
#        weight: f32,  //体重
#    }
#
#    let man1 = Rectangle{
#        height: 175.2, //最初から身長をmで設定するなら1.752
#        weight: 68.0,
#    };


#    println!("man1の中身は{:?}",man1);

#}
```
また`{:?}`を`{:#?}`とすることでフィールドごとに改行されて表示されます。

<br>

3人の学生を身長順に並び替えるコードを作成しましょう。また比較し並び替えるのは新しく関数を作成し行いましょう。(ヒント：可変な参照を行いましょう)

コードの条件
* 身長と体重のフィールドをもつ構造体を作成する
* 一つの配列に三人の学生のインスタンスを格納する
* 現在の並びと変更後の並びを表示する(身長と体重だけでいい)
* 並び替える関数を作成する

```rust,editable

//三人の身長と体重の値は自分で設定しましょう
fn main(){


}

```
<br>

入出力例

1人目 身長176.3 体重69.3 2人目 身長180.5 体重74.5 3人目 身長168.2 体重66.0<br> 
出力<br>
176.3 69.3<br>
180.5 74.5<br>
168.2 66.0<br>
並び替えた順番は<br>
168.2 66.0<br>
176.3 69.3<br>
180.5 74.5<br>


答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
#struct Student {
#    height: f32,
#    weight: f32,
#}

fn main(){
#    let st1 = Student{
#        height: 176.3,
#        weight: 69.3,
#    };
#    let st2 = Student{
#        height: 180.5,
#        weight: 74.5,
#    };
#    let st3 = Student{
#        height: 168.2,
#        weight: 66.0,
#    };

#    let mut std = [st1,st2,st3];//この型は&[String]

#    println!("現在の並びは");
#    for element in &std{
#        println!("身長：{} 体重：{}",element.height,element.weight);
#    }
    
#    sort(&mut std,3);

#    println!("並び替えた順番は");

#    for element in &std{
#        println!("身長：{} 体重：{}",element.height,element.weight);
#    }
}

#fn sort(std :&mut [Student], num: usize){
#    let mut i:usize = 0;
#
#    while i < num {
#        let mut j:usize = num-1;
#        while i < j {
#            if(std[j -1].height > std[j].height){
#                let height_swap = std[j-1].height;
#                std[j-1].height=std[j].height;
#                std[j].height= height_swap;
#
#                let weight_swap = std[j-1].weight;
#                std[j-1].weight=std[j].weight;
#                std[j].weight= weight_swap;
#            }
#            j-=1;
#        }
#        i+=1;
#    }
#    
#}
```
