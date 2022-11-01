# rust_fomat
用Rust做的格式化工具, 支持Shell.

使用方法, 
```
rust_format 文件类型 文件路径
```

比如, 我有一个Shell文件, 路径`/home/user/hello.sh`, 那么
```
rust_format --shell /home/user/hello.sh
```

没有异常提示则表示格式化成功.

由于逻辑复杂, 以及不常用, 有些语法的排版是不支持的(懒得做), 包括如下:

- vim, 命令行不支持换行, 比如:
```
execute !echo "hello world"
```

将上面的写成

```
execute 
    \!echo
    \"hello world"
```
这样是不能保证排版后结果正确的.

- xml, 不支持自定义元素声明, 也就是'< ![CDTA'的部分.

