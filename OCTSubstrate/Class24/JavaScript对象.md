```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <title></title>
        <script type="text/javascript">
            // 创建对象
            var obj = new Object();
            var obj2 = {};
            var obj3 = {
                name:"猪八戒", 
                age:28,
                brother:{
                    name:"沙和尚"
                } 
            };
            // 添加属性
            obj.name = "孙悟空";
            obj["gender"] = "男";
            obj.sayNmae = function(){
                console.log(obj.name);
            };
            // 删除属性
            delete obj.name;
            // in 运算符
            console.log("gender" in obj);
            
            /**
             * 函数也是一个对象
            */
            // 创建一个函数对象
            var fun = new Function();
            // 可以将要封装的代码以字符串的形式传递给构造函数，基本不用
            // var fun2 = new Function("console.log('Hello');");
            // 使用函数声明创建一个函数
            function fun2(){
                console.log("Second Function");
            }
            // 调用函数语法就是函数名加()
            fun2();
            // 声明匿名函数并赋给fun3
            var fun3 = function(){
                console.log("Second Function");
            };
            fun3()
            // 参数和返回值可以是任何东西，任何东西在JS里都是对象，函数，对象。。。
            function sum(a, b){
                var c = a+b;
                return c;
            }
            function fun3(fun2){}
            // 创建一个构造函数来创建对象
            function Person(name, age, gender){ // 普通函数和构造函数的区别就是用不用new调用
                this.name = name, 
                this.age = age,
                this.gender = "男"
                this.sayName = function(){ // 这里每次new都会创建一个新的sayName方法，这是个问题，解决方法如下
                    alert(this.name);
                };
            }
            var per = new Person("孙悟空",18,"男");

            function Person2(name, age, gender){ // 普通函数和构造函数的区别就是用不用new调用
                this.name = name, 
                this.age = age,
                this.gender = "男"
                this.sayName = say;
            }

            function say(){
                    alert(this.name);
            };
            // Person和per1有相同的原型对象，prototype，我们可以直接在原型对象上加入属性，所有对象都能访问到原型上的属性和方法。
            // Person.prototype.a = "Hello"; 
            // 所以我们可以往prototype中加方法，这样就不用再全局作用域里放方法了，安全
            // Person.prototype.sayName = function(){
            //      alert(this.name);
            // };
            // 如果自己写了和原型中的属性和方法名一样的东西就会覆盖，调用时候先调用自身的，如果没有会去原型中找

            /**
             * 数组(Array)
             * 数组也是一个对象
            */
            // 创建数组对象
            var arr = new Array();
            arr[0] = 10;
            console.log(arr[0]);
            var arr2 = [1, 2, 3, 4, 5];
        </script>
    </head>
    <body>
    </body>
</html>
```
## =>箭头函数
const materials = [
  'Hydrogen',
  'Helium',
  'Lithium',
  'Beryllium'
];

console.log(materials.map(material => material.length));
// expected output: Array [8, 6, 7, 9]

基础语法
(param1, param2, …, paramN) => { statements }
(param1, param2, …, paramN) => expression
//相当于：(param1, param2, …, paramN) =>{ return expression; }

// 当只有一个参数时，圆括号是可选的：
(singleParam) => { statements }
singleParam => { statements }

// 没有参数的函数应该写成一对圆括号。

高级语法
//加括号的函数体返回对象字面量表达式：
params => ({foo: bar})

//支持剩余参数和默认参数
(param1, param2, ...rest) => { statements }
(param1 = defaultValue1, param2, …, paramN = defaultValueN) => {
statements }

//同样支持参数列表解构
let f = ([a, b] = [1, 2], {x: c} = {x: a + b}) => a + b + c;
f();  // 6
Copy to Clipboard
() => { statements }

## 扩展运算符
三个点（...）真名叫扩展运算符
说白了就是把衣服脱了，不管是大括号（[]）、花括号（{}），统统不在话下，全部脱掉脱掉！
// 数组
var number = [1,2,3,4,5,6]
console.log(...number) //1 2 3 4 5 6
//对象
var man = {name:'chuichui',height:176}
console.log({...man}) / {name:'chuichui',height:176}

## async和await
```javascript
function edition(num){
                return new Promise((resolve, reject)=>{
                    setTimeout(() => {
                        resolve(2*num)
                    }, 2000)
                })
            }
            async function attract(){
                console.log(1);
                let result1 = await edition(10);
                return result1;
            }
            attract().then((res)=>{
                console.log(res);
            }, (res)=>{
                console.log("运行错误：" + res);
            })
```
程序第一肯定会输出1，然后进入attract函数内部，接着输出2，然后看见了await，等待edition函数的返回值，此时在attract函数中的代码是被阻塞的，但是外部不会被阻塞，所以就接着输出3。

然后两秒后，输出edition函数返回的4，最后回到attract函数then方法的第一个回调函数，输出了5。
