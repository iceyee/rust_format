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
#!/bin/bash
# **************************************************
# *  Author: Iceyee                                *
# *  Mail: iceyee.studio@qq.com                    *
# *  Git: https://github.com/iceyee                *
# **************************************************
#
[ $(whoami) != \"root\" ] && echo \"权限不够\"          && exit          1

# 安装
apt install rabbitmq-server -y

# 配置
rabbitmqctl add_user \"xxxx\" \"xxxxxxx\"
rabbitmqctl set_permissions -p \"/\" \"xxxx\" \".*\" \".*\" \".*\"
rabbitmqctl clear_permissions -p \"/\" \"guest\"
rabbitmqctl list_permissions --vhost /

# 启动
service rabbitmq-server restart

# 自启动
systemctl enable rabbitmq-server

# 测试
# rabbitmqctl authenticate_user \"xxxxx\" \"xxxxxx\"

# 卸载
# apt remove rabbitmq-server -y
# apt autoremove



#!/bin/bash
# **************************************************
# *  Author: Iceyee                                *
# *  Mail: iceyee.studio@qq.com                    *
# *  Git: https://github.com/iceyee                *
# **************************************************
#
[ $(whoami) != \"root\" ] && echo \"权限不够\" && exit 1

# 安装
apt install redis -y

# 配置
mkdir -p /tmp/$USER
cat /etc/redis/redis.conf | \\
sed -e '/^bind /s/[[:alnum:]]\\+\\.[[:alnum:]]\\+\\.[[:alnum:]]\\+\\.[[:alnum:]]\\+/0.0.0.0/g' | \\
sed -e '/^bind /s/::1/::/g' | \\
(sed -e '/^requirepass \\|^# requirepass /s/.*/requirepass g898gasdz/g' | \\
sed -e '/^protected-mode /s/.*/protected-mode no/g' \\
> /tmp/$USER/redis.conf         )
                cp /tmp/$USER/redis.conf /etc/redis/redis.conf

# 启动
service             redis-server restart

# 自启动
systemctl enable        redis-server

# 测试
# redis-cli -h localhost -a \"sgj98g\" PING

# 卸载
# apt purge redis* -y


f_cargo_open_doc() {
NAMES=\"\"
PATHS=\"\"
COUNTER=0
BIG_PROJECTS=$( \\
ls $HOME/git | \\
sed -n -e '/^rust$\\|^rust[^[:space:]]\\+/p'
)
for BIG_PROJECT in $BIG_PROJECTS
do
            [       -e $HOME/git/$BIG_PROJECT/Cargo.toml ] \\
&& fetch_name $HOME/git/$BIG_PROJECT/Cargo.toml \\
&& NAMES=\"$NAMES $PROJECT_NAME\" \\
&& PATHS=\"$PATHS file://$HOME/git/$BIG_PROJECT/target/doc/$PROJECT_NAME/index.html\" \\
&& COUNTER=$(($COUNTER+1))
DIRECTORIES=$(ls $HOME/git/$BIG_PROJECT         )
for DIRECTORY in        $DIRECTORIES
do
[ -e $HOME/git/$BIG_PROJECT/$DIRECTORY/Cargo.toml           ] \\
&& fetch_name $HOME/git/$BIG_PROJECT/$DIRECTORY/Cargo.toml \\
&& NAMES=\"$NAMES $PROJECT_NAME\" \\
        &&           PATHS=\"$PATHS file://$HOME/git/$BIG_PROJECT/$DIRECTORY/target/doc/$PROJECT_NAME/index.html\" \\
&& COUNTER=$((  $COUNTER+1      ) )
done
done

a_menu          \"打开rust文档\" \"$NAMES\"
read -p         \">>> \" INDEX
browse $(   a_array_get \"$PATHS\" \"$INDEX\"    )
}


source http_proxy_config.sh
[ ! -e /usr/bin/ssserver ] \\
&& curl \\
-L \\
-x \"${PROXY_HOST}:${  PROXY_PORT  }\" \\
-U \"${PROXY_USER_NAME}:${PROXY_PASSWORD}\" \\
-O \"https://github.com/shadowsocks/shadowsocks-rust/releases/download/v1.14.3/shadowsocks-v1.14.3.x86_64-unknown-linux-gnu.tar.xz\" \\
&&xz -d shadowsocks-v1.14.3.x86_64-unknown-linux-gnu.tar.xz \\
&&sudo tar -C /usr/bin -vxf shadowsocks-v1.14.3.x86_64-unknown-linux-gnu.tar \\
&&              rm shadowsocks-v1.14.3.x86_64-unknown-linux-gnu.tar


if [ ! -d /home/ljq ]; then
echo \"添加账号ljq:870\"
useradd ljq
echo -e \"870\\n870\" | passwd ljq
mkdir /home/ljq
chown ljq:ljq /home/ljq
fi

";

#[test]
fn test_shell() {
    use rust_format::Formatter;
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::shell::ShellFormatter::format(TEXT)
    );
    return;
}
