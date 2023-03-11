use std::{io};
use std::env;
use lazy_static::lazy_static;
#[cfg(target_os = "linux")]
use minstant::Instant;

#[cfg(not(target_os = "linux"))]
use std::time::Instant;
use bitintr::{Tzcnt, Lzcnt, Andn};
use std::default;

static BASICSTART_CHESS_BOARD:[[char;8];8] = [
        ['r','n','b','q','k','b','n','r'],
        ['p','p','p','p','p','p','p','p'],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        ['P','P','P','P','P','P','P','P'],
        ['R','N','B','Q','K','B','N','R'],
    ];

#[derive(Debug)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}
#[derive(Clone, Copy)]
pub struct Game {
    pub wp : u64, pub wn : u64, pub wb : u64, pub wr : u64, pub wq : u64, pub wk : u64,
    pub bp : u64, pub bn : u64, pub bb : u64, pub br : u64, pub bq : u64, pub bk : u64,
    pub white_to_play : bool,
    wking_rook_never_move : bool,
    wqueen_rook_never_move : bool,
    wking_never_move : bool,
    bking_rook_never_move : bool,
    bqueen_rook_never_move : bool,
    bking_never_move : bool,
}
impl Game {
    pub fn occupied(&self) -> u64 {
        self.wp | self.wn | self.wb | self.wr | self.wq | self.wk | self.bp | self.bn | self.bb | self.br | self.bq | self.bk
    }
    pub fn white(&self) -> u64 {
        self.wp | self.wn | self.wb | self.wr | self.wq | self.wk
    }
    pub fn black(&self) -> u64 {
        self.bp | self.bn | self.bb | self.br | self.bq | self.bk
    }
    
}
impl Default for Game {
    fn default() -> Self { 
        get_game_from_basicpos()
    }
}

pub fn convert_square_to_move(a_move : u64) -> String{
    let b = (a_move / 8) as u8;
    let a:u8 = (a_move % 8) as u8;
    let f = (b'a' + a ) as char;
    let mut a = String::from(f);
    a.push((48 + b+1 ) as char );
    a
}
static FILE_A:u64 = 72340172838076673;
static FILE_B:u64 = 144680345676153340;
static FILE_H:u64 = 9259542123273814000;
static FILE_G:u64 = 4629771061636907000;
static FILE_AB:u64 = FILE_A | FILE_B;
static FILE_GH:u64 = FILE_G | FILE_H;

static RANK_MASK : [u64;8] = [
    255, 65280, 16711680, 4278190080, 1095216660480, 280375465082880, 71776119061217280, 18374686479671624000
];
static FILE_MASKS : [u64;8] = [
    0x101010101010101, 0x202020202020202, 0x404040404040404, 0x808080808080808,
    0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080
];
/*
static FILE_MASKS : [u64;8] = [
    72340172838076670, 144680345676153340, 289360691352306700, 578721382704613400,
    1157442765409226800, 2314885530818453500, 4629771061636907000, 9259542123273814000
];*/
static DIAG_MASKS : [u64;15] = [
    0x1, 0x102, 0x10204, 0x1020408, 0x102040810, 0x10204081020, 0x1020408102040,
	0x102040810204080, 0x204081020408000, 0x408102040800000, 0x810204080000000,
	0x1020408000000000, 0x2040800000000000, 0x4080000000000000, 0x8000000000000000
];
static ANTIDIAG_MASKS : [u64;15] = [
    0x80, 0x8040, 0x804020, 0x80402010, 0x8040201008, 0x804020100804, 0x80402010080402,
	0x8040201008040201, 0x4020100804020100, 0x2010080402010000, 0x1008040201000000,
	0x804020100000000, 0x402010000000000, 0x201000000000000, 0x100000000000000
];

lazy_static! {
    static ref FIRST_RANK_ATTACKS: [[u64; 8]; 64] = {
        let mut first_rank_attacks = [[0; 8]; 64];
        for o in 0..64 {
            for f in 0..8 {
                first_rank_attacks[o][f] = 0;

                for i in (f + 1)..8 {
                    first_rank_attacks[o][f] |= 1 << i;
                    if (o << 1) & (1 << i) > 0 {
                        break;
                    }
                }
                for i in (0..f).rev() {
                    first_rank_attacks[o][f] |= 1 << i;
                    if (o << 1) & (1 << i) > 0 {
                        break;
                    }
                }
            }
        }

        first_rank_attacks
    };
}

