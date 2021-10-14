```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <title></title>
        <!--JS代码需要编写到script标签中--> 
        <script type="text/javascript">
            /*
             * 控制浏览器弹出一个警告框
             */
            // alert("这是我的第一行JS代码"); // 弹框

            /*
             * 让计算机在页面中输出一个内容
             */ 
            // document.write("页面中的第一个内容"); // 可以向body中输出一个内容

            /*
             * 向控制台输出一个内容
             */
            // console.log("你猜我在哪儿？"); // 页面中没有，控制台显示

        </script>
    </head>
    <body>
        <!--可以将js代码编写到标签的onclike属性中
            当我们点击按钮时, js代码才会执行
            
            虽然可以写在标签的属性中，但是他们属于结构与行为耦合，不方便维护，不推荐使用
        -->
        <button onclick="alert('为什么要点我？')">点我一下</button>
        
        <!--可以将js代码写在超链接的href属性中，这样当点击超链接时，会执行js代码-->
        <a href="javascript:alert('让你点你就点');">也点我一下啊</a>

        <a href="javascript:;">也点我一下啊</a> 
    </body>
</html>
```
外部引入，外部文件名script.js
```javascript
alert("我是外部文件中的JS代码");
```
```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <title></title>
        <!--
            可以将js代码编写到外部js文件中，然后通过script标签引入
            写到外部文件中可以在不同的页面中同时引用，也可以利用到浏览器的缓存机制
            推荐使用的方式
        --> 
        <!--
            script标签一旦用于引入了外部文件了，就不能在编写代码了，即使编写了浏览器也会忽略
            如果需要则可以再创建一个新的script标签用于编写内部代码
        -->
        <script type="text/javascript" src="script.js"></script>
        <script type="text/javascript">
            alert("我是内部的js代码");
        </script>
    </head>
    <body>
    </body>
</html>
```
```html
<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <title></title>
        <script type="text/javascript">
            // 声明变量
            var a;
            // 为变量赋值
            a = 123;
            // 声明与赋值同时进行
            var b = 234;
            /**
             * 数据类型
             * 基本数据类型：
             * String
             *  var str = "hello"; 
             * Number
             *  Number.MAX_VALUE; // 如果比这个大的话返回Infinity
             *  "abc" * "bcd"返回NaN(Not a Number)
             * Boolean true/false
             * Null null 
             *  var a  = null，用typeof返回object
             * Undefined undefined 只声明但是没赋值
             * -----------------------
             * Object
            */
            // 用typeof可以检查变量类型
            // console.log(typeof a);
            /**
             * 强制类型转换
             * var a = 123;
             * a = a.toString(); //方法
             * a = String(a); //函数
             * a = a + "";
             * Number(a);
             * a = a - 0;
             * a = +a;
             * a = "123.456px";
             * b = parseInt(a); // 从字符串中取出连续整数数字开头内容
             * b = parseFloat(a);
             * Boolean(0);
            */
            /**
             * !, &&, ||
             * =， +=， -=
             * >, <, <=, ==
             * var max = a > b ? a : b;
            */
            {
                var a = 10;
                var b = 20; // {}代码块只有分组作用，外部还是可以访问的
            }

            if(b > 20){
                alert("1");
                alert("2");
            } else if(b > 10){

            }else{

            };

            switch(a){
                case 10:
                case 9:
                case 8:
                    console.log("pass");
                    break;
                default:
                    console.log("failed")
                    break;
            }

            while(a > 8){
                a--;
            }

            do{
                a++;
            } while (a < 10);

            for(var i = 0; i < 10; i++){

            }

            // break continue
        </script>
    </head>
    <body>
    </body>
</html>
```
