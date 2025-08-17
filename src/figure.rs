
#[derive(Debug)]
pub enum Player{
    Player(u8),
    None
}

#[derive(Debug)]
pub enum FigureType{
    King,
    Pawn
}

#[derive(Debug)]
pub struct Figure {

    player: Player,
    value: u8,
    pub typ: FigureType
}

impl Figure {
    pub fn new(player: Player, typ: FigureType) -> Figure{
        Figure{
            player,
            typ,
            value: 250
        }
    }

    pub fn increase(&mut self, val: u8){
        if self.value as u16 + (val as u16) < 250 {
            self.value += val;
        }
        else{
            self.value = 250;
        }
    }

    pub fn decrease(&mut self, val: u8) {
        if self.value as i16 - (val as i16) > 0 {
            self.value -= val;
        }
        else {
            self.value = 0;
        }
    }

    pub fn val(&self) -> u8{
        self.value
    }
}
