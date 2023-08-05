use csv::{self};
use std::{fs, io};

fn main() {
  /* 1. 訓練データの学習 */

  // 入力する訓練データ
  let input_data = fs::read_to_string("wine.txt")
    .expect("訓練データファイルの読み取りに失敗しました");

  // 訓練データ
  let mut training_data = vec![vec![]; 3];

  // 入力されたデータをcsvとして読み取る
  let mut reader = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_reader(input_data.as_bytes());

  // csvのレコードを配列に入れる
  for record in reader.records(){
    let record = record.expect("訓練データ(wine.txt)の読み取りに失敗しました");
    let category = record[0].parse::<usize>().unwrap(); //レコードの最初の値をカテゴリ(クラス)とする

    if category >= 1 && category <= 3{
      training_data[category - 1].push(record);
    }else{
      // クラスが1, 2, 3以外だった場合
      panic!("入力された訓練データが正しくありません");
    }
  }

  // それぞれのカテゴリの各値の平均をテンプレートとする
  let mut template  = vec![vec![0f32; 13]; 3];
  for i in 0..3 {
    let len = training_data[i].len() as f32; // カテゴリの訓練データの数
    for j in 1..14{
      template[i][j-1] = training_data[i]
        .iter()
        .fold(0f32, |sum, record| sum + record[j].parse::<f32>().unwrap()) // データの合計を出す(Stringをfloatに変換してsumに畳みこむ)
        / len; // 合計値を配列の長さで割る(平均)
    }
  }


  /* 2. テストデータの識別 */

  println!("テストデータを入力してください");

  let mut reader = csv::ReaderBuilder::new()
  .has_headers(false)
  .from_reader(io::stdin());

  match reader.records().next() {
    Some(Ok(record)) => {
      if record.len() != 13 {
        panic!("入力データが正しくありません");
      }

      let mut length = vec![0.; 3]; // 各クラスと入力データの距離

      for i in 0..3{
        for j in 0..13{
          let muc = template[i][j]; // μc
          let x = record[j].parse::<f32>().unwrap(); // x
          length[i] += 2. * muc * x - (muc * muc); // 2*μc*x - μc * μc が最大となるクラスを探す
        }
      }

      let mut max = 0.;
      let mut ans = 0;

      for i in 0..3{
        if length[i] > max{
          max = length[i];
          ans = i+1;
        }
      }

      println!("入力されたテストデータのクラスは{}です", ans); //結果の出力
    }

    Some(Err(e)) =>  panic!("入力データが正しくありません, {}", e),
    None => panic!("入力データが正しくありません")
  }
}
