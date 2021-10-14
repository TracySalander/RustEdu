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
