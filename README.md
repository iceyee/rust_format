# rust_fomat
用Rust做的格式化工具, 支持Shell.

使用方法, 
```bash
rust_format 文件类型 文件路径
```

比如, 我有一个Shell文件, 路径`/home/user/hello.sh`, 那么
```bash
rust_format --shell /home/user/hello.sh
```

没有异常提示则表示格式化成功.

## 目前支持的文件类型
- html
- shell
- vim
- xml

## 部分不支持的功能

由于逻辑复杂, 以及不常用, 有些语法的排版是不支持的(懒得做), 包括如下:

- vim, 命令行不支持换行, 比如:
```
execute !echo "hello world"
```

将上面的写成

```vim
execute 
    \!echo
    \"hello world"
```
这样是不能保证排版后结果正确的.

- xml, 不支持自定义元素声明, 也就是'< ![CDTA'的部分.
- html, 由于用的排版规则同xml(区别是xml的缩进+4, html的缩进+2), 所有标签必须符合xml规则, 以及不支持读<script></script>中间的内容, 因为'<'和'>'这两个比较运算符, 会被解释成标签开始和标签结束.

## 后记
由于个人精力有限, 且赶时间, 所以只做了主要的功能, 上面说的不支持的功能, 等等以后有时间了再慢慢补充.


