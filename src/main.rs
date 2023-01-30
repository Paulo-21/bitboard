use std::io;
use std::time::{Instant};

static FILE_A:u64 = 72340172838076673;
static FILE_B:u64 = 144680345676153340;
static FILE_H:u64 = 9259542123273814000;
static FILE_G:u64 = 4629771061636907000;
static FILE_AB:u64 = FILE_A | FILE_B;
static FILE_GH:u64 = FILE_G | FILE_H;
static RANK_8:u64 = 255;
//static RANK_1:u64 = 18374686479671624000;


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
    u64::pow(2, (binary) as u32)
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
    let noNoEa:u64 =  (*wn << 17) & !FILE_A ;
    let noEaEa:u64 =  (*wn << 10) & !FILE_AB;
    let soEaEa:u64 =  (*wn >>  6) & !FILE_AB;
    let soSoEa:u64 =  (*wn >> 15) & !FILE_A ;
    let noNoWe:u64 =  (*wn << 15) & !FILE_H ;
    let noWeWe:u64 =  (*wn <<  6) & !FILE_GH;
    let soWeWe:u64 =  (*wn >> 10) & !FILE_GH;
    let soSoWe:u64 =  (*wn >> 17) & !FILE_H ;
    (noNoEa | noEaEa | soEaEa | soSoEa | noNoWe | noWeWe | soWeWe | soSoWe) & !white
}
fn possibility_bn(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> u64 {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    //let white = *wp | *wn | *wb | *wr | *wq | *wk;
    let noNoEa:u64 =  (*bn << 17) & !FILE_A ;
    let noEaEa:u64 =  (*bn << 10) & !FILE_AB;
    let soEaEa:u64 =  (*bn >>  6) & !FILE_AB;
    let soSoEa:u64 =  (*bn >> 15) & !FILE_A ;
    let noNoWe:u64 =  (*bn << 15) & !FILE_H ;
    let noWeWe:u64 =  (*bn <<  6) & !FILE_GH;
    let soWeWe:u64 =  (*bn >> 10) & !FILE_GH;
    let soSoWe:u64 =  (*bn >> 17) & !FILE_H ;
    (noNoEa | noEaEa | soEaEa | soSoEa | noNoWe | noWeWe | soWeWe | soSoWe) & !black
}
fn convert_move_to_bitboard(moves : &str) -> (u64,u64) {
    let piece = &moves[0..2];
    let move_to = &moves[2..];
    let un = *&piece[0..2].chars().next().unwrap() as u64-96;
    let deux = *&piece[0..2].chars().nth(1).unwrap() as u64-48;
    let trois = *&move_to[0..2].chars().next().unwrap() as u64-96;
    let quatre = *&move_to[0..2].chars().nth(1).unwrap() as u64-48;
    let a = u64::pow(2, ((deux-1) *8 +  un-1 )as u32);
    let b = u64::pow(2, ((quatre-1) *8 +  trois-1)as u32);
    //println!("a : {:b} \nb : {:b}", a, b);
    (a,b)
}

fn compute_move_w(a:u64, b:u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    let black = *bp | *bn | *bb | *br | *bq | *bk;
    let mut moves= 0;
    if ((*wp) & a) != 0 {
        let mut p = (*wp) & a;
        
        moves = possibility_wp(&mut p, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        //println!("M : {:b}", moves);
    }
    else if *wn & a != 0 {
        moves = possibility_wn(wp, &mut (*wn & a), wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        
    }
    else if *wb & a != 0 {
        
    }
    else if *wr & a != 0 {
        
    }
    else if *wq & a != 0 {
        
    }
    else if *wk & a != 0 {
        
    }
    else if *wn & a != 0 {
        
    }
    else {
        
    }
    if moves & b != 0 {
        (*wp) = (*wp) & (!a);
        (*wp) = (*wp) | b;
        if black & b != 0 {
            if *bp & b != 0 {
                *bp &= !b;
            }
            else if *bn & b != 0 {
                *bn &= !b;
            }
            else if *bb & b != 0 {
                *bb &= !b;
            }
            else if *br & b != 0 {
                *br &= !b;
            }
            else if *bq & b != 0 {
                *bq &= !b;
            }
        }
        true
    }
    else {
        false
    }
}
fn compute_move_b(a:u64, b:u64, wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) -> bool {
    //let black = *bp | *bn | *bb | *br | *bq | *bk;
    let white = *wp | *wn | *wb | *wr | *wq | *wk;
    if ((*bp) & a) != 0 {
        let mut p = (*bp) & a;
        let moves = possibility_bp(wp, wn, wb, wr, wq, wk, &mut p, bn, bb, br, bq, bk);
        //println!("M : {:b}", moves);

        if moves & b != 0 {
            (*bp) &= !a;
            (*bp) |= b;
            if white & b != 0 {
                if *wp & b != 0 {
                    *wp &= !b;
                }
                else if *wn & b != 0 {
                    *wn &= !b;
                }
                else if *wb & b != 0 {
                    *wb &= !b;
                }
                else if *wr & b != 0 {
                    *wr &= !b;
                }
                else if *wq & b != 0 {
                    *wq &= !b;
                }
            }
            true
        }
        else {
            false
        }
    }
    else if *wn & a != 0 {
        possibility_wn(wp, &mut (*wn & a), wb, wr, wq, wk, bp, bn, bb, br, bq, bk);
        false
    }
    else if *wb & a != 0 {
        false
    }
    else if *wr & a != 0 {
       false 
    }
    else if *wq & a != 0 {
        false
    }
    else if *wk & a != 0 {
        false
    }
    else if *wn & a != 0 {
        false
    }
    else {
        false
    }
}
fn main() {
    println!("Hello, world!");
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

    println!("{:b}", wp);
    let mut white_to_play = true;
    //let play_move = "e2e3";
    //let (a,b) = convert_move_to_bitboard(play_move);
    //println!("WP : {:b}", a&wp);
    let moves = ["e2e3", "d7d6", "e3e4", "d6d5", "e4d5"];
    draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    let now = Instant::now();
    for m in moves {
        //let mut buffer = String::new();
        //io::stdin().read_line(&mut buffer).unwrap();
        let (a,b) = convert_move_to_bitboard(&m);
        let response;
        if white_to_play {
            response = compute_move_w(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        }
        else {
            response = compute_move_b(a, b, &mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
        }
        white_to_play ^= response;
        
        //println!("{:b} \n{:b}",a ,b);
        //println!("{}", m);
        //println!("{}", response);
        //draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);
    }
    println!("{} nano seconde", now.elapsed().as_nanos());
}
