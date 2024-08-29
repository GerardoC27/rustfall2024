
// => Couple of words regarding Systems programming in general

// Programming is hard.

// This is a single core class, which basically can decide your fate as a software engineer.
// All 4 years of study, prepares your to take this most important class in CS ciriculum.

// In my humble opinion, it's importance even higher than Data Structures and Algorithm class.
// Make sure to focus heavily on it. Bring your best game :)


// Note: If you can make it, you can assume that you basically conquered the world of programming
// and LeetCode Easy-Medium problems will look like baby's playground.

=> Couple of words regarding Rust



// First Stop Always:
// https://www.rust-lang.org/
// Open Source Textbook
// Famous Community The Rust Book the so called THE BOOK
// https://doc.rust-lang.org/book/

Note: Don't write code in your "god"-programming language style and be open-minded:)

// Week # 1

//Intro Let's Obtain Same Environment:

// I am huge fun of JetBrains product
// but you can choose any IDE of course. For a starter even REPL will work.


// => JetBrains
//https://www.jetbrains.com/rust/

=> And of course practice:

// https://www.jetbrains.com/edu-products/learning/rust/

// => VS Code
// https://code.visualstudio.com/docs/languages/rust

//Note for Windows users: Surprisingly Rust is well supported , you won't have any issues at all

=> Phylosophy of System Programming. Big Overview

Programming Languages

Assembler -> C > C++,Rust(manual memory managment) (both LLVM compilers) -> Java,Go(garbage collector) -> Python

(CPU + Memory) <-> Operation System <-> Your Compiled Program

=> Before we begin...

https://leetcode.com/problems/fizz-buzz/

https://leetcode.com/problems/single-number/

https://leetcode.com/problems/find-the-difference/

https://leetcode.com/problems/merge-sorted-array/

https://leetcode.com/problems/reverse-linked-list/

Bonus: https://leetcode.com/problems/move-zeroes/


// Common Programming Concepts

/*
Specifically 
1) Variables
2) Basic Types
3) Functions
4) Control flow
5) Comments
*/

