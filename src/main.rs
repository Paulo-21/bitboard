static FILE_A:u64 = 72340172838076673;
static RANK_8:u64 = 255;


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
        print!("{i} | ");
        for c in x {
            print!("{c} ");
        }
        println!();
    }
}
fn convert_string_to_bitboard(binary:usize) -> u64 {
    u64::pow(2, (63-binary) as u32)
}
fn possibilityWP(wp:&mut u64, wn:&mut u64, wb:&mut u64, wr:&mut u64, wq:&mut u64, wk:&mut u64, bp:&mut u64, bn:&mut u64, bb:&mut u64, br:&mut u64, bq:&mut u64, bk:&mut u64) {
    let black = *bp & *bn & *bb & *br & *bq & *bk;

    let pmoves = *wp>>7 & black & !RANK_8;
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

    (a,b)
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

    //println!("{:b}", wr);
    //draw_board(&mut wp, &mut wn, &mut wb, &mut wr, &mut wq, &mut wk, &mut bp, &mut bn, &mut bb, &mut br, &mut bq, &mut bk);

    let play_move = "a2h8";
    let (a,b) = convert_move_to_bitboard(play_move);
    println!("{:b} \n{:b}",a ,b);
}
