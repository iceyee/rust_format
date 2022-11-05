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

// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
function        GetDateTime  (   date  ) {
        var         result      = \"\";
result += date.getFullYear();
if (    date.getMonth()     <   10  ) {
result  +=      \"-0\";
}       else  {
result      +=      \"-\";
}
result      +=      (date.getMonth() + 1);
if      (date.getDate() < 10) {
result += \"-0\";
} else {
result+=\"-\";
}
result += date.getDate();
if (date.getHours() < 10) {
result += \" 0\";
} else {
result += \" \";
}
result += date.getHours();
if (date.getMinutes() < 10) {
result += \":0\";
} else {
result += \":\";
}
result += date.getMinutes();
if (date.getSeconds() < 10) {
result += \":0\";
} else {
result += \":\";
}
result += date.getSeconds();
return result;
}

var a = /hello world\\//g;

var b = a/2;
var c = a;//2;

";

#[test]
fn test_js() {
    use rust_format::Formatter;
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::js::JavascriptFormatter::format(TEXT)
    );
    return;
}