fn main() {

    // Part #1
    
    // Section #1 Variables and Mutability
    
    // Surprising Idea Variables by default are immutable
    // Counterintuitive idea which is now acepted commonly Kotlin, Go follows the same logic
    // It turns out that immutability of variables helps with safety, as well as concurency
    
    
    //If you remember in C and C++ it's considered as a good practice to declare a variable as a constant
    // like int *const ptr;
    
    
    let x = 5; // Of course ; Heritage of C language )), by the way use 4 spaces, not tabs
    
    println!("The value of x is: {}",x); // -> This line should not surprise you, its common formater printer, only one thing to notice and keep in mind println! is a macros this fact is distinguished by ! mark.

    //x = 6; This line will not compile ;) Which is a nice. Imagine that you change the pointer to a database, that would be horrible.
        
    //Let's achieve what we want
    
    let mut y = 10; // After compiling, you will see the warning, compiler of Rust is really smart.
    // It constantly analyzes your code for weaknesses. Also notice red line.
    
    // Usually there is only one right way to do things in system programming. So from now on it's a Rust way ))
    println!("The value of y is: {}",y);
    
    // So if you declare something mutable, well you should change it right?
     y = 50;
    
    println!("The value of y is: {}",y);

    // So where is a const keyword
    // Well there is a const keyword, but it has like really constant meaning

    const MAX_POINT: u32 = 100_000;
    // Key features of constant, you should know the value in advance and declare it's type
    // Value cannot be obtained from a function, you should provide them before the runtime,
    // if something is constant, you probably know what it should hold
    
    // u32 -> unsigned integer of 32 bit in size

    // Also notice _ underscore, very useful thing when you type numbers with many digits
    // Improves readability
    
    println!("The value of MAX_POINT is: {}",MAX_POINT);

    // Shadowing -> This is a really useful feature, 
    
    // if you remember
    // Again for example in Java, when you read number from console and later need to convert that number to
    // integer or double, you usually create a two variables

    /*
        String str = "25";
        try{
            int number = Integer.parseInt(str);
            System.out.println(number); // output = 25
        }
        catch (NumberFormatException ex){
            ex.printStackTrace();
        }
    */

    // Which is absolutely lame.
        
    // But if you from Python world you know there is an idea which allows you to do next
    /*
    num = "25"
    num = int(num)
    */

    // So here is the same principle, you know what is going to happen.
    // But since we are operating in realm of compiled language, things will be a bit different.
        

        // Two examples of shadowing

    // Converting from one type to another, shadowing allows us to use same variable name, even though we change it's type
        
    let num = "25";
    let num: u32 = num.parse()
       .expect("please give me correct string number!");
    // This is an interesing code snippet, and really highlights the philosophy of Rust, by the way Golang follows the same philosophy.

    // So when you try to perform operation like calling a function in our case we want to convert string to unsigned integer 32 bit. It's a good to know what happened, so in Rust methods returns the special enum type called Result. It'can be OK or Err. Yes this is a real naming.
    
    
    // We will come back it later. 
    
    // But for know remember expect method will panick if result of method call happened to be error
    // Message for error is up to you, but let's compile and see

    // There is an elegant way to handle errors without panicking.
    // Error handling is another skill we will learn.
    
    // let num = "twenty five";
    // let num: u32 = num.parse()
    //    .expect("please give me correct string number!");
    

    // Reassigning the value
    // If we perform computations on some immutable value we can use the same variable name by reassigning it to a new variable -> Shadowing in practice ))
    
    let num = num + 25;
    let num = num *  2;

    println!("The value of num is: {}",num);

    // It look's strange. But it has some logic.
    // When you change value of variable, you need to use keyword "let", and after transformations you need to perform, variable value will not change, accidently. 

    // Another example:
    let wordlength = "UTRGV";
    let wordlength = wordlength.len(); // here len method will only work for string type so no need to worry to handle the error.


    // But this not gonna work
    // let mut wordlength = "UTRGV";
    // wordlength = wordlength.len();

    // len expects to return usize -> another special data type of Rust,
    // but in our case initially wordlength is a string, so even though it's mutable it cannot change it's type!
    println!("The value of wordlength is: {}",wordlength);

    // To the point: keyword let allows us to declare a new variable, so we can use any name.

    // -> Section 2): Data types

    // So if you noticed before sometimes we did provide a data type, sometimes we don't.

    // But we know by heart that in statically typed languages we should specify the type in a compile time, 
    //which is correct
    
    // The thing is a next: Rust compiler as well as many modern programming languages(Swift,Kotlin) can infer what type we want, based on the value we provide, but sometimes compiler need more information which type we want to use

    // Especially when we trying to obtain the value from a method call and result cannot be predicted for sure in advance

    // let user_num: u32 = "5000000000000000000000000000000".parse()
    //    .expect("Not a integer of 32 bit!");

    // // Imagine how many integer overflow errors this will save you from!

    // println!("The value of userNum is: {}",user_num);


    // So sometimes we are obliged to specify data type for own safety, And I really mean it. and you can be sure Rust will enforce it from you.

    // But before we go just remind ourself CS 101
    
    // 1) -> Type of a variable decide it's fate, or what kind of operation we can perform on them
    // 2) -> There are in general two big groups of data types Scalar(Integer,Float,Boolean,Char) and Compund(tuple,arrays) Those are basics and supported by all programming languages.

    // But know you will learn about accuracy and use them dilligently ))

    // => Scalar types

    /* Integer Types

     Length Signed Unsigned(always non-negative)
     8-bit   i8     u8
     16-bit  i16    u16
     32-bit  i32    u32
     64-bit  i64    u64
     128-bit i128   u128
     arch    isize  usize

     When I was a student I hated this kind of tables, because they randomly followed the rules.
     If you ever hit integer overflow problem from C and C++, when you change a value and randomly gets weird number unexpectedly, and spent hours trying to find a bug, you know how it feels.

     But now things will be different. Rust compiler analyzes your code and will enforce you to use correct data type, so from now on you can trust this table.

    */

    // Ex:
    // let mut money : u8 = 0; // I want to make sure that day variable always be non-negative u8 (0 to 255)
    
    // money -= 1;
    // println!("The value of money is: {}",money);

    // thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:203:5
    // getting that compile error message is much better than to  try to find overflow ))

    // Couple of words about "isize" and "usize"
    // special data type its size in bits depends on architecture where you running your program 64-bit 32-bit. 
    // you will see his type very often when we need index of some sort of collection (pointers)
    
    // We came across this data types very often. In general it's number of bits required to represent a single memory slot address, so it should be decided only by architecture of the machine. Because Rust allows you to write low level code, and architecture is important if you want to speak to bare metall.

    // And of course you can obtain integer literals in any other form

    // Decimal  98_222
    // Hex      0xff
    // Octal    0o77
    // Binary   0b1111_0000
    // Byte(u8) -> (Beauty of Rust, single ASCII character needs only 8 bit(0-256)) b'A'

    // Note: If you rely on Rust compiler to infer type of data based on provided value, by default it will stick with i32

    /* Float types

    f32 about 7 digits precision
    f64 about 16 digits precision

    By default f64

    */

    let my_f = 1.4567891016797080012345678998764;
    println!("The value of m_f is: {}",my_f);

    let my_f: f32 = 1.4567891016797080012345678998764;
    println!("The value of m_f is: {}",my_f);

    // Note : Please observe number of digits keeped!

    /* Numeric operations are the same 
     + -> addition
     - -> subtraction
     * -> multiplication
     / -> division
     % -> modular

     */

    // Boolean Type, nothing fancy

    //let t = true;
    //let f: bool = false;

    
    // Character Type (single quote of course) 4 bytes in size(or simpler UTF-8)
    let ch : char =  '\u{1F60D}'; // You can have any emoji you want, there is even a special crate(library)

    println!("Rust is: {}",ch);

    

    //Part # 2
    

    // Section #3 => Primitive Compund Types (Rust has a hashmaps, lists, PriorityQueue(or heap) don't worry)

    // Sometimes we want a group data together.
    // If we know in advance the size of our data and it has a same type 
    // usually we prefer arrays, but what if not, what to do ?
    
    // a) Tuples -> (if you know a bit of Python, you should be very familiar)
    // It has mainly two usages
    // 1) Tuples are immutable, which means once specified you can't really change it, like at all
    // 2) It allows you to keep many different data types together

    // Note: It has a main benefit, which we really on heavily in this class, it allows to return many results from a one function call

    /* Code snippet from Python

    def multplus(x,y):
        return (x*y,x+y)  <- this is a tuple, we will need to unpack them, another accepted name is "pattern matching"

    product, addition = multplus(10,5) -> our variables will be equal to product = 50, addition = 15
    
    */

    // Back to Rust

    let my_tup : (i32, f64, u8) = (500,7.7,100);

    let (x,y,z) = my_tup;

    println!("The value of x = {}, y = {}, z = {}",x,y,z);

    
    // Or another way to access elements in a tuple, since it's ordered compund type we can access the values
    // -> using a period(.) followed by index

    let point = (4,2); // please notice we rely on a Rust compiler to infer a data type from a provided values

    let row = point.0; // index accessing
    let column = point.1;

    println!("The coordinates of a single point  x = {}, y = {} ",row, column);

    // b) Array type

    // I assume that you are comfortable, things keep in mind (fixed length, same type, it's not a vector or list!!!)

    let cities = ["Edinburg", "MCAllen", "Austin"];
    // or

    let nums: [i32; 5] = [77,111,222,333,444];  // I must specify a type and size in advance

    // or 
    let zeros = [0; 100]; // I can specify like this [default value; number of elements]

    println!("My zeros {:?} ",zeros); // if you noticed :?, this is called printing in a debug node, arrays don't have string representation, so it's a convenient way to see the inside of array (you can try delete, we will get even error)
    // What is good in Rust, there is no silencing or ignoring.

    // Accessing, same as everywhere

    let my_home = cities[0];

    println!("My home is : {}",my_home);

    // And of course, most common problem Index out of bound

    
    let idx = 4;
    

    println!("{}", nums[idx]);

    // Curious thing to notice
    // If you hover your mouse over idx, you will see that it's a usize
    // Here again Rust compiler works.
    
    // Compiler analyzes your code and sees that we are using it as index to access our array
    // And it's data type for accessing arrays -> usize -> should be always positive
    // Try to change to -1 and see ))

    // Index out of bounds may be familiar to Python, Java programmers, but in C and C++
    // it happens to be that you can access a memory space which is out of bound, which leads to horrible mistakes and security issues.

    // Section 3: Functions

    // Functions are a first-class citizens. 
    // fn is a keyword used to declare them

    // Convention is snake_name for a function (surprisingly even a compiler will enforce it from you)

    // Plain function

    fn message() {
        println!("Just function");
    }
    message();

    // Function with parameters

    fn message_with_param(favnum: i32){
        println!("My favorite number is: {}",favnum);
    }
    
    message_with_param(777);

    // Its pretty much the same as in C++

    // Statements and Expressions in Functions

    // Rust distinguished statements and expressions
    // -> Statements are instructions x = 5;
    // -> Expressions are evaluated   x = 5 + 10;
    // Subtle the difference though. Key idea that statements don't return result value, 
    // expression should, which basically means it has a return type!!!! What an idea.

    // In practical terms x = y = 6 will not work :)
    // And statements closed with ;, but expressions don't

    let x = 5;
    let y = {
        // Curly braces, different scope
        let x = 3;
        x + 1 // Expression, returns the value )) 
    };

    println!("X is: {}. Y is {}",x,y);

    // Well why bother to understand , you will see this a lot in a Rust. Especially if you want to learn from other peoples code.
    

    
    fn add_five(mynum: i32) -> i32 { // notice return type after -> (very common nowdays syntax)
        mynum + 5
    }
    
    println!("Calling add_five function, relying on expression return {}",add_five(100));

    // So here comes next idea.
    // Return statement is usually explictly used when you want to return from a function earlier, based on some condition
    
    // otherwise you can rely on your last expression in the function

    // Short example on Safety in-Place

    fn subtract_five (mynum: i32) -> u8 {
        mynum as u8 - 5 // mynum - 5 not correct
    }

    println!("Calling subtract_five function, relying on expression return {}",subtract_five(6)); // try to pass 4 ))



    // Comments , by now I believe you already learned them

    // Single line comment
    
    /* 
    Multiline comment 
    or documentation comments
    of course
    */

    // Section 4: Control Flow

    // Executions based on boolean conditions If-> else if -> else expressions

    // Using conditional expressions to control the flow of your program should be familiar.

    let num = 5;
    
    if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0{
        println!("Buzz");
    } else if num % 5 == 0 {
        println!("Fizz");
    } else {
        println!("{}", num);
    }

    // Just regular if else expressions, but key word to focus here is expression
    // By the way don't worry there is even more nice syntax to handle multiple conditions, but we will talk about them when we discuss things specific to Rust. 
    // But may be if you are familiar with (when from Kotlin or match from Python, or switch from Java and Javascript you already know it)

    // => Key Idea: If is an expression

    // Let's remind to ourself : expression returns a value

    let number = 5;
    // Magic ))) Very close to ternary expression, but much better

    // C++ similar idea
    // string result = (n % 2 == 0) ? "even" : "odd";
    
    let divisible_by_two = if number % 2 == 0 {
        "even" // no semicolon, because then it becames expression
    } else {
        "odd" // both arms or branches should be evaluated to same type
    };
    
    println!("Number {} is a type of {}", number, divisible_by_two);


    // => LOOPS -> execute something again and again, till true 

    // Types of loops : LOOP,WHILE,FOR

    // You should be familiar with this
    
    // # LOOP < what a name )))
    // it's basically infinte loop, which will run until it hit a break
    
    let mut num = 0;
    loop {
        
        println!("{}",num);
        num += 1;

        if num == 10 {
            break;
        }
    }

    // But as you already know, that is not the end of the story.
    // Do you remeber how often you declared some variable in scope of the loop and needed that value back, especially with nested loops
    // but could not, and needed to create a variable outside of loop and somehow update your variable out of loop scope

    // At this point, we could say I have seen that, it's same as while true Loop,
    // techincally you are correct, but loop{} construct has unique feature.
    
    // loops can return values!
    

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is: {}", result);
    println!("Counter is: {}", counter);


    // Just regular while loop

    while counter != 0 {
        println!("Count down : {}",counter);
        counter -= 1;
    }

    // And of course, we know by heart loops exists to iterate through a collection : )

    let nums = [1,2,3,4,5];

    // for each loop

    for num in nums.iter() {
        println!("{} ", num);
    }

    // classic loop 
    for idx in 0..nums.len() { 
        // start inclusive ..end exclusive (it's fancy notation for range (Swift,Python,Kotlin))
        // whenever in doubt of type just hover your mouse over code you are interested in.
        println!("Element under idx {} := {}",idx,nums[idx]);
    }

    

    
    // Practice problems. LeetCode. With this knowledge I believe you should be able to code next problems in Rust:

    // 509. Fibonacci Number
    // https://leetcode.com/problems/fibonacci-number/

    // 278. First Bad Version
    // https://leetcode.com/problems/first-bad-version/

    // Just checking waters, no need to lookup for solutions online, 
    // if you can't dont worry )) I will disciss in class shortly.
        
}

