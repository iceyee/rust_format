# rust_fomat
用Rust做的代码格式化工具.

## 使用方法

```bash
rust_format 文件类型 文件路径
```

比如, 我有一个shell文件, 路径是`/home/user/hello.sh`, 那么执行命令
```bash
rust_format --shell /home/user/hello.sh
```

没有异常提示则表示格式化成功.

## 目前支持的文件类型

- html
- javascript
- shell
- vim
- xml

## 部分不支持的功能

由于逻辑复杂, 以及不常用, 有些语法的排版是不支持的(懒得做), 包括如下:

#### vim, 代码和注释不要写在同一行, 因注释会被错误地解释为字符串, 下面举例.

错误的写法:
```vim
echo "hello world" " 这里是注释
```

正确的写法:
```vim
" 这里是注释
echo "hello world"
```

#### html, 因为用的排版规则同xml(区别是xml的缩进+4, html的缩进+2), 所以所有标签必须符合xml规则.

## 后记

由于个人精力有限, 且赶时间, 所以只做了主要的功能, 上面说的不支持的功能, 等等以后有时间了再慢慢补充.


