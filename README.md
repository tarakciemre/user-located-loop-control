Author: Emre Tarakcı
Date: 03.12.2022
# User Located Loop Control Mechanisms
## Introduction
User located loop control mechanisms allow the program to exit the code before the loop control statement. That is, inside the loop statements, the program can exit the loop by a statement or continue from the next iteration. This is a convenient feature because there are many cases where it might make sense to break out of a loop because the rest of the loop is not needed anymore, or in a case where we want to continue from the next statement, instead of enclosing everything in a loop block in curly braces to check for it, we can simply use a continue statement to bypass this iteration of the loop according to a condition. In this report, the implementation of user located loop control mechanisms will be examined in languages Dart, Javascript, Lua, PHP, Python, Ruby, and Rust.
## Dart
In dart, the labeled break and continue statements are supported. Labels are written in front of the loop with “:” between the label and the loop. It supports unconditional exits. When a label is not given, the break or continue statement affects the closest loop that encloses the statement [1].
In the example below, continue statement is demonstrated. The loop continues with even values of i, so only the odd values of i are printed [2].
```
   var i = 0; 
   for(i = 0; i < 10; i++) { 
      if (i % 2 == 0) continue;
      print("i: $i"); 
   }
```
Output:
```
i: 1
i: 3
i: 5
i: 7
i: 9
```

In a more complex example demonstrating the labels, I have created two labeled loops, outer and inner. The innermost while loop is used to demonstrate that unlabeled break statement exits only that closest loop, so the middle and outer loops keep working. When break middle is used, the loop labeled middle and all the loops inside are bypassed and outer loop continues. Since the break outer statement is used in the case of j > 3, the loop outputs the values of j from 0 to 3. This demonstrates that complex loops can be created in Dart language.
```
   var j = 0;
   outer: while (true) { 
      middle: while(true) {
          while (true) {
              while (true) {
                print("Inside the deepest loop $j");
                if(j >= 3) {
                   break outer;
                }
                else j++;
                break;  
              }
              break middle;
          }
      }
   }
```
Output:
```
Inside the deepest loop 0
Inside the deepest loop 1
Inside the deepest loop 2
Inside the deepest loop 3
```

## Javascript
In Javascript, labeled break and continue statements are also supported, similar to dart. the label and “:” symbol are put before the loop to denote its label. To break from a loop, the label of the loop can be put after break or continue statement. Javascript supports unconditional exit statements [3]. 
In the example below, when break inner is called, only the inner loop is exited and the outer loop keeps its operation. When i > 3, the break statement is executed for the outer loop and therefore the code only outputs the values of i from 0 to 3. 
```
outer:
for (let i = 0; i < 100; i++) {
    console.log(i)
    inner:
    while (true) {
        if (true) {
            break inner;
        }
    }
    if (i >= 3) {
        break outer;
    }
}
```
Output:
```
0
1
2
3
```

In the example below, labeled continue statement is demonstrated. When values of j are even, the continue statement is executed. As a result, only the odd values of j are logged to the console.
```
let j = 0;

whileloop:
while (j < 10) {
 j++;
 if (j % 2 == 0) continue whileloop;
 console.log(j)
}
```
Output:
```
1
3
5
7
9
```

## Lua
Lua does not support labeled exit and control statements. Only unlabeled break statement can be used,  and that exits the closest loop that encloses the break statement. Lua also does not support continue statement [4] [5].
In the example below, the way break works in Lua is demonstrated. When i is larger than 3, the if statement becomes true and the break statement is executed, therefore the loop is exited and only the values from 0 to 3 are logged to console.
```
i = 0
while( true )
do
   print("i:", i)
   if(i >= 3)
   then
      break
   end
    i=i+1	
end
```
Output:
```
i:	0
i:	1
i:	2
i:	3
```

## PHP
PHP provides a different approach to break and continue statements. When a break or a continue statement is used without a numeric value in front of it, the statement affects the closest loop that encloses the statement. However, the user can also denote the degree of the loop that the break or continue will affect. For example, if “break 2;” statement is used, the number 2 means that the statement will break or continue from the second closest loop that encloses the statement [6] [7]. If the numeric value entered is larger than the nesting degree of the loop, PHP produces an error. Meaning, if there are two nested loops, and a ‘break 5’ statement is executed, an error occurs because there is no such loop that can be exited.
The example below demonstrated the numbered approach of PHP. When the statement “continue 3” is executed, the outmost loop is exited. For that reason, the string “This is the inner loop” is logged to the console only once per iteration of the outer loop. Since the outer loop executes 3 times, we see the following output in the console.
```
$i = 0;
while ($i < 3) {
    echo "-- Loop when i = $i\n";
    $i++;
    while (true) {
        while (true) {
            echo "This is the inner loop\n";
            continue 3;
        }
        echo "This is the loop in the middle\n";
    }
}
```
Output:
```
-- Loop when i = 0
This is the inner loop
-- Loop when i = 1
This is the inner loop
-- Loop when i = 2
This is the inner loop
```


