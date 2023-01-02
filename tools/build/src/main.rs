use std::fs;
use makepad_rust_tokenizer::{Cursor, State, FullToken, Delim, LiveId, live_id, id};
use std::{
    ops::Deref,
    ops::DerefMut,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TokenWithString {
    pub token: FullToken,
    pub value: String
}

impl Deref for TokenWithString {
    type Target = FullToken;
    fn deref(&self) -> &Self::Target {&self.token}
}

impl DerefMut for TokenWithString {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.token}
}

pub trait TokenSliceApi {
    fn find_tokens_index(&self, tokens: &[TokenWithString]) -> Option<usize>;
    fn find_str_index(&self, what: &str) -> Option<usize>;
    fn after(&self, what: &str) -> Option<&[TokenWithString]>;
    fn at(&self, what: &str) -> Option<&[TokenWithString]>;
    fn find_close(&self, delim: Delim) -> Option<&[TokenWithString]>;
    fn find_token(&self, token: FullToken) -> Option<&[TokenWithString]>;
    fn parse_use(&self) -> Vec<Vec<LiveId >>;
    fn to_string(&self) -> String;
}

impl<T> TokenSliceApi for T where T: AsRef<[TokenWithString]> {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for token in self.as_ref(){
            out.push_str(&token.value);
        }
        out
    }
    
    fn find_tokens_index(&self, what: &[TokenWithString]) -> Option<usize> {
        let source = self.as_ref();
        for i in 0..source.len() {
            for j in 0..what.len() {
                if source[i+j].token != what[j].token {
                    break;
                }
                if j == what.len() - 1 {
                    return Some(i)
                }
            }
        }
        None
    }
    
    fn find_str_index(&self, what: &str) -> Option<usize> {
        self.find_tokens_index(&parse_to_tokens(what))
    }
    
    fn after(&self, what: &str) -> Option<&[TokenWithString]> {
        let source = self.as_ref();
        let what = &parse_to_tokens(what);
        if let Some(pos) = source.find_tokens_index(what) {
            return Some(&source[pos + what.len()..])
        }
        None
    }
    
    fn at(&self, what: &str) -> Option<&[TokenWithString]> {
        let source = self.as_ref();
        let what = &parse_to_tokens(what);
        if let Some(pos) = source.find_tokens_index(what) {
            return Some(&source[pos..])
        }
        None
    }
    
    fn find_close(&self, delim: Delim) -> Option<&[TokenWithString]> {
        let source = self.as_ref();
        let mut depth = 0;
        for i in 0..source.len() {
            if source[i].is_open_delim(delim) {
                depth += 1;
            }
            else if source[i].is_close_delim(delim) {
                if depth == 0 { // unexpected end
                    panic!()
                }
                depth -= 1;
                if depth == 0 {
                    return Some(&source[0..i + 1])
                }
            }
        }
        None
    }
    
    fn find_token(&self, token: FullToken) -> Option<&[TokenWithString]> {
        let source = self.as_ref();
        let mut depth = 0;
        for i in 0..source.len() {
            if source[i].is_open() {
                depth += 1;
            }
            else if source[i].is_close() {
                if depth == 0 { // unexpected end
                    panic!()
                }
                depth -= 1;
            }
            else if depth == 0 && source[i].token == token {
                return Some(&source[0..i + 1])
            }
        }
        None
    }
    
    fn parse_use(&self) -> Vec<Vec<LiveId >> {
        // fetch use { }
        let after_use = self.after("use").unwrap();
        let source = after_use.find_close(Delim::Brace).unwrap();
        
        // now we have to flatten the use tree
        let mut stack = Vec::new();
        let mut ident = Vec::new();
        let mut deps = Vec::new();
        for i in 0..source.len() {
            match source[i].token {
                FullToken::Ident(id) => {
                    ident.push(id);
                }
                FullToken::Punct(live_id!(::)) => {}
                FullToken::Punct(live_id!(,)) => {
                    let len = *stack.last().unwrap();
                    if ident.len()>len {
                        deps.push(ident.clone());
                    }
                    ident.truncate(len);
                }
                FullToken::Open(Delim::Brace) => {
                    stack.push(ident.len());
                }
                FullToken::Close(Delim::Brace) => {
                    let len = stack.pop().unwrap();
                    if ident.len()>len {
                        deps.push(ident.clone());
                        ident.truncate(*stack.last().unwrap());
                    }
                }
                _ => {
                    // unexpected
                }
            }
        }
        // we should parse all our use things into a fully qualified list.
        deps
    }
    
}

