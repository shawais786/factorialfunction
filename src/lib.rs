use std::io;

pub fn factorial (){
println!("Enter a positive integer 0 ~ 12 to get its factorial!");

let mut ent_input = String::new();
io::stdin().read_line(&mut ent_input).expect("Error while reading the input");
let mut input:u32 = match ent_input.trim().parse() {
    Ok(input) => input,
    Err(_input) => {
        println!("A positive integer isn't entered, Please enter a positive integer!");
        let mut errr:u32 = 1;
        loop {
        let mut ent_input1 = String::new();
        io::stdin().read_line(&mut ent_input1).expect("Error while reading the input");
        errr = match ent_input1.trim().parse() {
            Ok(c) => {errr=c;
                        break;
                        },
            Err(_errr) => {println!("A positive integer isn't entered, Please enter a positive integer!");
                0},
        };
    };
        errr
},
};

let referance = input;       //  Just to store the passing number to print at the last
let mut result:u32 = input;  //  Initiated with numb so that if user passes number "1"
    if input == 0 {
        result = 1;
    }
    else {                        //  then loop will not run and result must be "1"
    while input>1{
    let mut temp:u32 = 0;   //  Initiated variable 'temp' with any number
    temp=result;            //  to store the number or result of the multiplication
                            //  so that it can be multiplied with 1 decreased
    input=input-1;                
    result=input*temp;
}
    }
println!("Factorial of {} is: {}",referance, result);
}