When break or continue is used without any number in front of it, if affects the closest loop that encloses that statement. An example of this can be seen below:
```
for ($j = 0; $j < 3; $j++) {
    echo "-- Loop when j = $j\n";
    while(true) {
        break;
        echo "The inner loop\n";    
   }
}
```
Output:
```
-- Loop when j = 0
-- Loop when j = 1
-- Loop when j = 2
```

An alternative statement: “goto” can also be possibly used, but it is not as practical and even the PHP documentation advises against its use [8].
The implementation of labeled break and continue statements is different from the other languages in the list and it is simple, as well as easy to read and write. Since finding appropriate names for variables and functions is not very easy, PHP’s implementation saves the programmers from having to find names for loops also.
## Python
In python, break and continue statements are used as user located loop control mechanisms. Python only allows usage of these statements without any labels. In this case, the break or continue statement is always affecting the innermost loop [9].
In the example below, the outer loop is executed three times. In the meanwhile, the inner loop is executed for every character of the given string. The break statement breaks out of the inner loop when the character is c, therefore the rest of the string after the first ‘c’ character is not printed out. Since the break does not affect outer loop, the inner loop is executed three times.
```
for i in range(3):
	print()
	for letter in "abcabcabcabc":
		if letter == "c":
			break
		print(str(letter), end="")
```
Output:
```
ab
ab
ab
```

In the example below, continue statement is executed whenever the iterated character equals character ‘c’. This results in an output where every time the c character occurs in the string, the print statement is ignored because the loop continues to the next iteration. It can be seen that, the outer for loop is executed three times. This means that the continue statement in the inner loop does not affect the outer loop.
```
for i in range(3):
	print("\nLoop at i = " + str(i))
	for letter in "abcabcabcabc":
		if letter == "c":
			continue
		print(str(letter), end="")
```
Output:
```
Loop at i = 0
abababab
Loop at i = 1
abababab
Loop at i = 2
abababab
```

Upon my research, I have seen that it was proposed for Python to have labeled continue and break statements, but the developers rejected this proposition, citing the fact that it would add complexity to the language and one of the strengths of Python is its simplicity [10].
## Ruby
In ruby, break and next statements are used as user located loop control mechanisms. Similar to Python, these statements do not have labeled implementations [11].
In the example below, I have implemented the edge cases of the Ruby implementation of break and next statements. In the outer loop, variable i is iterated with values from 0 to 5. When the value of i is even, the next statement is executed and the outer loop is continued from the next value of i. This works in the same way as continue but the word used is next instead. In the inner while loop, there is a break if statement. This is an easy way to implement a break statement. As it can be seen in the output, the outer loop continues when the break statement is executed. This demonstrates the fact that next and break statements affect the closest loop that embodies the next or the break statement.
```
for i  in  0..5
    puts "Loop at i = #{i}:"
    if  i % 2 == 0 then
        next
    end
    j = 1
    while true
        puts "j: #{j}"
        j += 1
        break if j > 2
    end
end
```
Output:
```
Loop at i = 0:
Loop at i = 1:
j: 1
j: 2
Loop at i = 2:
Loop at i = 3:
j: 1
j: 2
Loop at i = 4:
Loop at i = 5:
j: 1
j: 2
```

Ruby simple in its implementation of the user located loop control mechanisms. It allows for conditional break statement with “break if” statement. That sets Ruby apart from the other languages discussed in this homework.
Rust
In rust, the break and continue statements are provided for loop control. Rust allows labeled loop control as well as unlabeled. if a break or continue statement is not followed by a loop label, rust automatically breaks or continues the innermost loop [12] [13]. 
In the example below, labeled exit statement is demonstrated. Since the break statement is applied on the outer loop, the string “outer2.” is not logged to the console.
```
    'outer2: while true {
        'inner2: while true {
            println!("inner2.");
            break 'outer2;
        }
        println!("outer2.");
    }
 ```
 Output:
 ```
inner2.
```

