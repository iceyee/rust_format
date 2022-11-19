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
<!doctype html>


<?xml version=\"1.0\" encoding=\"UTF-8\"?><project><modelVersion>4.0.0</modelVersion><groupId>iceyee</groupId> <!-- hello world. --> <artifactId>iceyee-buff-buy</artifactId><version>2.0</version><packaging>pom</packaging><name>iceyee-buff-buy</name><url>http://maven.apache.org</url><properties><project.build.sourceEncoding>  UTF-8           </project.build.sourceEncoding></properties><modules><module>counter</module     ><module>timetomarket</module><module>blacklist</module><module>purchasefrequency</module><module>yue</module><module        >reportdate</module     ><module     >        app        </module></modules><br/>

<!--123-->
</project>


";

#[test]
fn test_xml() {
    use rust_format::Formatter;
    println!("");
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::xml::XmlFormatter::format(TEXT)
    );
    return;
}
