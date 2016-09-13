/* When I started working on this the
 * tests were different. Needed to add support
 * for named 'Peg's.
 */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Peg {
    A, B, C,
}

#[derive(Debug)]
struct Tower{
    hold: Vec<u32>,
    peg: Peg,
}

pub type Move = (Peg, Peg);

impl Tower {
    fn new(hold: Vec<u32>, peg: Peg) -> Tower {
        Tower{hold: hold, peg: peg}
    }
    fn pop(&mut self) -> u32 {
        self.hold.pop().unwrap()
    }
    fn push(&mut self, item: u32) {
        self.hold.push(item);
    }
}

fn move_pieces(moves: &mut Vec<Move>, n: u32,
               from: &mut Tower, to: &mut Tower,
               mid: &mut Tower) {
    if n > 0 {
        move_pieces(moves, n-1, from, mid, to);
        to.push(from.pop());
        moves.push((from.peg, to.peg));
        println!("moved from {:?} to {:?}", from.peg, to.peg);
        println!("{:?}\n{:?}\n{:?}\n---------", from, mid, to);
        move_pieces(moves, n-1, mid, to, from);
    }
}

pub fn hanoi(num_discs: u32, peg1: Peg, peg2: Peg, peg3: Peg) -> Vec<Move> {
    let mut a = Tower::new((1..num_discs+1).rev().collect::<Vec<u32>>(), peg1);
    let mut b = Tower::new(Vec::with_capacity(num_discs as usize), peg2);
    let mut c = Tower::new(Vec::with_capacity(num_discs as usize), peg3);
    let mut moves = Vec::with_capacity(2usize.pow(num_discs)-1);
    move_pieces(&mut moves, num_discs, &mut a, &mut c, &mut b);
    moves
}