#[allow(clippy::too_many_arguments)]
pub fn array_to_bitboard(chessboard : [[char;8]; 8], wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) {
    let mut i = 0;
    for v in chessboard {
        for c in v {
            match c {
                'p' => { *wp += convert_string_to_bitboard(i); },
                'n' => { *wn += convert_string_to_bitboard(i); },
                'b' => { *wb += convert_string_to_bitboard(i); },
                'r' => { *wr += convert_string_to_bitboard(i); },
                'q' => { *wq += convert_string_to_bitboard(i); },
                'k' => { *wk += convert_string_to_bitboard(i); },
                'P' => { *bp += convert_string_to_bitboard(i); },
                'N' => { *bn += convert_string_to_bitboard(i); },
                'B' => { *bb += convert_string_to_bitboard(i); },
                'R' => { *br += convert_string_to_bitboard(i); },
                'Q' => { *bq += convert_string_to_bitboard(i); },
                'K' => { *bk += convert_string_to_bitboard(i); },
                _ => {}
            }
            i+=1;
        }
    }
}
pub fn get_game_from_basicpos() -> Game {
    let mut wp : u64 = 0;
    let mut wn : u64 = 0;
    let mut wb : u64 = 0;
    let mut wr : u64 = 0;
    let mut wq : u64 = 0;
    let mut wk : u64 = 0;
    let mut bp : u64 = 0;
    let mut bn : u64 = 0;
    let mut bb : u64 = 0;
    let mut br : u64 = 0;
    let mut bq : u64 = 0;
    let mut bk : u64 = 0;

    array_to_bitboard(BASICSTART_CHESS_BOARD, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    
    Game {
        wp, wn, wb, wr, wq, wk,
        bp, bn, bb, br, bq, bk,
        white_to_play : true,
        wking_never_move : true, wqueen_rook_never_move : true, wking_rook_never_move : true,
        bking_never_move : true, bqueen_rook_never_move : true, bking_rook_never_move : true,
    }
}
pub fn _draw_bitboard(bitboard : u64) {
    let mut i = 0;
    for _k in 0..8 {
        println!();
        for _p in 0..8 {
            print!("{}", bitboard>>i & 1);
            i+=1;
        }
    }
    println!();
}
pub fn _count_bit(mut bit : u64) -> i8 {
    let mut count = 0;
    while bit != 0 {
        bit &= bit-1;
        count+=1;
    }
    count
}
#[allow(clippy::too_many_arguments)]
pub fn draw_board(game : &Game) {
    let mut chess_board:[[char;8];8] = [[' ';8];8];
    let mut i = 0;
    for x in &mut chess_board {
        for c in x {
            if ((game.wp >> i) & 1) == 1  { *c = 'P'; }
            if ((game.wn >> i) & 1) == 1  { *c = 'N'; }
            if ((game.wb >> i) & 1) == 1  { *c = 'B'; }
            if ((game.wr >> i) & 1) == 1  { *c = 'R'; }
            if ((game.wq >> i) & 1) == 1  { *c = 'Q'; }
            if ((game.wk >> i) & 1) == 1  { *c = 'K'; }
            if ((game.bp >> i) & 1) == 1  { *c = 'p'; }
            if ((game.bn >> i) & 1) == 1  { *c = 'n'; }
            if ((game.bb >> i) & 1) == 1  { *c = 'b'; }
            if ((game.br >> i) & 1) == 1  { *c = 'r'; }
            if ((game.bq >> i) & 1) == 1  { *c = 'q'; }
            if ((game.bk >> i) & 1) == 1  { *c = 'k'; }
            i+=1;
        }
    }
    let letter = 'a';
    print!("     ");
    for i in 0..8 {
        print!("  {} ", (letter as u8+i) as char);
    }
    println!();
    
    for (i, x) in chess_board.iter().enumerate() {
        println!("     ---------------------------------");
        print!("   {} ", i+1);
        for c in x {
            print!("| {c} ");
        }
        println!("|");
    }
    println!("     ---------------------------------");
}
pub fn convert_string_to_bitboard(binary:usize) -> u64 {
    //u64::pow(2, (binary) as u32)
    1<<binary
}
/*pub fn possibility_wp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let empty = !(black | white);
    let pmoves1 = *wp<<7 & black & !RANK_MASK[7] & !FILE_MASKS[0];
    let pmoves2 = *wp<<9 & black & !RANK_MASK[7] & !FILE_MASKS[7];
    let pmoves3 = *wp<<8 & empty & !RANK_MASK[7];
    let pmoves4 = *wp<<16 & empty & (empty<<8) & RANK_MASK[3];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}*/
pub fn possibility_wp(wpawn : u64, empty : u64, black : u64) -> u64 {
    let pmoves1 = wpawn<<7 & black & !RANK_MASK[7] & !FILE_MASKS[0];
    let pmoves2 = wpawn<<9 & black & !RANK_MASK[7] & !FILE_MASKS[7];
    let pmoves3 = wpawn<<8 & empty & !RANK_MASK[7];
    let pmoves4 = wpawn<<16 & empty & (empty<<8) & RANK_MASK[3];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}
pub fn attack_wp(wpawn : u64, black : u64) -> u64 {
    let pmoves1 = wpawn<<7 & black & !RANK_MASK[7] & !FILE_MASKS[0];
    let pmoves2 = wpawn<<9 & black & !RANK_MASK[7] & !FILE_MASKS[7];
    pmoves1 | pmoves2
}
pub fn attack_bp(wpawn : u64, white : u64) -> u64 {
    let pmoves1 = wpawn>>7 & white & !RANK_MASK[7] & !FILE_MASKS[0];
    let pmoves2 = wpawn>>9 & white & !RANK_MASK[7] & !FILE_MASKS[7];
    pmoves1 | pmoves2
}
/*pub fn _possibility_bp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let empty = !(black | white);

    let pmoves1 = *bp>>7 & white & !RANK_MASK[0] & !FILE_MASKS[7];
    let pmoves2 = *bp>>9 & white & !RANK_MASK[0] & !FILE_MASKS[0];
    let pmoves3 = *bp>>8 & empty & !RANK_MASK[0];
    let pmoves4 = *bp>>16 & empty & (empty>>8) & RANK_MASK[4];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}*/
pub fn possibility_bp2( bpawn: u64, empty : u64, white : u64) -> u64 {

    let pmoves1 = bpawn>>7 & white & !RANK_MASK[0] & !FILE_MASKS[7];
    let pmoves2 = bpawn>>9 & white & !RANK_MASK[0] & !FILE_MASKS[0];
    let pmoves3 = bpawn>>8 & empty & !RANK_MASK[0];
    let pmoves4 = bpawn>>16 & empty & (empty>>8) & RANK_MASK[4];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}

pub fn possibility_n(knight : u64) -> u64 {
    let nonoea:u64 =  (knight << 17) & !FILE_A ;
    let noeaea:u64 =  (knight << 10) & !FILE_AB;
    let soeaea:u64 =  (knight >>  6) & !FILE_AB;
    let sosoea:u64 =  (knight >> 15) & !FILE_A ;
    let nonowe:u64 =  (knight << 15) & !FILE_H ;
    let nowewe:u64 =  (knight <<  6) & !FILE_GH;
    let sowewe:u64 =  (knight >> 10) & !FILE_GH;
    let sosowe:u64 =  (knight >> 17) & !FILE_H ;
    nonoea | noeaea | soeaea | sosoea | nonowe | nowewe | sowewe | sosowe
}

pub fn possibility_k(mut wk : u64) -> u64 {
    let mut attack = wk<<1 | wk>>1;
    wk |= attack;
    attack |= wk<<8 | wk>>8;
    attack
}

pub fn hyperbola_quintessence(occupied : u64, mask: u64, mut number : u64) -> u64 {
    number = 1<<number;
    let mut forward = occupied & mask ;
    let mut reverse = forward.swap_bytes();

    forward = forward.wrapping_sub(number.wrapping_mul(2));
    reverse = reverse.wrapping_sub(number.swap_bytes().wrapping_mul(2));
    forward ^= reverse.swap_bytes();
    forward & mask
    //( - 2 * number) ^ ((occupied & mask).swap_bytes() - 2 * number.swap_bytes()).swap_bytes()
    //(occupied - 2 * number) ^ (occupied.reverse_bits() - 2 * number.reverse_bits()).reverse_bits()
}
pub fn rank_attacks(occupied: u64, sq: u64) -> u64 {
    
    let f = sq & 7; // sq.file() as Bitboard;
    let r = sq & !7; // (sq.rank() * 8) as Bitboard;
    let o = (occupied >> (r + 1)) & 63;
    FIRST_RANK_ATTACKS[o as usize][f as usize] << r
}
pub fn convert_move_to_bitboard(moves : &str) -> (u64, u64) {
    
    let mut iter1 = moves[0..4].chars();
    let un = iter1.next().unwrap() as u64-96;
    let deux = iter1.next().unwrap() as u64-48;
    let trois = iter1.next().unwrap() as u64-96;
    let quatre = iter1.next().unwrap() as u64-48;
    let a = (deux-1) *8 +  un-1 ;
    let b = (quatre-1) *8 +  trois-1;
    (a,b)
}

pub fn compute_move_w(mut a:u64, mut b:u64, game : &mut Game) -> bool {
    let black = game.bp | game.bn | game.bb | game.br | game.bq | game.bk;
    let white = game.wp | game.wn | game.wb | game.wr | game.wq | game.wk;
    let occupied = black | white;
    
    let square_a = a;
    let square_b = b;
    a = 1<<a;
    b = 1<<b;
    let mut moves= 0;
    let mut from: &mut u64 = &mut 0;
    if ((game.wp) & a) != 0 {
        moves = possibility_wp(a, !occupied, black);
        from = &mut game.wp;
    }
    else if game.wn & a != 0 {
        moves = possibility_n(game.wn & a) & !white;
        from = &mut game.wn;
    }
    else if game.wb & a != 0 {
        let occupied = black | white;
        moves = diag_antid_moves(square_a, occupied);
        from = &mut game.wb;
    }
    else if game.wr & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied);
        from = &mut game.wr;
    }
    else if game.wq & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) | diag_antid_moves(square_a, occupied);
        from = &mut game.wq;
    }
    else if game.wk & a != 0 {
        //println!("{square_b} {} {} {}", game.wking_never_move, game.wking_rook_never_move, game.wqueen_rook_never_move);
        if square_b == 2 { // Grand roque
            //check if the king and the rook has never move
            if game.wking_never_move && game.wking_rook_never_move && (black | white) & (2u64.pow(1) + 2u64.pow(2)) == 0 && possibility_b(game) & (2u64.pow(58) + 2u64.pow(57)) == 0 {
                game.wking_never_move = false;
                game.wking_rook_never_move = false;
                //Do grand roque
                game.wk &= !a;
                game.wk |= b;
                game.wr &= !(2u64.pow(0));
                game.wr |= 2u64.pow(3);
                return true;
            }
            return false;
            //check if no piece is between
            //check if square between isn't attacked
        }
        else if square_b == 6 { //Petit Roque
            if game.wking_never_move && game.wqueen_rook_never_move && (black | white) & (2u64.pow(6) + 2u64.pow(5)) == 0 && possibility_b(game) & (2u64.pow(6) + 2u64.pow(5)) == 0 {
                game.wking_never_move = false;
                game.wqueen_rook_never_move = false;
                game.wk &= !a;
                game.wk |= b;
                game.wr &= !(2u64.pow(7));
                game.wr |= 2u64.pow(5);
                return true;
            }
            return false;
        }
        moves = possibility_k(game.wk) & !white;
        from = &mut game.wk;
    }
    if moves & b != 0 {
        (*from) &= !a;
        (*from) |= b;
        if black & b != 0 {
            if game.bp & b != 0 { game.bp &= !b; }
            else if game.bn & b != 0 { game.bn &= !b; }
            else if game.bb & b != 0 { game.bb &= !b; }
            else if game.br & b != 0 { game.br &= !b; }
            else if game.bq & b != 0 { game.bq &= !b; }
        }
        true
    }
    else {
        false
    }
}

