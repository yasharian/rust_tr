use std::io;

fn main(){
    let mut string = io::String();
    io::stdin().read_line(&mut string).expect("the statement was unreadable");
	
    let mut this = 5;
    println!("{}" , this );
	this = 4 ; 
	this = 3;
	println!("{this}");
}
