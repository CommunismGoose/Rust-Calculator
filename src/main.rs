use std::io;
fn main() {
    println!("please choose a number for x");
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Your operating system does not support this");
    let x: f64 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => 1.0,
    };
    println!("Choose an operator you may choose from the following:");
    println!("+ for addition");
    println!("- for subtraction");
    println!("x or * for mutiplication");
    println!("/ for division");
    println!("** to raise x to the power of y");
    let mut opp = String::new();
    io::stdin()
        .read_line(&mut opp)
        .expect("Your operating system does not support this");
    let opp = opp.trim();
    println!("please choose your second number, y");
    if opp == "**"{
        println!("please choose a number 1 or greater that is an interger")
    }
    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Your operating system does not support this");
    let y: f64 = match y.trim().parse(){
        Ok(num) => num,
        Err(_) => 1.0,
    };
    if opp == "+"{
        let finalnum = add(x,y);
        println!("{x}+{y}={finalnum}");
    } else if opp == "-"{
        let finalnum = sub(x, y);
        println!("{x}+{y}={finalnum}");
    } else if opp == "x" || opp == "*"{
        let finalnum = multipy(x, y);
        println!("{x}*{y}={finalnum}");
    } else if opp == "/" {
        let finalnum = divide(x,y);
        println!("{x}/{y}={finalnum}");
    } else if opp == "**"{
        let finalnum=expoent(x, y);
        println!("{x}**{y}={finalnum}")
    }
}
fn add(x:f64, y:f64) -> f64 {
    let finalnum:f64 = x+y;
    finalnum
}
fn sub(x:f64, y:f64) -> f64 {
    let finalnum:f64 = x-y;
    finalnum
}
fn multipy(x:f64, y:f64) -> f64 {
    let finalnum:f64 = x*y;
    finalnum
}
fn divide(x:f64, y:f64) -> f64 {
    let finalnum:f64 = x/y;
    finalnum
}
fn expoent(x:f64, y:f64) -> f64 {
    let mut count: f64 = 1.0;
    let mut finalnum:f64 = x;
    while count<y{
        finalnum = finalnum*x;
        count=count+1.0
    }
    finalnum
}