fn parse_to_tokens(source: &str) -> Vec<TokenWithString> {
    let mut tokens = Vec::new();
    let mut total_chars = Vec::new();
    let mut state = State::default();
    let mut scratch = String::new();
    let mut last_token_start = 0;
    for line_str in source.lines() {
        let start = total_chars.len();
        total_chars.extend(line_str.chars());
        let mut cursor = Cursor::new(&total_chars[start..], &mut scratch);
        loop {
            let (next_state, full_token) = state.next(&mut cursor);
            if let Some(full_token) = full_token {
                let next_token_start = last_token_start + full_token.len;
                let value:String = total_chars[last_token_start..next_token_start].into_iter().collect();
                if !full_token.is_ws_or_comment() {
                    tokens.push(TokenWithString{
                        token: full_token.token,
                        value
                    });
                }
                else{
                    if let Some(last) = tokens.last_mut(){
                        last.value.push_str(&value);
                    }
                }
                last_token_start = next_token_start;
            }
            else {
                break;
            }
            state = next_state;
        }
        if let Some(last) = tokens.last_mut(){
            last.value.push_str("\n");
        }
    }
    tokens
}

fn parse_file(file: &str) -> Result<Vec<TokenWithString>, Box<dyn std::error::Error >> {
    let source = fs::read_to_string(file) ?;
    let source = parse_to_tokens(&source);
    Ok(source)
}

fn filter_symbols(inp: Vec<Vec<LiveId >>, filter: &[LiveId]) -> Vec<Vec<LiveId >> {
    let mut out = Vec::new();
    'outer: for sym in inp {
        if sym.len() >= filter.len() {
            for i in 0..filter.len() {
                if sym[i] != filter[i] {
                    continue 'outer;
                }
            }
            out.push(sym[filter.len()..sym.len()].to_vec());
        }
    }
    out
}

enum Node{
    Sub(Vec<(LiveId,Node)>),
    Value(String)
}
    