In the example below, we can see how the unlabeled continue statement makes the inner loop continue. This can be seen from the fact that only odd numbers are printed in the output, but the inner loops keep working for the even numbers in the inner loop.
The labeled continue, on the other hand, makes the outer loop continue, so the outer loop immediately continues from the next value of i and the j value for the inner loop is reset.
```
    'outer3: for i in 0..10 {
        print!("\nouter: {} , inner: ", i.to_string());
        'inner3: for j in 0..10 {
            if ( j % 2 == 0) {
                continue;
            }
            if ( j > i) {
                continue 'outer3;
            }
            print!("{} ", j.to_string());
        } 
    }
```
Output:
```
outer: 0 , inner: 
outer: 1 , inner: 1 
outer: 2 , inner: 1 
outer: 3 , inner: 1 3 
outer: 4 , inner: 1 3 
outer: 5 , inner: 1 3 5 
outer: 6 , inner: 1 3 5 
outer: 7 , inner: 1 3 5 7 
outer: 8 , inner: 1 3 5 7 
outer: 9 , inner: 1 3 5 7 9 
```
## Conclusion and Evaluation
In this report, the implementations of user located loop control mechanisms were examined in Dart, Javascript, Lua, PHP, Python, Ruby, and Rust. While the implementation of unlabeled statements were the same for these languages with minor changes such as Lua not having a continue statement and ruby calling the control statement ‘next’ instead of  ‘continue’. When it comes to labeled loop control mechanisms, these languages varied greatly. In Rust for example, the loop names are denoted with a ‘ symbol at the beginning and : symbol after label name. Ruby, Python and Lua do not support labeled loops. Javascript and Dart support it with a similar implementation to Rust, but the label names are not restricted.  PHP, on the other hand, supports it using a number rather than label. In my opinion, Javascript, Dart and Rust allow the programmer for writing very specific loop scenarios and they are very versatile in that manner. Rust restricting the naming of labels may make reading a bit better than the other languages. But overall, the readability aspect was similar in these languages. But among all, my favorite was PHP. That is because giving descriptive names to variables and functions is an important part of programming. In the case of labels, the labels might be hard to find for the programmer and the language may suffer from writability issues. Also, I think that the labels make the code loop more complicated and bloated. I believe that PHP found the best solution without complicating the code, since it can handle what the labeled implementations can do, and the programmer does not have to label the loops for making a labeled loop. The other side of that argument might be that labels may be easy to point at if there are many nested loops instead of counting the loop degree as it might be required in PHP. However, I believe that would be a rare case and PHP implementation’s advantages outweigh its shortcomings.
## Learning Strategy
In order to learn the behaviour of user located loop control mechanisms in these languages, I have read official documentations of the languages and looked at tutorials on the internet. When I got errors in my code, I have found the solutions using Google and Stack Overflow. I have created programs in these languages that would specifically test for the edge cases in the language. For example, since I knew that PHP was a language that had a different implementation using numbers, I have written code in which I could explore how it behaved in the absence of a number and different things that can be done with this numbered implementation. In other cases with labeled loops, I particularly used the example where the program would only output the odd values for testing the continue statement. That provided a universal way to test how continue works in each of these languages without thinking of new examples in each of them. I specifically tested if a plain break or continue statement only affects the closest loop that encloses the statement, and that was the case in all of these languages. During my experiments, I thought that maybe some of these languages would break out of the outer loop with a unlabeled break statement, but that was not the case. This project helped me quickly adapt to new syntax for new languages and learn new properties of these languages by reading the documentation, checking code examples and implementations.

## Bibliography
[1] Break statement in Dart Programming. Tutorials Point. (n.d.). Retrieved December 3, 2022, from  https://www.tutorialspoint.com/dart_programming/dart_programming_break_statement.htm 
[2] Dart programming - continue statement. Tutorials Point. (n.d.). Retrieved December 3, 2022, from https://www.tutorialspoint.com/dart_programming/dart_programming_continue_statement.htm 
[3] Break - javascript: MDN. JavaScript | MDN. (n.d.). Retrieved December 3, 2022, from https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/break 
[4] Lua continue: How continue statement works in Lua: Examples. EDUCBA. (2021, May 11). Retrieved December 3, 2022, from https://www.educba.com/lua-continue/ 
[5] Lua - break statement. Tutorials Point. (n.d.). Retrieved December 3, 2022, from https://www.tutorialspoint.com/lua/lua_break_statement.htm 
[6] Break - Manual. php. (n.d.). Retrieved December 3, 2022, from https://www.php.net/manual/en/control-structures.break.php 
[7] Continue - manual. php. (n.d.). Retrieved December 3, 2022, from https://www.php.net/manual/en/control-structures.continue.php 
[8] Goto - Manual. php. (n.d.). Retrieved December 3, 2022, from https://www.php.net/manual/en/control-structures.goto.php 
[9] 4. more control flow tools¶. 4. More Control Flow Tools - Python 3.11.0 documentation. (n.d.). Retrieved December 3, 2022, from https://docs.python.org/3/tutorial/controlflow.html 
[10] [python-3000] announcing PEP 3136. (n.d.). Retrieved December 3, 2022, from https://mail.python.org/pipermail/python-3000/2007-July/008663.html 
[11] Control expressions¶ ↑. control_expressions - Documentation for Ruby 2.4.0. (n.d.). Retrieved December 3, 2022, from https://docs.ruby-lang.org/en/2.4.0/syntax/control_expressions_rdoc.html 
[12] Rust by example. Nesting and labels - Rust By Example. (n.d.). Retrieved December 3, 2022, from https://doc.rust-lang.org/rust-by-example/flow_control/loop/nested.html 
[13] Keyword break. Rust. (n.d.). Retrieved December 3, 2022, from https://doc.rust-lang.org/std/keyword.break.html 

