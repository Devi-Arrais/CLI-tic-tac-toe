mod game;
use game::Velha;
use std::io;

fn main() {
   let mut game = Velha::new();
   //std::process::Command::new("clear").status().unwrap(); 
   println!("{}", game.campo());
   let mut jogador = 1; 
   loop{
      if jogador == 1{
      println!("Digite a posição que deseja: ");

      let mut posicao = String::new();
      io::stdin().read_line(&mut posicao).ok().expect("Não foi possivel pega a posição");
      let conv: u8 = posicao.trim().parse().unwrap();
      game.player1(conv, game::Status::X);
     // std::process::Command::new("clear").status().unwrap();
      println!("{}", game.campo());
      jogador += 1;
      }else{
      println!("Digite a posição que deseja: ");

      let mut posicao = String::new();
      io::stdin().read_line(&mut posicao).ok().expect("Não foi possivel pega a posição");
      let conv: u8 = posicao.trim().parse().unwrap();
      game.player1(conv, game::Status::O);
     // std::process::Command::new("clear").status().unwrap();
      println!("{}", game.campo());
      jogador -= 1;

      }
}
}
