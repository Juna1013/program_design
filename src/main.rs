use rand::Rng;
use std::io::{self, Write};

// じゃんけんの手
#[derive(Debug, Clone, Copy, PartialEq)]
enum Hand {
    Rock,     // グー
    Scissors, // チョキ
    Paper,    // パー
}

impl Hand {
    // ユーザーの入力文字列から手に変換する
    fn from_input(s: &str) -> Option<Hand> {
        match s.trim() {
            "1" => Some(Hand::Rock),
            "2" => Some(Hand::Scissors),
            "3" => Some(Hand::Paper),
            _ => None,
        }
    }

    // 乱数でランダムな手を生成する
    fn random() -> Hand {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Hand::Rock,
            1 => Hand::Scissors,
            _ => Hand::Paper,
        }
    }

    // 手の表示名
    fn name(&self) -> &'static str {
        match self {
            Hand::Rock => "グー",
            Hand::Scissors => "チョキ",
            Hand::Paper => "パー",
        }
    }
}

// 勝敗の結果
enum Outcome {
    Win,
    Lose,
    Draw,
}

// プレイヤー視点で勝敗を判定する
fn judge(player: Hand, cpu: Hand) -> Outcome {
    if player == cpu {
        return Outcome::Draw;
    }

    match (player, cpu) {
        (Hand::Rock, Hand::Scissors)
        | (Hand::Scissors, Hand::Paper)
        | (Hand::Paper, Hand::Rock) => Outcome::Win,
        _ => Outcome::Lose,
    }
} 

fn main() {
    println!("じゃんけんゲーム");

    loop {
        println!("\n手を選んでください");
        println!("1: グー");
        println!("2: チョキ");
        println!("3: パー");
        println!("q: 終了");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("入力の読み込みに失敗しました");
            continue;
        }

        if input.trim().eq_ignore_ascii_case("q") {
            println!("ゲームを終了します");
            break;
        }

        let player = match Hand::from_input(&input) {
            Some(h) => h,
            None => {
                println!("無効な入力です。1, 2, 3のいずれかを入力してください");
                continue;
            }
        };

        let cpu = Hand::random();

        println!("\nあなた: {}", player.name());
        println!("CPU    : {}", cpu.name());

        match judge(player, cpu) {
            Outcome::Win => println!("結果: あなたの勝ち🎉"),
            Outcome::Lose => println!("結果: あなたの負け..."),
            Outcome::Draw => println!("結果: あいこ"),
        }
    }
}
