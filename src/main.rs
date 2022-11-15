use std::env;
mod players;
mod cpu;
mod help;
use help::help;
use cpu::cpu;
use players::play;
fn main() {
    let versus = env::args().nth(1).expect("Não foi possivel");
    if versus == "p" {
        play()
    }else if versus == "c" {
        cpu()
    }else if versus == "help" || versus == "h"{
        help()
    }else{
        println!("Digite h ou help para ajuda, não foi possivel achar essa opção...")

    }
}