pub fn diag_antid_moves(square : u64, occupied : u64) -> u64 {
    hyperbola_quintessence(occupied, DIAG_MASKS[((square/8) + (square%8)) as usize], square) | hyperbola_quintessence(occupied, ANTIDIAG_MASKS[((square/8)+7 - (square%8)) as usize], square)
}
pub fn hv_moves(square : u64, occupied : u64) -> u64 {
    
    let b = hyperbola_quintessence(occupied, FILE_MASKS[(square % 8) as usize], square);
    rank_attacks(occupied, square) | b
}
pub fn compute_move_b(mut a : u64, mut b: u64, game :&mut Game) -> bool {
//pub fn compute_move_b(mut a : u64, mut b: u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    let black = game.bp | game.bn | game.bb | game.br | game.bq | game.bk;
    let white = game.wp | game.wn | game.wb | game.wr | game.wq | game.wk;
    let square_a = a;
    let square_b = b;
    a = 1<<a;
    b = 1<<b;
    let mut moves = 0;
    let mut from = &mut (0);
    if ((game.bp) & a) != 0 {
        moves = possibility_bp2(a, !(black | white), white);
        from = &mut game.bp;
    }
    else if game.bn & a != 0 {
        moves = possibility_n( a) & !black;
        from = &mut game.bn;
    }
    else if game.bb & a != 0 {
        let occupied = black | white;
        moves = diag_antid_moves(square_a, occupied) & !black;
        from = &mut game.bb;
    }
    else if game.br & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) & !black;
        from = &mut game.br;
    }
    else if game.bq & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) | diag_antid_moves(square_a, occupied);
        from = &mut game.bq;
    }
    else if game.bk & a != 0 {
        println!("{square_b} {} {} {}", game.bking_never_move, (black | white) & (2u64.pow(61) + 2u64.pow(62)) == 0, possibility_w(game) & (2u64.pow(61) + 2u64.pow(62)) == 0);
        _draw_bitboard(possibility_w(game));
        if square_b == 58 { // Grand roque
            //check if the king and the rook has never move
            if game.bking_never_move && game.bking_rook_never_move && (black | white) & (2u64.pow(58) + 2u64.pow(57)) == 0 && possibility_w(game) & (2u64.pow(58) + 2u64.pow(57)) == 0 {
                //println!("Grand roque");
                game.bking_never_move = false;
                game.bking_rook_never_move = false;
                //Do grand roque
                game.bk &= !a;
                game.bk |= b;
                game.br &= !(2u64.pow(56));
                game.br |= 2u64.pow(59);
                return true;
            }
            return false;
            //check if no piece is between
            //check if square between isn't attacked
        }
        else if square_b == 62 { //Petit Roque
            if game.bking_never_move && game.bqueen_rook_never_move && (black | white) & (2u64.pow(61) + 2u64.pow(62)) == 0 && possibility_w(game) & (2u64.pow(61) + 2u64.pow(62)) == 0 {
                game.bking_never_move = false;
                game.bqueen_rook_never_move = false;
                game.bk &= !a;
                game.bk |= b;
                game.br &= !(2u64.pow(63));
                game.br |= 2u64.pow(61);
                return true;
            }
            return false;
        }
        moves = possibility_k(game.bk) & !black;
        from = &mut game.bk;
    }
    if moves & b != 0 {
        (*from) &= !a;
        (*from) |=  b;
        if white & b != 0 {
            if game.wp & b != 0 { game.wp &= !b; }
            else if game.wn & b != 0 { game.wn &= !b; }
            else if game.wb & b != 0 { game.wb &= !b; }
            else if game.wr & b != 0 { game.wr &= !b; }
            else if game.wq & b != 0 { game.wq &= !b; }
        }
        true
    }
    else {
        false
    }
}
pub fn possibility_w( game : &Game) -> u64 {
    let black = game.black();
    let white = game.white();
    let occupied = black | white;
    let mut attack = 0;
    attack |= attack_wp(game.wp, black);
    
    if game.wn != 0 {
        attack |= possibility_n(game.wn) & !white;
    }
    
    if game.wb != 0 {
        let devant = 63-(game.wb).leading_zeros();
        let arriere = (game.wb).trailing_zeros();
        attack |= diag_antid_moves(arriere as u64, occupied) & !white;
        if devant != arriere {
            attack |= diag_antid_moves(devant as u64, occupied) & !white;
        }
    }
    
    if game.wr != 0 {
        let devant = 63 - (game.wr).leading_zeros();
        let arriere = (game.wr).trailing_zeros();
        attack |= hv_moves(arriere as u64, occupied) & !white;
        if devant != arriere {
            attack |= hv_moves(devant as u64, occupied) & !white;
        }
    }
    
    if game.wq != 0 {
        attack |= (hv_moves(game.wq.tzcnt(), occupied) | diag_antid_moves(game.wq.tzcnt(), occupied)) & !white;
    }
    attack |= possibility_k(game.wk) & !white;
    
    attack
}
pub fn possibility_b( game : &Game) -> u64 {
    let black = game.bp | game.bn | game.bb | game.br | game.bq | game.bk;
    let white = game.wp | game.wn | game.wb | game.wr | game.wq | game.wk;
    let occupied = black | white;
    let mut attack = 0;

    attack |= attack_bp(game.wp, black);

    if game.bn != 0 {
        attack |= possibility_n(game.bn) & !black;
    }
    
    if game.bb != 0 {
        let devant = 63-(game.bb).leading_zeros();
        let arriere = (game.bb).trailing_zeros(); 
        attack |= diag_antid_moves(arriere as u64, occupied) & !black;
        if devant != arriere {
            attack |= diag_antid_moves(devant as u64, occupied) & !black;
        }
    }
    
    //attack |= possibility_bn(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
    if game.br != 0 {
        let devant = 63-(game.br).leading_zeros();
        let arriere = (game.br).trailing_zeros();
        attack |= hv_moves(arriere as u64, occupied) & !black;
        if devant != arriere {
            attack |= hv_moves(devant as u64, occupied) & !black;
        }
    }
    
    if game.bq != 0 {
        attack |= (hv_moves(game.bq.trailing_zeros() as u64, occupied) | diag_antid_moves(game.bq.trailing_zeros() as u64, occupied) ) & !black;
    }
    attack |= possibility_k(game.bk) & !black;
    attack
}
pub fn _copy_bitboard(wp:&u64, wn:&u64, wb:&u64, wr:&u64, wq:&u64, wk:&u64, bp:&u64, bn:&u64, bb:&u64, br:&u64, bq:&u64, bk:&u64) -> (u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64){
    (*wp, *wn, *wb, *wr, *wq, *wk, *bp, *bn, *bb, *br, *bq, *bk)
}

