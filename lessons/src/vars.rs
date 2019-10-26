pub fn vars(){
    let name = "Hello World";
    println!("{}" , name);

    let name_one = name;
    println!("{}", name_one);

    let mut age = 30;
    println!("{} {}",name,age);

    //age = 31; we can't reassign it unless we put mut
    age = 31;
    println!("{} {}",name_one,age);

    //Constant we must need to assign types to const
    // const should be in upper case warn 
    const ID: i8 = 001;
    println!("ID : {}",ID);

    const id: i8 = 002;
    println!("ID : {}",id);


    //Assign multiple vars
    let (a , b, c, d) = (1,2,4,5);
    println!("{:?}",  (a,b,c,d))
}