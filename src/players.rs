mod game;
use game::Velha;
use std::io;

pub fn play(){

    let mut game = Velha::new();
    std::process::Command::new("clear").status().unwrap();
    println!("{}", game.campo());
    let mut jogador: i32 = 1;
    while game.check_win() {
        if jogador == 1 {
            println!("Digite a posição que deseja: ");

            let mut posicao = String::new();
            io::stdin()
                .read_line(&mut posicao)
                .ok()
                .expect("Não foi possivel pega a posição");
            let conv: u8 = posicao.trim().parse().unwrap();
            jogador = game.player1(conv, game::Status::X);
            std::process::Command::new("clear").status().unwrap();
            println!("{}", game.campo());
        } else if jogador == 2 {
            println!("Digite a posição que deseja: ");

            let mut posicao = String::new();
            io::stdin()
                .read_line(&mut posicao)
                .ok()
                .expect("Não foi possivel pega a posição");
            let conv: u8 = posicao.trim().parse().unwrap();
            jogador = game.player2(conv, game::Status::O);
            std::process::Command::new("clear").status().unwrap();
            println!("{}", game.campo());
        }
    }
}
