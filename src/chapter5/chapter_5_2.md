# Chapter 5.2 メソッド記法

メソッドは関数に似たもので`fn`キーワードと名前で宣言したり引数や戻り値もありますが、構造体(またはenumかトレイトオブジェクト　トレイトオブジェクトはこのコースでは扱いません)上に定義します。

関数と違うところとして最初の引数は必ず`self`になります。この`self`はメソッドが呼び出されている構造体インスタンスを表しています。

では前のchapterで作成したBMIの面積を計算するbmi_cal関数をbmi_calメソッドとして作り替えましょう。
```rust
#[derive(Debug)]
struct Data{
        height: f32,  //身長
        weight: f32,  //体重
}

impl Data {//メソッド定義
    fn bmi_cal(&self) -> f32{
        self.weight / (self.height/100.0 * self.height/100.0)
    }
}

fn main() {
    let man1 = Data{
        height: 175.2, 
        weight: 68.0,
    };

    println!("man1のBMIは{}",man1.bmi_cal());

}   
```
`Rectangle`構造体上に関数を定義するには`impl`というキーワードから始めます。そして引数を`&self`に変え、自分自身の値(`height`や`weight`)を使い計算し値を返すようにしています。呼び出す際には`man1.bmi_cal()`のように自分の名前とメソッド名をつけて呼び出すことができます。またメソッドの引数において`mut`をつけることで可変な借用にすることも可能です。もちろん複数の引数を渡すことも可能です。

では下のコードに適正体重を計算するメソッドを定義し[5.1. 構造体](./chapter_5_1.md)の最後に行った問題と同じ答えになるか確認してみましょう。
```rust,editable
#[derive(Debug)]
struct Data{
        height: f32,  //身長
        weight: f32,  //体重
}

impl Data {//メソッド定義
    fn bmi_cal(&self) -> f32{
        self.weight / (self.height/100.0 * self.height/100.0)
    }

}

fn main() {
    let man1 = Data{
        height: 175.2, 
        weight: 68.0,
    };

    println!("man1のBMIは{}",man1.bmi_cal());

}   
```
答えは下記のようになります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押して確認してみましょう。
```rust,noplayground
impl Data {//メソッド定義
    fn bmi_cal(&self) -> f32{
        self.weight / (self.height/100.0 * self.height/100.0)
    }

#   fn app_wei(&self) -> f32 {
#       (self.height/100.0 * self.height/100.0) * 22.0   
#   }
}

#   
#//impl Data{
#//    fn app_wei(&self) -> f32 {
#//       (self.height/100.0 * self.height/100.0) * 22.0   
#//   }
#//}
#//このようにまたimplキーワードを使い定義するのも可能 
```

<br>

またメソッドに複数の引数を渡すことも可能です。
```rust
#[derive(Debug)]
struct Data{
        height: f32,  //身長
        weight: f32,  //体重
}

impl Data {//メソッド定義
    fn bmi_cal(&self,x:f32) -> f32{
        self.weight / (self.height/x * self.height/x)
    }
    fn app_wei(&self, x:f32, y:f32) -> f32 {
        (self.height/100.0 * self.height/100.0) * 22.0   
    }

}

fn main() {
    let man1 = Data{
        height: 175.2,
        weight: 68.0,
    };

    println!("man1のBMIは{}",man1.bmi_cal(100.0));
    println!("man1の適正体重は{}",man1.app_wei(100.0 ,22.0));

}  
```
`man1.bmi_cal(100.0)`とすることで`bmi_cal`メソッドの第二引数に100.0が入ります。このように複数の引数を取り処理を行うことも可能です。

メソッド記法のメリットとしては全てのメソッドで自分の型(`Data`など)を繰り返し書く必要がなくなることとコードを書く際にある構造体が提供している機能を`impl`ブロックまとめることで、他の人が構造体が提供している機能についてコード全体から探す必要がなくなるというメリットがあります。


### 関連関数
`impl`のブロック内には`self`を引数に取らない関数を定義することができます。これは構造体に関連するので**関連関数**と呼ばれます。あくまで関数であり、対象となる構造体のインスタンスが存在しないためメソッドではありません。今まで使ってきた`String:from`は関連関数です。関連関数の定義の方法は`impl`ブロック内に今までの関数と同様に定義するだけです。

関連関数は構造体の新規インスタンスを返すときによく使用されます。
```rust
#[derive(Debug)]
struct Data{
        height: f32,  //身長
        weight: f32,  //体重
}

impl Data {//メソッド定義
    fn bmi_cal(&self,x:f32) -> f32{
        self.weight / (self.height/x * self.height/x)
    }
    fn app_wei(&self, x:f32, y:f32) -> f32 {
        (self.height/100.0 * self.height/100.0) * 22.0   
    }
    
    fn data_insert(height: f32, weight: f32) -> Data{ //関連関数の定義
        Data{
            height,
            weight,
        }
    }
}

fn main() {
    let man1 = Data{
        height: 175.2, 
        weight: 68.0,
    };

    let woman1 = Data::data_insert(160.4, 54.0);

    println!("woman1のデータは{:?}",woman1);

    println!("man1のBMIは{}",man1.bmi_cal(100.0));
    println!("man1の適正体重は{}",man1.app_wei(100.0 ,22.0));
    println!("woman1のBMIは{}",woman1.bmi_cal(100.0));
    println!("woman1の適正体重は{}",woman1.app_wei(100.0 ,22.0));
}  
```