pub fn is_attacked(target_is_wking : bool, game : &Game) -> bool {
    if target_is_wking {
        possibility_b(game) & game.wk != 0
    }
    else {
        possibility_w(game) & game.bk != 0
    }
}

pub fn get_legal_move(side_w : bool, game : &Game) -> Vec<(u64, Piece)> {
    //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
    let black = game.bp | game.bn | game.bb | game.br | game.bq | game.bk;
    let white = game.wp | game.wn | game.wb | game.wr | game.wq | game.wk;
    let occupied = black | white;
    let mut legal_moves = Vec::<(u64, Piece)>::new();
    if side_w { //White Possibility
        //Pions Possibility
        let mut wp_test = game.wp;
        while  wp_test != 0 {
            let piece = wp_test.trailing_zeros() as u64;
            let wp_extract = 1u64 << piece;
            wp_test = wp_test & (wp_test-1);
            let mut possi_wp = possibility_wp(wp_extract, !(occupied), black);
            while possi_wp != 0 {
                let mut game1 = game.clone();
                let b = possi_wp.trailing_zeros() as u64;
                compute_move_w(piece, b, &mut game1);
                let is_check = is_attacked(true, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::PAWN));
                }
                possi_wp = possi_wp & (possi_wp - 1);
            }
        }
        //Knight
        //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
        let mut wn_test = game.wn;
        while wn_test != 0 {
            let piece = wn_test.tzcnt();
            let wn_extract = (1 << piece) as u64;
            wn_test = wn_test & (wn_test - 1);
            let mut wn_possi = possibility_n( wn_extract) & !white;
            while wn_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = wn_possi.tzcnt();
                compute_move_w(piece, b, &mut game1);
                let is_check = is_attacked(true, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::KNIGHT));
                }
                wn_possi = wn_possi & (wn_possi - 1);
            }
        }
        
        //Bishop
        let mut wb_test = game.wb;
        while wb_test != 0 {
            let piece = wb_test.tzcnt();
            wb_test = wb_test & (wb_test - 1);
            let mut wb_possi = diag_antid_moves(piece, occupied) & !white;
            while wb_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = wb_possi.tzcnt();
                compute_move_w(piece, b, &mut game1);
                let is_check = is_attacked(true, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                wb_possi = wb_possi & (wb_possi - 1);
            }
        }
        //Rook
        let mut wr_test = game.wr;
        while wr_test != 0 {
            let piece = wr_test.tzcnt();
            wr_test = wr_test & (wr_test - 1);
            let mut wr_possi = hv_moves(piece, occupied) & !white;
            while wr_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = wr_possi.tzcnt();
                compute_move_w(piece, b, &mut game1);
                let is_check = is_attacked(true, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                wr_possi = wr_possi & (wr_possi - 1);
            }
        }

        //Queen
        if game.wq != 0 {
            let piece = game.wq.tzcnt();
            let mut wq_possi = (hv_moves(piece, occupied) | diag_antid_moves(piece, occupied)) & !white;
            while wq_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = wq_possi.tzcnt();
                compute_move_w(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::QUEEN));
                }
                wq_possi = wq_possi & (wq_possi - 1);
            }
        }
        //King
        let mut possi_wk = possibility_k(game.wk) & !white;
        while possi_wk != 0 {
            //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
            let mut game1 = game.clone();
            let b = possi_wk.tzcnt();
            compute_move_w((game.wk).trailing_zeros() as u64, b, &mut game1);
            let is_check = is_attacked(false, &game1);
            if !is_check {
                legal_moves.push((((game.bk.trailing_zeros() as u64)<<8) + b, Piece::KING));
            }
            possi_wk = possi_wk & (possi_wk - 1);
        }
    }
    else { //Black Possiblity
        //Pions Possibility
        
        /*
        let possi_wp = possibility_w(&mut devant, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        */
        let mut bp_test = game.bp;
        while  bp_test != 0 {
            let piece = bp_test.trailing_zeros() as u64;
            let bp_extract = 1u64 << piece;
            bp_test = bp_test & (bp_test-1);
            let mut possi_bp = possibility_bp2(bp_extract, !(occupied), white);
            while possi_bp != 0 {
                let mut game1 = game.clone();
                let b = possi_bp.trailing_zeros() as u64;
                compute_move_b(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::PAWN));
                }
                possi_bp = possi_bp & (possi_bp - 1);
            }
        }
        //Knight
        //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
        let mut bn_test = game.bn;
        while bn_test != 0 {
            let piece = bn_test.trailing_zeros() as u64;
            let bn_extract = (1u64) << piece;
            bn_test = bn_test & (bn_test-1);
            let mut bn_possi = possibility_n(bn_extract) & !black;
            while bn_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = bn_possi.trailing_zeros() as u64;
                compute_move_b(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::KNIGHT));
                }
                bn_possi = bn_possi & (bn_possi - 1);
            }
        }
        
        //Bishop
        let mut bb_test = game.bb;
        while bb_test != 0 {
            let piece = bb_test.tzcnt();
            bb_test = bb_test & (bb_test - 1);
            let mut bb_possi = diag_antid_moves(piece, occupied) & !black;
            while bb_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = bb_possi.tzcnt();
                compute_move_b(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                bb_possi = bb_possi & (bb_possi - 1);
            }
        }
        //Rook
        let mut br_test = game.br;
        while br_test != 0 {
            let piece = br_test.tzcnt();
            br_test = br_test & (br_test - 1);
            let mut br_possi = hv_moves(piece, occupied) & !black;
            while br_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = br_possi.tzcnt();
                compute_move_b(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                br_possi = br_possi & (br_possi - 1);
            }
        }

        //Queen
        if game.bq != 0 {
            let piece = game.bq.tzcnt();
            let mut bq_possi = (hv_moves(piece, occupied) | diag_antid_moves(piece, occupied)) & !black;
            while bq_possi != 0 {
                //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let mut game1 = game.clone();
                let b = bq_possi.tzcnt();
                compute_move_b(piece, b, &mut game1);
                let is_check = is_attacked(false, &game1);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                bq_possi = bq_possi & (bq_possi - 1);
            }
        }
        
        //King
        let mut possi_bk = possibility_k(game.bk) & !black;
        while possi_bk != 0 {
            //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
            let mut game1 = game.clone();
            let b = possi_bk.tzcnt();
            compute_move_b(game.bk.trailing_zeros() as u64, b, &mut game1);
            let is_check = is_attacked(false, &game1);
            if !is_check {
                legal_moves.push((((game.bk.trailing_zeros() as u64)<<8) + b, Piece::KING));
            }
            possi_bk = possi_bk & (possi_bk - 1);
        }
    }
    legal_moves
}

