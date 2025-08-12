
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

    pub typ: FigureType

}
