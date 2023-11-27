// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

// Enum.

// Trait.

// Struct.

// Function.

const TEXT: &str = "
<html xmlns=\"http://www.w3.org/1999/xhtml\"><head>
<meta charset=\"UTF-8\"/>
<meta name=\"generator\" content=\"HTML Tidy for HTML5 for Linux version 5.6.0\"   /  >
<title>查看Buff的统计数据</title>
<meta http-equiv=\"content-language\" content=\"zh-cn\"/>
<meta name=\"description\" content=\"This site is developed by Iceyee.\"/>
<meta name=\"viewport\" content=\"width=device-width, user-scalable=no, max-scale=1\"/>
<script src=\"/\"></script>
<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@4.6.1/dist/css/bootstrap.min.css\" rel=\"stylesheet\"/>

<style>
    something1
    something2
</style>
<style>something3
    something2
</style>
</head>
<body>
<div id=\"BODY\" class=\"container py-3\">
<p>
<a href=\"./html/chu_shou_tong_ji2.html\">出售记录</a>
</p>
<p>
<a href=\"./html/gou_mai_tong_ji_60.html\">购买统计</a>
</p>
<p>
<a href=\"./html/shi_chang_hang_qing.html\">市场行情</a>
</p>
<p>
<a href=\"./html/zhi_xiao_ji_shu.html\">滞销统计</a>
</p>
<p>
<a href=\"./html/zi_jin_tong_ji.html\">资金统计</a>
</p>
</div>
<script src=\"https://cdn.jsdelivr.net/npm/jquery@3.6.0/dist/jquery.min.js\"></script>

<script src=\"https://cdn.jsdelivr.net/npm/bootstrap@4.6.1/dist/js/bootstrap.bundle.min.js\"></script>

<script src=\"https://cdn.jsdelivr.net/npm/vue@2\"></script>
<script src=\"https://cdn.jsdelivr.net/npm/vue@2\">

var a == \"hello world\"
var b == a > b
var c == a < b

</script>


</body></html>


<pre 
id=\"TEXT\" 
class=\"py-3\" 
style=\"font-size: 1em; line-height: 2em; white-space: pre-wrap;\">{{text}}</pre>

";

#[test]
fn test_html() {
    use rust_format::Formatter;
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::html::HtmlFormatter::format(TEXT)
    );
    return;
}