関連関数は今まで使ってきた`String::from`のように`構造体名::関連関数名`のように呼び出すことができます。

### 問題
ある直線上にある点のy座標を求めるコードを作成しましょう。

コードの条件
* 傾きと断片のフィールドを持つ構造体を作成する
* x座標を与えられたときに直線上のy座標を求めるコードをメソッドとして定義する

```rust,editable
fn main(){

}
```
<br>

入出力例

傾き 2 断片 7の時<br>
x座標 -5 <br>
出力 yは-3<br>


答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。
```rust
#struct Line{
#    slope: i32,
#    segment: i32,
#}

#impl Line{
#    fn point(&self, x:i32) -> i32 {
#        self.slope * x + self.segment
#    } 
#}

fn main(){
#    let str = Line{
#        slope: 2,
#        segment: 7
#    };

#    let x = -5;

#    println!("y={}x+{}のxが{}の時のyは{}",str.slope, str.segment, x, str.point(x));

}
```
<br>

5人の成績を判断するコードを作成しましょう。(90点以上 S 80点以上 A 70点以上 B 60以上 C 60点以下 D)成績の判断は構造体のメソッドか関連関数で定義しましょう。

コードの条件
* 名前と成績のフィールドを持つ構造体を作成する
* 成績の判断をメソッドか関連関数で行う
* 一つの配列に5人のインスタンスを格納する。
```rust,editable
fn main(){
    
}
```
<br>

入出力例

変数の値 
名前　Taro　点数 79<br>
名前　Yusuke　点数 97<br>
名前　Hanako　点数 60<br>
名前　Hide　点数 43<br>
名前　Miyu　点数 88<br>

出力
Taro 点数は79 評価はB<br>
Yusuke 点数は97 評価はS<br>
Hanako 点数は60 評価はC<br>
Hide 点数は43 評価はD<br>
Miyu 点数は88 評価はA<br>


答えは下記になります。![表示](../img/%E8%A1%A8%E7%A4%BA.png)を押してください。

メソッドバージョン
```rust
#struct Student{
#    name: String,
#    score: u32,
#}
#impl Student {
#    fn score_judge(&self) -> char{
#        if (self.score >= 90) {
#            'S'
#        }else if (self.score >= 80){
#            'A'
#        }else if (self.score >= 70){
#            'B'
#        }else if (self.score >= 60){
#            'C'
#        }else{
#            'D'
#        }
#    }
#}

fn main(){
#    let st1 = Student{
#        name: String::from("Taro"),
#        score: 79,
#    };
#    let st2 = Student{
#        name: String::from("Yusuke"),
#        score: 97,
#    };
#    let st3 = Student{
#        name: String::from("Hanako"),
#        score: 60,
#    };
#    let st4 = Student{
#        name: String::from("Hide"),
#        score: 43,
#    };
#    let st5 = Student{
#        name: String::from("Miyu"),
#        score: 88,
#    };
    
#    let std = [&st1, &st2, &st3, &st4, &st5];
#    let std_score = [st1.score_judge(), st2.score_judge(), st3.score_judge(), st4.score_judge(), st5.score_judge()];
    
#    let mut count = 0;

#   while count < 5{
#      println!("st{}の名前は{} 点数は{} 評価は{}",count+1 ,std[count].name, std[count].score, std_score[count]);
#        count += 1;
#    }
}
```
 
関連関数バージョン
```rust
#struct Student{
#    name: String,
#    score: u32,
#}
#impl Student {
#    fn score_judge(score:u32) -> char{
#        if (score >= 90) {
#            'S'
#        }else if (score >= 80){
#            'A'
#        }else if (score >= 70){
#            'B'
#        }else if (score >= 60){
#            'C'
#        }else{
#            'D'
#        }
#    }
#}

fn main(){
#    let st1 = Student{
#        name: String::from("Taro"),
#        score: 79,
#    };
#    let st2 = Student{
#        name: String::from("Yusuke"),
#        score: 97,
#    };
#    let st3 = Student{
#        name: String::from("Hanako"),
#        score: 60,
#    };
#    let st4 = Student{
#        name: String::from("Hide"),
#        score: 43,
#    };
#    let st5 = Student{
#        name: String::from("Miyu"),
#        score: 88,
#    };

#    let std = [&st1, &st2, &st3, &st4, &st5];
#    let mut count = 0;

#    while count < 5{
#        println!("st{}の名前は{} 点数は{} 評価は{}",count+1 ,std[count].name, std[count].score, Student::score_judge(std[count].score));
#        count += 1;
#    }
}
```

