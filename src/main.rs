use std::{io, fmt::Error};
use lazy_static::lazy_static;
#[cfg(target_os = "linux")]
use minstant::Instant;

#[cfg(not(target_os = "linux"))]
use std::time::Instant;
use bitintr::{Tzcnt, Lzcnt, Andn};
#[derive(Debug)]
enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

fn convert_square_to_move(a_move : u64) -> String{
    let b = (a_move / 8) as u8;
    let a:u8 = (a_move % 8) as u8;
    let f = ('a' as u8 + a ) as char;
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
fn array_to_bitboard(chessboard : [[char;8]; 8], wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) {
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
fn draw_bitboard(bitboard : u64) {
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
pub fn count_bit(mut bit : u64) -> i8 {
    let mut count = 0;
    while bit != 0 {
        bit &= bit-1;
        count+=1;
    }
    count
}
#[allow(clippy::too_many_arguments)]
fn draw_board(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) {
    let mut chess_board:[[char;8];8] = [[' ';8];8];
    let mut i = 0;
    for x in &mut chess_board {
        for c in x {
            if ((*wp >> i) & 1) == 1  { *c = 'P'; }
            if ((*wn >> i) & 1) == 1  { *c = 'N'; }
            if ((*wb >> i) & 1) == 1  { *c = 'B'; }
            if ((*wr >> i) & 1) == 1  { *c = 'R'; }
            if ((*wq >> i) & 1) == 1  { *c = 'Q'; }
            if ((*wk >> i) & 1) == 1  { *c = 'K'; }
            if ((*bp >> i) & 1) == 1  { *c = 'p'; }
            if ((*bn >> i) & 1) == 1  { *c = 'n'; }
            if ((*bb >> i) & 1) == 1  { *c = 'b'; }
            if ((*br >> i) & 1) == 1  { *c = 'r'; }
            if ((*bq >> i) & 1) == 1  { *c = 'q'; }
            if ((*bk >> i) & 1) == 1  { *c = 'k'; }
            i+=1;
        }
    }
    let letter = 'a';
    print!("     ");
    for i in 0..8 {
        print!("  {} ", (letter as u8+i) as char);
    }
    print!("\n");
    
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
fn convert_string_to_bitboard(binary:usize) -> u64 {
    //u64::pow(2, (binary) as u32)
    1<<binary
}
fn possibility_wp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let empty = !(black | white);
    let pmoves1 = *wp<<7 & black & !RANK_MASK[7] & !FILE_MASKS[0];
    let pmoves2 = *wp<<9 & black & !RANK_MASK[7] & !FILE_MASKS[7];
    let pmoves3 = *wp<<8 & empty & !RANK_MASK[7];
    let pmoves4 = *wp<<16 & empty & (empty<<8) & RANK_MASK[3];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}
fn possibility_bp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let empty = !(black | white);

    let pmoves1 = *bp>>7 & white & !RANK_MASK[0] & !FILE_MASKS[7];
    let pmoves2 = *bp>>9 & white & !RANK_MASK[0] & !FILE_MASKS[0];
    let pmoves3 = *bp>>8 & empty & !RANK_MASK[0];
    let pmoves4 = *bp>>16 & empty & (empty>>8) & RANK_MASK[4];
    pmoves1 | pmoves2 | pmoves3 | pmoves4
}
fn possibility_wn(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    //let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;

    let nonoea:u64 =  (*wn << 17) & !FILE_A ;
    let noeaea:u64 =  (*wn << 10) & !FILE_AB;
    let soeaea:u64 =  (*wn >>  6) & !FILE_AB;
    let sosoea:u64 =  (*wn >> 15) & !FILE_A ;
    let nonowe:u64 =  (*wn << 15) & !FILE_H ;
    let nowewe:u64 =  (*wn <<  6) & !FILE_GH;
    let sowewe:u64 =  (*wn >> 10) & !FILE_GH;
    let sosowe:u64 =  (*wn >> 17) & !FILE_H ;
    (nonoea | noeaea | soeaea | sosoea | nonowe | nowewe | sowewe | sosowe) & !white
}
fn possibility_bn(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    //let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let nonoea:u64 =  (*bn << 17) & !FILE_A ;
    let noeaea:u64 =  (*bn << 10) & !FILE_AB;
    let soeaea:u64 =  (*bn >>  6) & !FILE_AB;
    let sosoea:u64 =  (*bn >> 15) & !FILE_A ;
    let nonowe:u64 =  (*bn << 15) & !FILE_H ;
    let nowewe:u64 =  (*bn <<  6) & !FILE_GH;
    let sowewe:u64 =  (*bn >> 10) & !FILE_GH;
    let sosowe:u64 =  (*bn >> 17) & !FILE_H ;
    (nonoea | noeaea | soeaea | sosoea | nonowe | nowewe | sowewe | sosowe) & !black
}

fn possibility_k(mut wk : u64) -> u64 {
    let mut attack = wk<<1 | wk>>1;
    wk |= attack;
    attack |= wk<<8 | wk>>8;
    attack
}

fn hyperbola_quintessence(occupied : u64, mask: u64, mut number : u64) -> u64 {
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
fn rank_attacks(occupied: u64, sq: u64) -> u64 {
    
    let f = sq & 7; // sq.file() as Bitboard;
    let r = sq & !7; // (sq.rank() * 8) as Bitboard;
    let o = (occupied >> (r + 1)) & 63;
    FIRST_RANK_ATTACKS[o as usize][f as usize] << r
}
fn convert_move_to_bitboard(moves : &str) -> (u64,u64) {
    /*if moves.len() != 4 {
        return;
    }*/
    let mut iter1 = moves[0..4].chars();
    let un = iter1.next().unwrap() as u64-96;
    let deux = iter1.next().unwrap() as u64-48;
    let trois = iter1.next().unwrap() as u64-96;
    let quatre = iter1.next().unwrap() as u64-48;
    let a = (deux-1) *8 +  un-1 ;
    let b = (quatre-1) *8 +  trois-1;
    (a,b)
}

fn compute_move_w(mut a:u64, mut b:u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let square_a = a;
    a = (1 as u64)<<a;
    b = 1<<b;
    let mut moves= 0;
    let mut from: &mut u64 = &mut 0;
    if ((*wp) & a) != 0 {
        let mut p = (*wp) & a;
        moves = possibility_wp(&mut p, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        from = wp;
    }
    else if *wn & a != 0 {
        moves = possibility_wn(wp, &mut (*wn & a), wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        from = wn;
    }
    else if *wb & a != 0 {
        let occupied = black | white;
        moves = diag_antid_moves(square_a, occupied);
        from = wb;
    }
    else if *wr & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied);
        from = wr;
    }
    else if *wq & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) | diag_antid_moves(square_a, occupied);
        from = wq;
    }
    else if *wk & a != 0 {
        moves = possibility_k(*wk) & !white;
        from = wk;
    }
    if moves & b != 0 {
        (*from) &= !a;
        (*from) |= b;
        if black & b != 0 {
            if *bp & b != 0 { *bp &= !b; }
            else if *bn & b != 0 { *bn &= !b; }
            else if *bb & b != 0 { *bb &= !b; }
            else if *br & b != 0 { *br &= !b; }
            else if *bq & b != 0 { *bq &= !b; }
        }
        true
    }
    else {
        false
    }
}

fn diag_antid_moves(square : u64, occupied : u64) -> u64 {
    let a = hyperbola_quintessence(occupied, DIAG_MASKS[((square/8) + (square%8)) as usize], square) | hyperbola_quintessence(occupied, ANTIDIAG_MASKS[((square/8)+7 - (square%8)) as usize], square);
    //draw_bitboard(a);
    a
}
fn hv_moves(square : u64, occupied : u64) -> u64 {
    let b = hyperbola_quintessence(occupied, FILE_MASKS[(square % 8) as usize], square);
    rank_attacks(occupied, square) | b
}
fn compute_move_b(mut a:u64, mut b:u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let square_a = a;
    a = 1<<a;
    b = 1<<b;
    let mut moves = 0;
    let mut from = &mut (0) ;
    if ((*bp) & a) != 0 {
        let mut p = (*bp) & a;
        moves = possibility_bp(wp, wn, wb, wr, wq, wk, &mut p, bn, bb, br, bq, bk);
        from = bp;
    }
    else if *bn & a != 0 {
        moves = possibility_bn(wp, &mut (*wn & a), wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        from = bn;
    }
    else if *bb & a != 0 {
        let occupied = black | white;
        moves = diag_antid_moves(square_a, occupied) & !black;
        from = bb;
    }
    else if *br & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) & !black;
        from = br;
    }
    else if *bq & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) | diag_antid_moves(square_a, occupied);
        //draw_bitboard(moves);
        //moves &= !black;
        from = bq;
    }
    else if *bk & a != 0 {
        moves = possibility_k(*bk) & !black;
        from = bk;
    }
    if moves & b != 0 {
        (*from) &= !a;
        (*from) |= b;
        if white & b != 0 {
            if *wp & b != 0 { *wp &= !b; }
            else if *wn & b != 0 { *wn &= !b; }
            else if *wb & b != 0 { *wb &= !b; }
            else if *wr & b != 0 { *wr &= !b; }
            else if *wq & b != 0 { *wq &= !b; }
        }
        true
    }
    else {
        false
    }
}
fn possibility_w( wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let occupied = black | white;
    let mut attack = 0;
    attack |= possibility_wp(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
    //attack |= possibility_wn(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
    //let devant = (*wb).lzcnt();
    //let arriere = (*wb).tzcnt();
    let devant = 63-(*wb).leading_zeros();
    let arriere = (*wb).trailing_zeros();
    
    attack |= diag_antid_moves(arriere as u64, occupied) & !white;
    if devant != arriere {
        attack |= diag_antid_moves(devant as u64, occupied) & !white;
    }
    
    /*let devant = (*wr).clz();
    let arriere = (*wr).tzcnt();*/
    let devant = 63 - (*wr).leading_zeros();
    let arriere = (*wr).trailing_zeros();
    attack |= hv_moves(arriere as u64, occupied) & !white;
    if devant != arriere {
        attack |= hv_moves(devant as u64, occupied) ;
    }
    attack |= (hv_moves(wq.tzcnt() as u64, occupied) | diag_antid_moves(wq.tzcnt() as u64, occupied)) & !white;
    attack
}
fn possibility_b( wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64{
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let occupied = black | white;
    let mut attack = 0;
    attack |= possibility_bp(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
    attack |= possibility_bn(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk) & !black;
    let devant = 63-(*bb).leading_zeros();
    let arriere = (*bb).trailing_zeros();

    attack |= diag_antid_moves(arriere as u64, occupied) & !black;
    if devant != arriere {
        attack |= diag_antid_moves(devant as u64, occupied) & !black;
    }
    //attack |= possibility_bn(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
    let devant = 63-(*br).leading_zeros();
    let arriere = (*br).trailing_zeros();
    attack |= hv_moves(arriere as u64, occupied) & !black;
    if devant != arriere {
        attack |= hv_moves(devant as u64, occupied) & !black;
    }
    attack |= (hv_moves(bq.trailing_zeros() as u64, occupied) | diag_antid_moves(bq.trailing_zeros() as u64, occupied) ) & !black;
    attack
}
fn copy_bitboard(wp:&u64, wn:&u64, wb:&u64, wr:&u64, wq:&u64, wk:&u64, bp:&u64, bn:&u64, bb:&u64, br:&u64, bq:&u64, bk:&u64) -> (u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64){
    (*wp, *wn, *wb, *wr, *wq, *wk, *bp, *bn, *bb, *br, *bq, *bk)
}

fn is_attacked(target_is_w : bool, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    if target_is_w {
        let attacks = possibility_w(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        //draw_bitboard(attacks);
        attacks & *bk != 0
    }
    else {
        possibility_b(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk) & *wk != 0
    }
}
fn get_legal_move(side_w : bool, wp1:&mut u64, wn1:&mut u64, wb1:&mut u64, wr1:&mut u64, wq1:&mut u64, wk1:&mut u64, bp1:&mut u64, bn1:&mut u64, bb1:&mut u64, br1:&mut u64, bq1:&mut u64, bk1:&mut u64) -> Vec<(u64, Piece)> {
    //let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
    let black = *bp1 | *bn1 | *bb1 | *br1 | *bq1 | *bk1;
    let white = *wp1 | *wn1 | *wb1 | *wr1 | *wq1 | *wk1;
    let occupied = black | white;
    let mut legal_moves = Vec::<(u64, Piece)>::new();
    if side_w { //White Possibility
        //Pions Possibility
        let mut devant = 0;
        let mut derriere = 0;
        /*let possi_wp = possibility_w(&mut devant, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        */
        //Knight
        let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
        let mut wn_test = *wn1;
        while wn_test != 0 {
            let piece = wn_test.tzcnt();
            let mut wn_extract = (1 << piece) as u64;
            wn_test = wn_test & wn_test - 1;
            let mut wn_possi = possibility_wn(&mut wp, &mut wn_extract, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk) & !white;
            while wn_possi != 0 {
                let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let b = wn_possi.tzcnt();
                compute_move_w(piece, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                let is_check = is_attacked(true, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::KNIGHT));
                }
                wn_possi = wn_possi & wn_possi - 1;
            }
        }
        
        //Bishop
        let mut wb_test = *wb1;
        while wb_test != 0 {
            let piece = wb_test.tzcnt();
            wb_test = wb_test & wb_test - 1;
            let mut wb_possi = diag_antid_moves(piece, occupied) & !white;
            while wb_possi != 0 {
                let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let b = wb_possi.tzcnt();
                compute_move_w(piece, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                let is_check = is_attacked(true, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                wb_possi = wb_possi & wb_possi - 1;
            }
        }
        //Rook
        devant = wr.leading_zeros() as u64;
        derriere = wr.leading_zeros() as u64;
        let possi_wr = hv_moves(devant, occupied);
        let possi_wr2 = if devant != derriere {
            hv_moves(derriere, occupied)
        } else { 0 };

        //Queen
        let queen_pos = wq.leading_zeros();
        let possi_wq = hv_moves(queen_pos as u64, occupied) | diag_antid_moves(queen_pos as u64, occupied);
        
        //King
        let possi_wk = possibility_k(wk);
    }
    else { //Black Possiblity
        //Pions Possibility
        let mut devant = 0;
        let mut derriere = 0;
        /*let possi_wp = possibility_w(&mut devant, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        */
        //Knight
        let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
        let mut bn_test = *bn1;
        while bn_test != 0 {
            let piece = bn_test.trailing_zeros() as u64;
            let mut bn_extract = ((1 as u64) << piece) as u64;
            bn_test = bn_test & bn_test-1;
            let mut bn_possi = possibility_wn(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn_extract, &mut bb, &mut br, &mut bq, &mut bk) & !black;
            while bn_possi != 0 {
                let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let b = bn_possi.trailing_zeros() as u64;
                compute_move_b(piece, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                let is_check = is_attacked(false, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::KNIGHT));
                }
                bn_possi = bn_possi & bn_possi - 1;
            }
        }
        
        //Bishop
        let mut bb_test = *bb1;
        while bb_test != 0 {
            let piece = bb_test.tzcnt();
            bb_test = bb_test & bb_test - 1;
            let mut bb_possi = diag_antid_moves(piece, occupied) & !black;
            while bb_possi != 0 {
                let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let b = bb_possi.tzcnt();
                compute_move_w(piece, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                let is_check = is_attacked(false, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                if !is_check {
                    legal_moves.push(((piece<<8) + b, Piece::BISHOP));
                }
                bb_possi = bb_possi & bb_possi - 1;
            }
        }
        //Rook
        devant = br.leading_zeros() as u64;
        derriere = br.trailing_zeros() as u64;
        let possi_br = hv_moves(devant, occupied);
        let possi_br2 = if devant != derriere {
            hv_moves(derriere, occupied)
        } else { 0 };

        //Queen
        let queen_pos = bq.leading_zeros();
        let possi_bq = hv_moves(queen_pos as u64, occupied) | diag_antid_moves(queen_pos as u64, occupied);
        
        //King
        let mut possi_bk = possibility_k(bk) & !black;
        while possi_bk != 0 {
                let (mut wp, mut wn, mut wb, mut wr, mut wq, mut wk, mut bp, mut bn, mut bb, mut br, mut bq, mut bk) = copy_bitboard(wp1, wn1, wb1, wr1, wq1, wk1, bp1, bn1, bb1, br1, bq1, bk1);
                let b = possi_bk.tzcnt();
                compute_move_w(bk.trailing_zeros() as u64, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                let is_check = is_attacked(false, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
                if !is_check {
                    legal_moves.push((((bk.trailing_zeros() as u64)<<8) + b, Piece::KING));
                }
                possi_bk = possi_bk & possi_bk - 1;
            }
    }
    legal_moves
}
fn check_mate() -> bool {
    true
}
fn undo_move(a :u64, b: u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> Result<(), Error> {
    let from : &u64;
    if ((*bp) & b) != 0 {

        from = bp;
    }
    else if *bn & b != 0 {
        
        from = bn;
    }
    else if *bb & b != 0 {
        from = bb;
    }
    else if *br & b != 0 {
        from = br;
    }
    else if *bq & b != 0 {
        
        from = bq;
    }
    else if *bk & b != 0 {
        
        from = bk;
    }
    
    Ok(())
}
fn print_custum_move(a_move : (u64,Piece)) {
    let a = a_move.0>>8;
    let b = a_move.0 & 255;
    //println!("{a} {b}");
    println!("{}{} {:?}", convert_square_to_move(a), convert_square_to_move(b), a_move.1);
}
fn main() {
    let now = Instant::now();
    println!("Instant init : {} nano seconde", now.elapsed().as_nanos());
    
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
    
    let mut white_to_play = true;
    //let moves = ["e2e3", "e7e6", "f1d3", "d8g5"];
    //let moves = ["b1c3","g8f6", "c3b1"];
    //let moves = ["e2e4","e7e5", "f2f4", "d2d4", "d7d5", "f1e2", "d8d6" ];
    //let moves = ["e2e4", "e7e5", "f1e2"];
    let moves = ["e2e4", "e7e5", "d1h5", "b8c6", "h5f7"]; //Just Check
    //let moves = ["e2e4", "e7e5", "f1c4", "b8c6", "d1h5", "g8f6", "h5f7"]; //Bergé
    draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    //let now = Instant::now();
    for m in moves {
    //loop {
        //let mut m = String::new();
        if white_to_play { println!("WHITE : "); }
        else { println!("BLACK : "); }
        
        //io::stdin().read_line(&mut m).unwrap();
        let (a,b) = convert_move_to_bitboard(&m);
        
        let now = Instant::now();
        let mut k_attacked = false;
        
        let response = if white_to_play {
            compute_move_w(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk)
        }
        else {
            compute_move_b(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk)
        };

        k_attacked = is_attacked(white_to_play, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        white_to_play ^= response;
        let legal = get_legal_move(white_to_play, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        println!(" {} nano seconde", now.elapsed().as_nanos());
        for x in legal {
            print_custum_move(x);
        }
        if k_attacked {
            print!("CHECK");
            if get_legal_move(white_to_play, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk).len() == 0 {
                print!(" MATE");
            }
            println!();
        }
        draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    }
    
    //println!("{} nano seconde", now.elapsed().as_nanos());
    //draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
}