fn generate_win32_outputs_from_file(file:&str, output:&mut Node){
    
    let source = parse_file(file).unwrap();
    let symbols = source.parse_use();
    let symbols = filter_symbols(symbols, id!(crate.windows_crate.Win32));

    fn push_unique(output:&mut Node, what:&[LiveId], value:String){
        if what.len() == 1{
            // terminator node
            if let Node::Sub(vec) = output{
                if  vec.iter_mut().find(|v| v.0 == what[0]).is_none(){
                    vec.push((what[0], Node::Value(value)));
                }
            }
            else{
                panic!();
            }
        }
        else{
            if let Node::Sub(vec) = output{
                if let Some(child) = vec.iter_mut().find(|v| v.0 == what[0]){
                    return push_unique(&mut child.1, &what[1..], value);
                }
                let mut child = Node::Sub(Vec::new());
                push_unique(&mut child, &what[1..], value);
                vec.push((what[0], child));
            }
            else{
                panic!();
            }
        }
    }
    
    fn include_struct_dep(input:&[TokenWithString], output:&mut Node, ty:LiveId, sym_base:&[LiveId]){
        if let Some(is_struct) = input.at(&format!("pub struct {}", ty)){
            let mut out = String::new();
            let is_struct = if let FullToken::Open(Delim::Paren) = is_struct[3].token{
                out.push_str("#[repr(transparent)]\n");
                //out.push_str("#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]\n");
                is_struct.find_token(FullToken::Punct(live_id!(;))).unwrap()
            }
            else{
                out.push_str("#[repr(C)]\n");
                is_struct.find_close(Delim::Brace).unwrap()
            };
            out.push_str(&is_struct.to_string());
            
            // lets add our impl struct{}
            fn add_impl(out:&mut String,input:&[TokenWithString], at:String,  )->bool{
                if let Some(is_impl) = input.at(&at){
                    let is_impl = is_impl.find_close(Delim::Brace).unwrap();
                    out.push_str(&is_impl.to_string());
                    true
                }
                else{
                    false
                }
            }
            add_impl(&mut out, input, format!("impl {}", ty));
            add_impl(&mut out, input, format!("impl ::core::marker::Copy for {}", ty));
            
            add_impl(&mut out, input, format!("impl ::core::cmp::Eq for {}", ty));
            if !add_impl(&mut out, input, format!("impl ::core::cmp::PartialEq for {}", ty)){
                if let FullToken::Open(Delim::Paren) = is_struct[3].token{
                    out.insert_str(0, "#[derive(PartialEq, Eq)]")
                }
            }
            
            add_impl(&mut out, input, format!("impl ::core::clone::Clone for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::default::Default for {}", ty));
            add_impl(&mut out, input, format!("unsafe impl ::windows::core::Abi for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::fmt::Debug for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::ops::BitOr for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::ops::BitAnd for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::ops::BitOrAssign for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::ops::BitAndAssign for {}", ty));
            add_impl(&mut out, input, format!("impl ::core::ops::Not for {}", ty));
            add_impl(&mut out, input, format!("impl::core::convert::From<::core::option::Option<{}>> for {}", ty, ty));
            add_impl(&mut out, input, format!("unsafe impl ::core::marker::Send for {}", ty));
            add_impl(&mut out, input, format!("unsafe impl ::core::marker::Sync for {}", ty));
            add_impl(&mut out, input, format!("unsafe impl ::windows::core::Vtable for {}", ty));
            add_impl(&mut out, input, format!("unsafe impl ::windows::core::Interface for {}", ty));
            
            
            let mut sym = sym_base.to_vec();
            sym.push(ty);
            push_unique(output, &sym, out);
            
        }
    }
    
    for sym in symbols {
        // allright lets open the module
        let mut path = format!("./platform/bind/windows/generate/src/Windows/Win32");
        // ok so everything is going to go into the module Win32
        // but how do we sort the substructure
        for i in 0..sym.len() - 1 { 
            path.push_str(&format!("/{}", sym[i]));
        }
        let mod_tokens = parse_file(&format!("{}/mod.rs", path)).expect(&format!("{}", path));

        let sym_id = sym[sym.len()-1];

        if let Some(is_fn) = mod_tokens.at(&format!("pub unsafe fn {}", sym_id)){
            let is_fun = is_fn.find_close(Delim::Brace).unwrap();
            //  ok so how do we do this
            push_unique(output, &sym, is_fun.to_string());
        }
        else if let Some(is_const) = mod_tokens.at(&format!("pub const {}", sym_id)){
            let is_const = is_const.find_token(FullToken::Punct(live_id!(;))).unwrap();
            push_unique(output, &sym, is_const.to_string());
            // lets also fetch the type
            if let FullToken::Ident(ident) = is_const[4].token{
                include_struct_dep(&mod_tokens, output, ident, &sym[0..sym.len()-1]);
            }
        }
        else if let Some(is_type) = mod_tokens.at(&format!("pub type {}", sym_id)){
            let is_type = is_type.find_token(FullToken::Punct(live_id!(;))).unwrap();
            push_unique(output, &sym, is_type.to_string());
        }
        else if let Some(is_union) = mod_tokens.at(&format!("pub union {}", sym_id)){
            let is_union = is_union.find_close(Delim::Brace).unwrap();
            push_unique(output, &sym, format!("#[repr(C)]#[derive(Clone, Copy)]\n{}",is_union.to_string()));
        }
        else{
            include_struct_dep(&mod_tokens, output, sym_id, &sym[0..sym.len()-1])
        }
        
        if let Some(is_com) = mod_tokens.at(&format!("pub struct {}_Vtbl", sym_id)){
            let mut sym = sym.clone();
            let sym_end = sym.len() -1;

            let is_com = is_com.find_close(Delim::Brace).unwrap();
            sym[sym_end] = LiveId::from_str(&format!("{}_Vtbl",sym_id)).unwrap();
            push_unique(output, &sym, format!("#[repr(C)]\n{}",is_com.to_string()));
            
            let impl_tokens = parse_file(&format!("{}/impl.rs", path)).unwrap();
            
            let is_trait = impl_tokens.at(&format!("pub trait {}_Impl", sym_id)).unwrap();
            let is_trait = is_trait.find_close(Delim::Brace).unwrap();
            sym[sym_end] = LiveId::from_str(&format!("{}_Impl",sym_id)).unwrap();
            push_unique(output, &sym, is_trait.to_string());
            
            let is_impl = impl_tokens.at(&format!("impl {}_Vtbl", sym_id)).unwrap();
            let is_impl = is_impl.find_close(Delim::Brace).unwrap();
            sym[sym_end] = LiveId::from_str(&format!("{}_Vtbl2",sym_id)).unwrap();
            push_unique(output, &sym, is_impl.to_string());
            
            
            if let Some(is_hier) = mod_tokens.at(&format!("::windows::core::interface_hierarchy!({}", sym_id)){
                let is_hier = is_hier.find_token(FullToken::Punct(live_id!(;))).unwrap();
                sym[sym_end] = LiveId::from_str(&format!("{}_hierarchy",sym_id)).unwrap();
                push_unique(output, &sym, is_hier.to_string());
            }
        }
    }
}

fn main() {
    let mut output = Node::Sub(Vec::new());
    generate_win32_outputs_from_file("./platform/src/os/mswindows/win32_app.rs",&mut output);
    generate_win32_outputs_from_file("./platform/src/os/mswindows/win32_window.rs",&mut output);
    generate_win32_outputs_from_file("./platform/src/os/mswindows/win32_deps.rs",&mut output);
    generate_win32_outputs_from_file("./platform/src/os/mswindows/d3d11.rs",&mut output);
    
    fn generate_string_from_outputs(node:&Node, output:&mut String){
        match node{
            Node::Sub(vec)=>{
                for (sub,node) in vec{
                    if let Node::Value(v) = node{
                        output.push_str(&v);
                        output.push_str("\n");
                    }
                    else{
                        output.push_str(&format!("pub mod {}{{\n", sub));
                        generate_string_from_outputs(node, output);
                        output.push_str("}\n");
                    }
                }
            }
            _=>panic!()
        }
    }
    
    // ok lets recursively walk the tree now
    let mut gen = String::new();
    gen.push_str("#![allow(non_camel_case_types)]\npub mod Win32{\n");
    generate_string_from_outputs(&output, &mut gen);
    gen.push_str("\n}\n");
    // lets write the output file
    fs::write("./platform/bind/windows/src/Windows/mod.rs", gen).unwrap();    
}
