fn main() {
    // let x: i8 = 10;
    // let y:u32 = 16;
    // let z:f32 = 1000.001;


    // for i in 0..1000 {
    //     x += 100;
    // }


    // println!("x = {}",x);


    // let is_male = true;
    // let is_above_18 = true;


    // if is_male {
    // println!("You are a male");
    // } else {
    // println!("You are not a male");
    // }
    // if is_male && is_above_18 {
    // print!("You are a legal male");
    // }
    

    //string
    // let greeting  = String::from("Sachin Kumar!");
    // println!("{}",greeting);
    // let char1 = greeting.chars().nth(20);


    // match char1 {
    //     Some(c) => println!("{}",c),
    //     None => println!("No character at index 1000"),
    // }


    // let s1 = String::from("Hello");
    // let s2 = &s1;
    // print!("{}\n",s2);
    // print!("{}",s1);


    // let is_even = true;
    // if is_even {
    //     println!("The number is even");
    // } else if !is_even {
    //     println!("The number is odd");
    // }


    // for i in 0..11 {
    //     print!("{} ",i);
    // }
    

    //iterate over loop in string
    // let sentence = String::from("my name is sachin");
    // let first_word = get_first_word(sentence);
    // print!("First word is: {}",first_word);


    // fn get_first_word(sentence: String) -> String{
    //     let mut ans = String::from("");
    //     for char in sentence.chars(){
    //         ans.push_str(char.to_string().as_str());
    //         if char == ' '{
    //             break;
    //         }
    //     }
    //     return ans;
    // }
    

    //functions
    // let a = 10;
    // let b = 13;
    // let sum = do_sum(a,b);
    // println!("Sum of {} and {} is {}",a,b,sum);
    

    //im mutability
    // let mut x = String::from("hi there");
    // x.push_str("asd");
    // println!("{}",x);


        // stack_fn(); // Call the function that uses stack memory
        // heap_fn(); // Call the function that uses heap memory
        // update_string(); // Call the function that changes size of variable at run


        // let mut s1 = String::from("Hello");
        // update_str(&mut s1);
        // print!("{}",s1);


        // //Struct In Rust
        // let user1 = User{
        //     active: true,
        //     user_name: String::from("Sachin123"),
        //     email: String::from("sachin123@gmail.com"),
        //     sign_in_count: 1,
        // };
        // println!("User 1 username: {:?}",user1.user_name);
    }

    // struct User{
    //     active: bool,
    //     user_name: String,
    //     email: String,
    //     sign_in_count : u64,
    // }


    // fn update_str(s: &mut String){
    //     s.push_str(" World");
    // }


    //    fn stack_fn() {
    //     // Declare a few integers on the stack
    //     let a = 10;
    //     let b = 20;
    //     let c = a + b;
    //     println!("Stack function: The sum of {} and {} is {}", a, b, c);
    //    }


    //    fn heap_fn() {
    //     // Create a string, which is allocated on the heap
    //     let s1 = String::from("Hello");
    //     let s2 = String::from("World");
    //     let combined = format!("{} {}", s1, s2);
    //     println!("Heap function: Combined string is '{}'", combined);
    //    }


    //    fn update_string() {
    //     // Start with a base string on the heap
    //     let mut s = String::from("Initial string");
    //     println!("Before update: {}", s);
    //     // Append some text to the string
    //     s.push_str(" and some additional text");
    //     println!("After update: {}", s);
    //   }



// fn do_sum(a:i32,b:i32)->i32{
//     return a+b;
// }
