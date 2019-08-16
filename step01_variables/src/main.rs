use typename::TypeName;

fn main() {
    let var1 = 5;
    println!("The value of var1 is {} and its type is {}", var1, var1.type_name_of());

    //Allvariables by default are immutable and cannot be resigned 
    // var1 = 10;

    let mut var2 = 6;
    println!("The value of var2 is {} and its type is {}", var2, var2.type_name_of());
    var2 = 11;
    println!("The value of var2 is {} after update and its type is {}", var2, var2.type_name_of());

    let var3 = 5.9;
    println!("The value of var3 is {} and its type is {}", var3, var3.type_name_of());

    let var4: u32 = 2;
    println!("The value of var4 is {} and its type is {}", var4, var4.type_name_of());

    let var5 = true;
    println!("The value of var5 is {} and its type is {}", var5, var5.type_name_of());

    let var5 = 10;//shadow
    println!("The value of var5 is {} and its type is {}", var5, var5.type_name_of());

}
