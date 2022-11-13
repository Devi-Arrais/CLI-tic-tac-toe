use std::clone::*;
use std::fmt::*;

#[derive(Clone, PartialEq)]
pub enum Status {
    X,
    O,
    Nulo,
}

impl Status {
    fn to_string(&self) -> String {
        match self {
            Status::X => " X ".to_string(),
            Status::O => " O ".to_string(),
            Status::Nulo => "   ".to_string(),
        }
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Velha {
    campo: [Status; 9],
}
impl Velha {
    pub fn new() -> Velha {
        Velha {
            campo: [
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
                Status::Nulo,
            ],
        }
    }
    pub fn campo(&self) -> String {
        let mut map = "   |   |   \n".to_string();
        let mut i = 0;
        while i < 8 {
            let fmt = format!(
                "{:?}|{:?}|{:?}\n",
                self.campo[i],
                self.campo[i + 1],
                self.campo[i + 2]
            );
            map.push_str(&fmt);

            if i < 6 {
                map.push_str("---|---|---\n");
            } else {
                map.push_str("   |   |   \n");
            }
            i += 3;
        }
        map
    }
    pub fn player1(&mut self, posicao: u8, play: Status) -> i32 {
        let jogada: u8 = posicao - 1;
        let p;
        if self.campo[jogada as usize] == Status::Nulo {
            self.campo[jogada as usize] = play;
            p = 2;
        } else {
            println!("Posição invalida escolha novamente");
            p = 1;
        }
        p
    }
    pub fn player2(&mut self, pos: u8, player: Status) -> i32 {
        let zone: u8 = pos - 1;
        let y;
        if self.campo[zone as usize] == Status::Nulo {
            self.campo[zone as usize] = player;
            y = 1;
        } else {
            println!("Posição ocupada escolha novamente");
            y = 2;
        };
        y
    }
    pub fn check_win(&mut self) -> bool {
        // Ganhador horizontal
        let mut contador = 0;
        while contador < 9 {
            let campo_1 = self.campo[contador].clone();
            if campo_1 == Status::Nulo {
                break;
            };
            if campo_1 == self.campo[contador + 1] && campo_1 == self.campo[contador + 2] {
                self.quem_ganhou(campo_1);
                return false;
            };
            contador += 3;
        }
        // ganhador vertical
        for i in 0..3 {
            if self.campo[i] == self.campo[i + 3]
                && self.campo[i] == self.campo[i + 6]
                && self.campo[i] != Status::Nulo
            {
                let l = self.campo[i].clone();
                self.quem_ganhou(l);
                return false;
            };
        }
        // Ganhador diagonal
        if self.campo[0] == self.campo[4]
            && self.campo[0] == self.campo[8]
            && self.campo[0] != Status::Nulo
        {
            let k = self.campo[0].clone();
            self.quem_ganhou(k);
            return false;
        } else if self.campo[2] == self.campo[4]
            && self.campo[2] == self.campo[6]
            && self.campo[2] != Status::Nulo
        {
            let x = self.campo[2].clone();
            self.quem_ganhou(x);
            return false;
        };
        // Se deu velha
        for i in 0..9 {
            if self.campo[i] == Status::Nulo {
                return true;
            };
        }
        self.quem_ganhou(Status::Nulo);
        return false;
    }
    fn quem_ganhou(&mut self, play: Status) {
        match play {
            Status::X => println!("X Ganhou"),
            Status::O => println!("O Ganhou"),
            _ => println!("Deu Empate"),
        }
    }
}