pub fn print_custum_move(a_move : (u64,Piece)) {
    let a = a_move.0>>8;
    let b = a_move.0 & 255;
    println!("{}{} {:?}", convert_square_to_move(a), convert_square_to_move(b), a_move.1);
}
pub fn convert_custum_move(the_move : (u64, Piece)) -> (u64, u64) {
    (the_move.0>>8, the_move.0 & 255)
}
fn main() {
    let now = Instant::now();
    println!("Instant init : {} nano seconde", now.elapsed().as_nanos());
    env::set_var("RUST_BACKTRACE", "1");
    let chess_board:[[char;8];8] = [
        ['r','n','b','q','k','b','n','r'],
        ['p','p','p','p','p','p','p','p'],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        [' ',' ',' ',' ',' ',' ',' ',' '],
        ['P','P','P','P','P','P','P','P'],
        ['R','N','B','Q','K','B','N','R'],
    ];
    
    let mut wp : u64 = 0;
    let mut wn : u64 = 0;
    let mut wb : u64 = 0;
    let mut wr : u64 = 0;
    let mut wq : u64 = 0;
    let mut wk : u64 = 0;
    let mut bp : u64 = 0;
    let mut bn : u64 = 0;
    let mut bb : u64 = 0;
    let mut br : u64 = 0;
    let mut bq : u64 = 0;
    let mut bk : u64 = 0;

    array_to_bitboard(chess_board, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    
    let mut game = Game {
        wp, wn, wb, wr, wq, wk,
        bp, bn, bb, br, bq, bk,
        white_to_play : true,
        wking_never_move : true, wqueen_rook_never_move : true, wking_rook_never_move : true,
        bking_never_move : true, bqueen_rook_never_move : true, bking_rook_never_move : true,
    };
    
    //let moves = ["e2e3", "e7e6", "f1d3", "d8g5"];
    //let moves = ["b1c3","g8f6", "c3b1"];
    //let moves = ["e2e4","e7e5", "f2f4", "d2d4", "d7d5", "f1e2", "d8d6" ];
    //let moves = ["e2e4", "e7e5", "f1e2"];
    //let moves = ["e2e4", "e7e5", "d1h5", "b8c6", "h5f7"]; //Just Check
    //let moves = ["e2e4", "e7e5", "f1c4", "b8c6", "d1h5", "g8f6", "h5f7"]; //Bergé
    let moves = ["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5", "e1g1", "g8f6", "d1e2", "e8g8"]; // Test ROQUE
    draw_board(&game);
    //let now = Instant::now();
    for m in moves {
    //loop {
        //let mut m = String::new();
        if game.white_to_play { println!("WHITE : "); }
        else { println!("BLACK : "); }
        let legal = get_legal_move(game.white_to_play, &game);
        for x in legal {
            print_custum_move(x);
        }
        //io::stdin().read_line(&mut m).unwrap();
        println!("MOVE {m}");
        let (a,b) = convert_move_to_bitboard(&m);
        //println!("{a} {b}");
        let now = Instant::now();
        
        let response = if game.white_to_play {
            compute_move_w(a, b, &mut game)
        }
        else {
            compute_move_b(a, b, &mut game)
        };
        
        game.white_to_play ^= response;
        
        let k_attacked =  is_attacked(game.white_to_play, &game);
        //let legal = get_legal_move(game.white_to_play, &game);
        println!(" {} nano seconde", now.elapsed().as_nanos());        
        if k_attacked {
            print!("CHECK");
            if get_legal_move(game.white_to_play, &game).is_empty() {
                print!(" MATE");
            }
            println!();
        }
        draw_board(&game);
        println!("{response}");
    }
    
    //println!("{} nano seconde", now.elapsed().as_nanos());
    //draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
}
