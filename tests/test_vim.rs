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

syntax clear            
syntax on              
set autoindent        
set autoread         
set autowrite       
set cursorline     
set foldenable    
set nobackup            
set nocompatible       
set nopaste           
set ruler            
set showcmd         
set showmode            
set smartindent        
set foldmethod=marker 
set encoding=utf-8
set linespace=4
set shiftwidth=4        
set softtabstop=4
set tabstop=4
\" /是Makefile文件/, 不扩展\\t, 否则扩展\\t.
if  expand(         \"%:p:t\"           )       =~?         'Makefile'
set noexpandtab
else
set expandtab
endif
\" 打开文件时, 光标回到该文件最后一次退出时的位置.
let g:PF = {
\\\"F2\": \"Menu\",
\\\"F3\": \"Write\",
\\\"F4\": \"Quit\",
\\\"F5\": \"Comment\",
\\\"F6\": \"Uncomment\",
\\\"F7\": \"Run Or Browse\",
\\\"F8\": \"Build Or Check Syntax\",
\\\"F9\": \"Format\",
\\\"F10\": \"Default Template\",
\\\"F11\": \"Vertical Window\",
\\\"F12\": \"Horizen Window\",
\\}
let g:CF = {
\\\"C_F5\": \"Custom_1\",
\\\"C_F6\": \"Custom_2\",
\\\"C_F7\": \"Custom_3\",
\\\"C_F8\": \"Custom_4\",
\\\"C_F9\": \"Custom_5\",
\\\"C_F10\": \"Custom_6\",
\\\"C_F11\": \"Custom_7\",
\\\"C_F12\": \"Custom_8\",
\\}
let g:SF =          {
\\\"S_F5\": \"Copy_1\"                  ,
\\\"S_F6\": \"Paste_1\",
\\\"S_F7\": \"Copy_2\",
\\\"S_F8\": \"Paste_2\",
\\\"S_F9\": \"Reopen\",
\\\"S_F10\": \"Shell\",
\\\"S_F11\": \"source /tmp/test.vim\",
\\\"S_F12\": \"{Undefined}\",
\\}
let g:RG = {
\\\"@n\": '\"\\\\n\" => \"\\n\"',
\\\"@r\": \"Random Number\",
\\}
let g:IS = []

\" 加载所有的脚本.
\" if !isdirectory(             $HOME.\"/.iceyee_vim\"           )
\"     exit
\" endif
let             s:file_name          =       expand(\"%:p:t\")
let s:extension = expand(\"%:e\")
let s:path=$HOME.\"/.iceyee_vim\"
for s:module in readdir(s:path)
if !isdirectory(s:path.\"/\".s:module)
continue
endif
for s:file in readdir               (s:path.\"/\".s:module, {       name -> name =~ '^g_.*\\.vim' })
call execute(\"source \".s:path.\"/\".s:module.\"/\".s:file)
endfor
for s:file in readdir(s:path.\"/\".s:module, { name -> name =~ '^e_.*\\.vim' })
if s:extension == s:file[2 : len(s:file)-5]
call execute(\"source \".s:path.\"/\".s:module.\"/\".s:file)
endif
endfor
for s:file in readdir(s:path.\"/\".s:module, { name -> name =~ '^f_.*\\.vim' })
if s:file_name == s:file[2 : len(s:file)-5]
call execute(\"source \".s:path.\"/\".s:module.\"/\".s:file)
endif
endfor
endfor


call setreg(\"n\", ':s/ *\\\\n/\\r/g'.\"\\n\", \"c\")
call setreg(\"r\", \":!expr $RANDOM \\\\% 1000\\n\", \"c\")
nmap <F2> :call KeyboardShortcut()<CR>
nmap <F3> :write<CR>
nmap <F4> :quit!<CR>
nmap <F5> :call GlobalComment(g:comment_symbol)<CR>
vmap <F5> :call GlobalComment2(g:comment_symbol)<CR>
nmap <F6> :call GlobalUncomment(g:comment_symbol)<CR>
vmap <F6> :call GlobalUncomment2(g:comment_symbol)<CR>
nmap <F7> :call RunOrBrowse()<CR>
nmap <F8> :call BuildOrCheckSyntax()<CR>
nmap <F9> :call Format()<CR>
nmap <F10> :call DefaultTemplate2()<CR>
nmap <F11> :split! %:h<CR>
nmap <F12> :vsplit! %:h<CR>
nmap <C-F5> :call Custom1()<CR>
nmap <C-F6> :call Custom2()<CR>
nmap <C-F7> :call Custom3()<CR>
nmap <C-F8> :call Custom4()<CR>
nmap <C-F9> :call Custom5()<CR>
nmap <C-F10> :call Custom6()<CR>
nmap <C-F11> :call Custom7()<CR>
nmap <C-F12> :call Custom8()<CR>
nmap <S-F5> :call GlobalCopy(\"vim_paste_1\")<CR>
vmap <S-F5> :call GlobalVCopy(\"vim_paste_1\")<CR>
nmap <S-F6> :call GlobalPaste(\"vim_paste_1\")<CR>
nmap <S-F7> :call GlobalCopy(\"vim_paste_2\")<CR>
vmap <S-F7> :call GlobalVCopy(\"vim_paste_2\")<CR>
nmap <S-F8> :call GlobalPaste(\"vim_paste_2\")<CR>
nmap <S-F9> :write<CR>:edit<CR>:source ~/.vimrc<CR>:echo \"Reopen\"<CR>
nmap <S-F10> :call execute(\"!\".GlobalGreenPrintCommand(\"现在是从Vim打开终端, 按[CTRL+D]退出\"))<CR>:shell<CR>
nmap <S-F11> :source /tmp/test.vim<CR>
\" nmap <S-F12> :undefined
imap <C-X><C-A> <C-R>=InsertTemplate12()<CR>
imap <C-X><C-B> <C-R>=InsertTemplate22()<CR>
imap <C-X><C-C> <C-R>=InsertTemplate32()<CR>
vmap <F8> :call GlobalSort()<CR>
\" 打印快捷键菜单.
let s:line = printf(
\\\"%5s - %s\\n\", 
\\\"i:<C_X><C_A>\",
\\\"Template_1\")
call add(g:IS, s:line)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\\"i:<C_X><C_B>\",
\\\"Template_2\")
call add(g:IS, s:line)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\\"i:<C_X><C_C>\",
\\\"Template_3\")
call add(g:IS, s:line)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\\"v:F8\",
\\\"Sort\")
call add(g:IS, s:line)
function! KeyboardShortcut()
let s:text_1 = []
for [s:key,s:value] in items(g:PF)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\s:key, 
\\s:value)
call add(s:text_1, s:line)
endfor
let s:text_2 = []
for [s:key,s:value] in items(g:CF)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\s:key, 
\\s:value)
call add(s:text_2, s:line)
endfor
let s:text_3 = []
for [s:key,s:value] in items(g:SF)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\s:key, 
\\s:value)
call add(s:text_3, s:line)
endfor
let s:text_4 = []
for [s:key,s:value] in items(g:RG)
let s:line = printf(
\\\"%5s - %s\\n\", 
\\s:key, 
\\s:value)
call add(s:text_4, s:line)
endfor
let s:text_1 = sort(s:text_1)
let s:text_2 = sort(s:text_2)
let s:text_3 = sort(s:text_3)
let s:text_4 = sort(s:text_4)
let s:text = []
call extend(s:text, s:text_1)
call extend(s:text, s:text_2)
call extend(s:text, s:text_3)
call extend(s:text, s:text_4)
call extend(s:text, g:IS)
let s:text = join(s:text, \"\")
echo s:text
endfunction

\" 多选模式排序.
\" @return 没有返回
\"
\" 在多选模式下重复调用, 只有到最后一次才会真正执行. 读所有选中的行, 排序, 删掉选中的行, 把排序结果追加到目标位置.
function! GlobalSort()
let l:first_line_number = line(\"'<\")
let l:last_line_number = line(\"'>\")
let l:current_line_number = line(\".\")
if l:current_line_number != l:last_line_number
return \"\"
endif
if l:first_line_number == l:last_line_number
return \"\"
endif
let l:lines = []
for l:current_line_number in range(l:first_line_number, l:last_line_number)
call add(l:lines, getline(l:current_line_number))
endfor
let l:lines = sort(l:lines)
call cursor(l:first_line_number, 1)
for x in range(len(l:lines))
call execute(\"normal dd\")
endfor
call append(l:first_line_number - 1, l:lines)
call cursor(l:first_line_number, 1)
return \"\"
endfunction

\" 全局复制
\" @param $1 文件名, 不带'/'就行
\" @return 没有返回
\"
\" 读行, 写到指定文件.
function! GlobalCopy(path)
echo a:path
let l:line = getline(\".\")
call writefile([l:line], printf(\"/tmp/%s_%s\", $USER, a:path))
return \"\"
endfunction

\" 全局复制
\" @param $1 文件名, 不带'/'就行
\" @return 没有返回
\"
\" 读行, 写到指定文件. 因为是在多选模式下调用, 所以要对第一行特别对待. 
\" 如果是第一行则用覆盖模式写入, 否则用追加模式写入.
function! GlobalVCopy(path)
let l:line = getline(\".\")
if line(\".\") == line(\"'<\")
call writefile([l:line], printf(\"/tmp/%s_%s\", $USER, a:path))
else
call writefile([l:line], printf(\"/tmp/%s_%s\", $USER, a:path), \"a\")
endif
if line(\".\") == line(\"'>\")
call cursor(line(\"'<\"), 1)
normal ^
endif
return \"\"
endfunction


autocmd BufReadPost * 
\\if 1 < line(\"'\\\"\") 
\\       && line(\"'\\\"\") <= line(\"$\") 
\\       && &ft !~# 'commit' |
\\   execute \"normal! g`\\\"\" |
\\endif

\" 全局粘贴
\" @param $1 文件名, 不带'/'就行
\" @return 没有返回
\"
\" 读文件, 写到当前行.
function! GlobalPaste(path)
let l:content = readfile(printf(\"/tmp/%s_%s\", $USER, a:path))
call append(line(\".\"), l:content)
return \"\"
endfunction


function! RunOrBrowse()
write
normal mg
call GlobalFindArguments(\"//\")
call GlobalFindFlags(\"//\")
if searchpos('package *\\(\\S\\+\\);')[0] == 0
let l:package = \"\"
else
let l:package = matchlist(
\\getline(\".\"),
\\'package *\\(\\S\\+\\);')[1]
let l:package = l:package.\".\"
endif
let l:command = '
\\!javac %s %s -Xlint:unchecked -d /tmp/$USER
\\&& cd /tmp/$USER
\\&& %s
\\&& java %s%s %s
\\'
let l:command =
\\printf(
\\l:command,
\\expand(\"%:p\"),
\\b:flags,
\\GlobalGreenPrintCommand(\"java\"),
\\l:package,
\\expand(\"%:t:r\"),
\\b:arguments)
execute l:command
normal `gzz
return \"\"
endfunction

    let l:class = expand (\"%:t:r\")

    normal mg  

";

#[test]
fn test_vim() {
    use rust_format::Formatter;
    println!(
        "原文:\n{}\n\n==================================================",
        TEXT
    );
    println!(
        "格式化之后:\n{}\n\n==================================================",
        rust_format::vim::VimFormatter::format(TEXT)
    );
    return;
}
