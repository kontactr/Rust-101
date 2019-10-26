pub fn run(){
    // Print to console
    println!("Hello from the print file");
    println!("Number: {}",1);
    println!("{} is from {}","A","B");
    println!("{0} {1} {2} {0} {2}","A","B","C");
    println!("{name} , {name1}", name = "A"  , name1 = "B");
    println!("Binary {:b} Hex {:x} Octal {:o}" , 10 , 11 ,  12);
    println!("{:?}",(1,2,3,4,true,5,6,7 , "L"));
    println!("{}" , 20 + 20 * 20 - 20 / 20);
}
