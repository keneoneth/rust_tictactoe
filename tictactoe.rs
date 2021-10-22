
// https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language

use std::fmt;
use std::io::stdin;
//=============================================


#[derive(Debug)]
struct Game {

    name: String,
    size: (u8,u8),
    base: [[u8;3];3]
}

// https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
impl Default for Game {
    fn default() -> Game {
        Game {
            name : "tictactoe".to_string(),
            size : (3,3),
            base : [[0u8;3];3]
        }
    }
}

impl Game {



    fn ended(&self) -> bool {
        // Customize so only `x` and `y` are denoted.
        // https://dev.to/anilkhandei/mutable-arrays-in-rust-1k5o

        // check horizontally
        let mut ret = 0u8;
        let mut zerofound = false;

        for i in 0..self.size.0{
            ret = 0u8;
            zerofound = false;
            for j in 0..self.size.1{
                if (self.base[i as usize][j as usize] == 0) {zerofound = true;}
                ret += self.base[i as usize][j as usize];
                // println!("i{} j{} y{} ret{}",i,j,self.base[i as usize][j as usize],ret);
            }
            if (!zerofound && u8::from(ret)==1u8*self.size.1 || u8::from(ret)==2u8*self.size.1){
                // println!("return horizontally");
                return true;
            }
        }

        //check vertically
        for j in 0..self.size.0{
            ret = 0u8;
            zerofound = false;
            for i in 0..self.size.1{
                if (self.base[i as usize][j as usize] == 0) {zerofound = true;}
                ret += self.base[i as usize][j as usize];
                // println!("i{} j{} y{} ret{}",i,j,self.base[i as usize][j as usize],ret);
            }
            if (!zerofound && u8::from(ret)==1u8*self.size.1 || u8::from(ret)==2u8*self.size.1){
                // println!("return vertically");
                return true;
            }
        }

        //check diagonally
        ret = 0u8;
        zerofound = false;
        for i in 0..self.size.1{
            if (self.base[i as usize][i as usize] == 0) {zerofound = true;}
            ret += self.base[i as usize][i as usize];
        }
        if (!zerofound && u8::from(ret)==1u8*self.size.1 || u8::from(ret)==2u8*self.size.1){
            // println!("return diagonally 0");
            return true;
        }

        ret = 0u8;
        zerofound = false;
        for i in 0..self.size.1{
            if (self.base[i as usize][i as usize] == 0) {zerofound = true;}
            ret += self.base[i as usize][(2u8-i) as usize];
        }
        if (!zerofound && u8::from(ret)==1u8*self.size.1 || u8::from(ret)==2u8*self.size.1){
            // println!("return diagonally 1");
            return true;
        }

        return false;
    }

    fn valueof(val : u8, index : u32) -> String {
        match val {
            0 => return index.to_string(),
            1 => return "O".to_string(),
            2 => return "X".to_string(),
            _ => return "?".to_string()
        };
    }

    fn display(&self) {
        let mut ret = String::from("");
        for i in 0..self.size.0{
            let mut rowret = String::from("|");
            let mut rowret2 = String::from("\n|");
            for j in 0..self.size.1{
                rowret.push_str( &Game::valueof(self.base[i as usize][j as usize],((i)*self.size.0+(j+1)).into()) );
                rowret.push_str( "|" );
                rowret2.push_str( "-|" );
            }
            ret.push_str(&rowret);
            if(i+1!=self.size.0) {rowret2.push_str( "\n" );ret.push_str(&rowret2);}
        }

        println!("{}",ret);

    }

    fn promtforinput(& mut self, playerid : u8) {

        let mut validinput = false;

        while (!validinput){

            println!("PLAYER {} turn, please enter your input position in (1~9):",playerid);
            let mut input_string = String::new();
            stdin().read_line(&mut input_string)
            	.ok()
                .expect("Failed to read line");



            let input_num = input_string.trim().parse::<u8>();

            match input_num {
                Ok(val) => (),
                Err(ref why) => {println!("Doesn't look like a number input ({}), please try again.", why); continue;},
            }

            let unwrap_input_num = input_num.as_ref().unwrap();

            if (unwrap_input_num>=&1 && unwrap_input_num<=&9) {
                let rowid = (unwrap_input_num - 1) / self.size.0;
                let colid = (unwrap_input_num - 1) % self.size.1;

                if (self.base[rowid as usize][colid as usize] == 0){
                    // println!("{} {}",rowid, colid);
                    self.base[rowid as usize][colid as usize] = playerid;
                    validinput = true;
                } else {
                    println!("Repeated Input! please try again.");
                }
            }else {
                println!("Invalid Input! please try again.");
            }

        }

    }

    fn checkend(&self, playerid: u8) -> bool {
        let ret = self.ended();
        if (ret) {
            self.display();
            println!("PLAYER {} WON, GAME OVER !!!",playerid);
        }
        return ret;
    }

    fn run(& mut self) {
        println!("GAME START !!!");
        let mut roundcnt = 1;
        let mut curplayer = 2;
        while (!self.checkend(curplayer)){
            println!(">>>>>>>>>>>>>>>>>> \nROUND {} !\n", roundcnt);
            self.display();
            match curplayer {
                1 => curplayer = 2,
                2 => curplayer = 1,
                _ => {println!("[error] unexpected player is found!"); assert!(false)}
            };
            self.promtforinput(curplayer);
            roundcnt += 1;

        }
    }
}

// https://stackoverflow.com/questions/27022848/how-i-can-mutate-a-structs-field-from-a-method

fn main(){

    let mut mygame = Game{..Default::default()};
    // println!("{:#?}", mygame);
    mygame.run();

}
