use std::fmt::*;



pub enum Status{
    X, O, Nulo
}

impl Status{
    fn to_string(&self)-> String{
        match self{
            Status::X => " X ".to_string(),
            Status::O => " O ".to_string(),
            Status::Nulo => "   ".to_string(),
        }
    }
}

impl Debug for Status{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Velha{
    campo: [Status; 9],
}
impl Velha{
    pub fn new()-> Velha{
        Velha { campo: [Status::Nulo,Status::Nulo,Status::Nulo,
                        Status::Nulo,Status::Nulo,Status::Nulo,
                        Status::Nulo,Status::Nulo,Status::Nulo] }
    }
    pub fn campo(&self)-> String{
       let mut map = "   |   |   \n".to_string();
       let mut i = 0;
       while i < 8{
           let fmt = format!("{:?}|{:?}|{:?}\n", self.campo[i], self.campo[i + 1], self.campo[i + 2]);
           map.push_str(&fmt);

           if i < 6{
               map.push_str("---|---|---\n");
           }else {
               map.push_str("   |   |   \n");
           }
           i += 3;
       }
     map
    }
    pub fn player1(&mut self, posicao: u8, play: Status){
        let jogada: u8 = posicao - 1;
        loop{
            if self.campo[jogada as usize] == Status::Nulo{
               self.campo[jogada as usize] = play;
               break;

            }else{
                println!("Posição invalida escolha novamente");
                continue;
            }
        }
    }
    pub fn player2(&mut self, pos: u8, player: Status){
        let zone: u8 = pos - 1;
        loop{
            if self.campo[zone as usize] == Status::Nulo{
                 self.campo[zone as usize] = player;
                 break;
            }else{
                println!("Posição ocupada escolha novamente");
                continue;
            };
        }
   }
}
