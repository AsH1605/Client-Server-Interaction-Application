#![allow(unused)] //to ignore warnings

use std::io;
use rand::Rng;
// crates.io for getting the latest version
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restraurant;
use crate::restraurant::order_food;

fn say_hello(){
    println!("Hello");
}

fn get_sum(x: i32,y: i32){
    println!("{} + {} = {}",x,y,x+y);
}

fn get_sum2(x: i32,y: i32)-> i32{
    x+y //return x+y;
}

fn get2_val(x: i32)-> (i32,i32){
    return (x+1,x+2);
}

fn sum_list(list: &[i32])-> i32{
    let mut sum=0;
    for &val in list.iter(){
        sum+=&val;
    }
    sum
}
//generic

use std::ops::Add;
fn get_sum_gen<T:Add<Output =T>>(x:T, y:T)->T{
    return x+y;
}

fn print_str(x:String){
    println!("A string {}",x);
}
fn print_return_str(x:String)->String{
    println!("A string {}",x);
    x
}
fn change_str(name: &mut String){
    name.push_str(" is happy");
    println!("Message: {}",name);
}

fn main() {
    // println!("What is your name?");
    // // all values created inside are immutable
    // let mut name= String::new();
    // let greeting="Nice to meet you";
    // io::stdin().read_line(&mut name)
    //     .expect("Didnt recieve input");

    // println!("Hello {}! {}", name.trim_end(),greeting);

    // DATA TYPES
    // const ONE_MIL: u32=1_000_000; ;//unsigned 32 bit integer
    // const PI: f32=3.141592;
    // let age="47";
    // let mut age: u32= age.trim().parse()
    //     .expect("age want assigned a number");
    // age=age+1;
    // println!("I am {} and i want ${}", age, ONE_MIL);

    // integers
    // unsigned integers: u8,u16,u32,u64,u128,usize
    // signed integer: i8,i16,i32,i64,i128,isize
    // println!("Max u32: {}",u32::MAX);
    // println!("Max u64: {}",u64::MAX);
    // println!("Max usize: {}",usize::MAX);
    // println!("Max u128: {}",u128::MAX);

    // booleans
    // let is_true= true;
    // let my_grade = 'A'; //char

    // let num_1: f32=1.1111111111111111;
    // println!("f32: {}", num_1+0.111111111111111);

    // let num_3: u32=5;
    // let num_4: u32=4;
    // println!("5+4={}",num_3+num_4); //arithematic operator

    // let random_num: i32= rand::thread_rng().gen_range(1..101);
    // print!("random: {}",random_num);


    // conditional
    // let age=8;
    // if(age>=1) && (age<=18){
    //     print!("IMportant Birthday");
    // } else if(age==21) || (age==50){
    //     print!("IMportant Birthday");
    // } else if(age>=65){
    //     print!("IMportant Birthday");
    // } else{
    //     print!(" Not an IMportant Birthday");
    // }

    // let mut my_age=47;
    // let can_vote = if my_age>=18{
    //     true
    // } else{
    //     false
    // };
    // println!("Can Vote: {}", can_vote);

    // let age2=8;
    // match age2 {
    //     1..=18 =>print!("IMportant Birthday"),
    //     21|50 =>print!("IMportant Birthday"),
    //     65..=i32 ::MAX =>print!("IMportant Birthday"),
    //     _=> print!(" Not an IMportant Birthday"),
    // };

    // let my_age = 18;
    // let voting_age=18;
    // match my_age.cmp(&voting_age){
    //     Ordering::Less=>print!("Cant vote"),
    //     Ordering::Greater=>print!("Vote"),
    //     Ordering::Equal=>print!("U have the right to vote"),
    // };

    // array
    // let arr_1=[1,2,3,45,6,7,8,9];
    // print!("1st: {}",arr_1[0]);
    // print!("length: {}",arr_1.len());
    // let mut loop_idx=0;
    // loop{
    //     if arr_1[loop_idx] % 2==0{
    //         loop_idx+=1;
    //         continue;
    //     }
    //     if arr_1[loop_idx]==9{
    //         break;
    //     }
    //     print!("Val: {}",arr_1[loop_idx]);
    //     loop_idx+=1;
    // }

    // while loop_idx<arr_1.len(){
    //     print!("Array: {}",arr_1[loop_idx]);
    //     loop_idx+=1;
    // }

    // for val in arr_1.iter(){
    //     println!("Val: {} ", val);
    // }

    // let my_tuple:(u8,String,f64)=(47,"hiiiii".to_string(),50_000.00);
    // println!("Name: {}",my_tuple.1);
    // let(v1,v2,v3)=my_tuple;
    // println!("Age: {}",v1);

    // let mut st1=String::new();
    // st1.push('A');
    // st1.push_str("words");
    // for word in st1.split_whitespace(){
    //     println!(" {} ",word);
    // }
    // let st2=st1.replace("A","Another");
    // println!(" {} ",st2);
    // let st3=String::from("x r t b h k k a c");
    // let mut v1:Vec<char>=st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1{
    //     println!("{}",char);
    // }
    // let st4="Random string"; 
    // let mut st5=st4.to_string();
    // println!("{}",st5);
    // let byte_arr=st5.as_bytes();
    // let st6= &st5[0..6];
    // println!("String length: {}",st6.len());
    // st5.clear();
    // let st6=String::from("JUst sum");
    // let st7=String::from("words");
    // let st8=st6+&st7; //st7 still exists arfter op but st6 doesnt
    // for char in st8.bytes(){
    //     println!("{}",char);
    // }
    // let int_u8: u8=5;
    // let int2_u8:u8=4;
    // let int3_u32: u32=(int_u8 as u32)+(int2_u8 as u32);

    // enums
    // enum Days{
    //     MOnday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday
    // }

    // impl Days{
    //     fn is_weekend(&self)->bool{
    //         match self{
    //             Days::Saturday | Days::Sunday =>true,
    //             _=> false
    //         }
    //     }
    // }
    // let today:Days=Days::MOnday;
    // match today{
    //     Days::MOnday=>println!("Everyone hates monday"),
    //     Days::Tuesday=>println!("Donut day"),
    //     Days::Wednesday=>println!("Hump day"),
    //     Days::Thursday=>println!("Pay day"),
    //     Days::Friday=>println!("day"),
    //     Days::Saturday=>println!("Weekend"),
    //     Days::Sunday=>println!("Weekend"),
    // }

    // println!("Is today the weekend? {}", today.is_weekend());

    // vector
    // let vec1: Vec<i32>=Vec::new();
    // let mut vec2=vec![1,2,3,4];
    // vec2.push(5);
    // println!("1st: {}",vec2[0]);
    // let second: &i32 =&vec2[1];
    // match vec2.get(1){
    //     Some(second)=> println!("2nd : {}",second),
    //     None=>println!("No 2nd value"),
    // }
    // for i in &mut vec2{
    //     *i *=2;
    // }
    // for i in &vec2{
    //     println!("{}",i);
    // }
    // println!("Vector len {}",vec2.len());
    // println!("Pop : {:?}",vec2.pop());

    // say_hello();
    // get_sum(5,4);
    // print!("{}",get_sum2(5, 4));

    // let (val1,val2)=get2_val(3);
    // println!("Nums: {} {}",val1,val2);
    // let num_list=vec![1,2,3,4,5];
    // println!("sum of list : {}",sum_list(&num_list));

    // println!("5+4={}",get_sum_gen(5, 4));
    // println!("5.3+4.1={}",get_sum_gen(5.3, 4.1));
    
    // stacks and heaps
    // let mut str1=String::from("WOrld");
    // let str2=str1;  //str1 no longer exist
    // let str2=str1.clone();  //with string array vector
    // println!("Hello {}",str1);
    // print_str(str1);
    // let str3=print_return_str(str1);
    // change_str(&mut str1);

    // hashmaps
    // let mut heros=HashMap::new();
    // heros.insert("SUperman", "Clark Kent");
    // heros.insert("Batman", "Bruce Wayne");
    // heros.insert("The Flash", "Barry Allen");

    // for(k,v) in heros.iter(){
    //     println!("{} = {}",k,v);
    // }
    // if heros.contains_key(&"Batman"){
    //     let the_bat=heros.get(&"Batman");
    //     match the_bat{
    //         Some(x)=>println!("Batman is a hero"),
    //         None=>println!("Batman is not a hero"),
    //     }
    // }

    // struct customer{
    //     name: String,
    //     add: String,
    //     balance: f32,
    // }
    // let mut bob = customer{
    //     name: String::from("Bob Smith"),
    //     add: String::from("555 Main St"),
    //     balance: 234.50
    // };
    // bob.balance=3344.3

    // struct Rectangle<T,U>{
    //     length: T,
    //     height: U,
    // }
    // let rec= Rectangle{length:4,height:10.5};
    // const PI:f32=3.141592;
    // trait Shape{
    //     fn new(length: f32, width:f32)->Self; //constructor
    //     fn area(&self)->f32;
    // }

    // struct Rectangle{length:f32, width: f32};
    // struct Circle{length:f32, width: f32};

    // impl Shape for Rectangle{
    //     fn new(length:f32, width:f32)->Rectangle{
    //         return Rectangle { length, width};
    //     }
    //     fn area(&self)->f32{
    //         return self.length*self.width;
    //     }
    // }
    // impl Shape for Circle{
    //     fn new(length:f32, width:f32)->Circle{
    //         return Circle{ length, width};
    //     }
    //     fn area(&self)->f32{
    //         return (self.length/2.0).powf(2.0)*PI;
    //     }
    // }

    // let rec:Rectangle=Shape::new(10.0,10.0);
    // let circ:Circle=Shape::new(10.0,10.0);
    // println!("Rect rea: {}",rec.area());
    // println!("Cir rea: {}",circ.area());

    // crates: modules that produce a libraru or executable
    // modules: organise and handle privacy
    // packages: build, test and share crates
    // paths: a away of naming an item such as a struct, function
//    order_food(); 
    
    // panic!("Terrible Error");
    // let lil_arr=[1,2];
    // println!("{}",lil_arr[10]);
    
}
