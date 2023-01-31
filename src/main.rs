use std::io;
use std::time::{Instant};

static FILE_A:u64 = 72340172838076673;
static FILE_B:u64 = 144680345676153340;
static FILE_H:u64 = 9259542123273814000;
static FILE_G:u64 = 4629771061636907000;
static FILE_AB:u64 = FILE_A | FILE_B;
static FILE_GH:u64 = FILE_G | FILE_H;
static RANK_8:u64 = 255;
static RANK_MASK : [u64;8] = [
    255, 65280, 16711680, 4278190080, 1095216660480, 280375465082880, 71776119061217280, 18374686479671624000
];
static FILE_MASKS : [u64;8] = [
    72340172838076670, 144680345676153340, 289360691352306700, 578721382704613400,
    1157442765409226800, 2314885530818453500, 4629771061636907000, 9259542123273814000
];
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
    print!("    ");
    for i in 0..8 {
        print!("{} ", (letter as u8+i) as char);
    }
    print!("\n   ");
    for _i in 0..8 {
        print!("__");
    }
    println!();
    for (i, x) in chess_board.iter().enumerate() {
        print!("{} | ", i+1);
        for c in x {
            print!("{c} ");
        }
        println!();
    }
}
fn convert_string_to_bitboard(binary:usize) -> u64 {
    //u64::pow(2, (binary) as u32)
    1<<binary
}
fn possibility_wp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let pmoves1 = *wp<<7 & black & !RANK_8;
    let pmoves2 = *wp<<9 & black & !RANK_8;
    let pmoves3 = *wp<<8 & !(black | white);
    pmoves1 | pmoves2 | pmoves3
}
fn possibility_bp(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let pmoves1 = *bp>>7 & white ;//& !RANK_1;
    let pmoves2 = *bp>>9 & white ;//& !RANK_1;
    let pmoves3 = *bp>>8 & !(white | black);
    pmoves1 | pmoves2 | pmoves3
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

fn hyperbola_quintessence(occupied : u64, mask: u64, mut number : u64) -> u64 {
    number = 1<<number;
    let mut forward = occupied & mask ;
    let mut reverse = forward.reverse_bits();
//Pc200055
    forward = forward.wrapping_sub(number.wrapping_mul(2));
    reverse = reverse.wrapping_sub(reverse.wrapping_mul(2)).reverse_bits();
    forward ^= reverse.reverse_bits();
    forward & mask
    //( - 2 * number) ^ ((occupied & mask).swap_bytes() - 2 * number.swap_bytes()).swap_bytes()
    //(occupied - 2 * number) ^ (occupied.reverse_bits() - 2 * number.reverse_bits()).reverse_bits()
}

fn convert_move_to_bitboard(moves : &str) -> (u64,u64) {
    let mut iter1 = moves[0..4].chars();
    let un = iter1.next().unwrap() as u64-96;
    let deux = iter1.next().unwrap() as u64-48;
    let trois = iter1.next().unwrap() as u64-96;
    let quatre = iter1.next().unwrap() as u64-48;
    /*let a = u64::pow(2, ((deux-1) *8 +  un-1 )as u32);
    let b = u64::pow(2, ((quatre-1) *8 +  trois-1)as u32);*/
    //println!("a : {:b} \nb : {:b}", a, b);
    let a = (deux-1) *8 +  un-1 ;
    let b = (quatre-1) *8 +  trois-1;
    (a,b)
}

fn compute_move_w(mut a:u64, mut b:u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let square_a = a;
    a = 1<<a;
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
    hyperbola_quintessence(occupied, DIAG_MASKS[((square/8) + (square%8)) as usize], square) | hyperbola_quintessence(occupied, ANTIDIAG_MASKS[((square/8)+7 - (square%8)) as usize], square)
}
fn hv_moves(square : u64, occupied : u64) -> u64 {
    hyperbola_quintessence(occupied, FILE_MASKS[(square % 8) as usize], square) | hyperbola_quintessence(occupied, RANK_MASK[(square / 8) as usize], square)
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
        //println!("M : {:b}", moves);
    }

    else if *bn & a != 0 {
        moves = possibility_bn(wp, &mut (*wn & a), wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        from = bn;
    }
    else if *bb & a != 0 {
        let occupied = black | white;
        moves = diag_antid_moves(square_a, occupied);
        from = bb;
    }
    else if *br & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied);
        from = br;
    }
    else if *bq & a != 0 {
        let occupied = black | white;
        moves = hv_moves(square_a, occupied) | diag_antid_moves(square_a, occupied);
        from = bq;
    }
    else if *bk & a != 0 {
        
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
fn main() {

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

    println!("{wp:b}");
    let mut white_to_play = true;
    //let play_move = "e2e3";
    //let (a,b) = convert_move_to_bitboard(play_move);
    let moves = ["e2e3", "d7d6", "f1d3", "d8g5"];
    //let moves = ["b1c3","g8f6", "c3b1"];
    draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    //let now = Instant::now();
    for m in moves {
    //loop {
        //let mut m = String::new();
        if white_to_play {
            println!("WHITE : ");
        }
        else {
            println!("BLACK : ");
        }
        //io::stdin().read_line(&mut m).unwrap();
        let (a,b) = convert_move_to_bitboard(&m);
        //println!("{a} et {b}");
        let response = if white_to_play {
            compute_move_w(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk)
        }
        else {
            compute_move_b(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk)
        };
        white_to_play ^= response;
        /*
        println!("{a:b} \n{b:b}");
        println!("{m}");
        println!("{response}");
        */
        draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    }
    //println!("{} nano seconde", now.elapsed().as_nanos());
}
