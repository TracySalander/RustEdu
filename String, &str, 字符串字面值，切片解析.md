# String, &str, 字符串字面值，切片

`let a = "ok"`; 

`let b = "hello world";`

a和b是&str 

&str => String:

`let c = a.to_string();` //方法一

`let d = String::from()` //方法二

String => &str: 直接加一个&

`let e = &String::from("hello world");` //方法一

`let e_0 = String::from("hello world");` 

`let e = e_0.as_str();` // 方法二，用as_str()

String加长的方法：

一个String + 一个&str=>String

另一个方法是用push_str()

"ok", "hello world"就是字面值

String和&str都不能直接进行切片，必须要加&

