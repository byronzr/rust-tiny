use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

// 定义派生宏
// 主要目的是简化 update 方法和 view 方法的实现
// 原理是通过宏遍历 enum 所有成员，并生成 match 语句,如果人工实现，需要写很多重复代码
#[proc_macro_derive(TinyPageMacro)]
pub fn tiny_page_update_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("TinyPageMacro only works with enums");
    };

    // 通过 collect 方法，将所有的变体名称收集到一个 Vec 中
    // 在 quote! 就可以重复使用，Iterator 也可以直接使用,但会在第二次使用时耗尽
    let variant_names = variants.iter().map(|v| &v.ident).collect::<Vec<_>>();

    let expanded = quote! {
        impl #name {
            // update 方法
            pub fn update(&self,idx: &mut usize, state: &mut State) -> iced::Command<Message> {
                match self {
                    #(
                        #name::#variant_names(sub_message)=>sub_message.update(idx,state).map(#name::#variant_names),
                    )*
                }
                .map(Message::Action)
            }
            // subview 方法
            pub fn subview<'a>(&self,state: &'a State) -> iced::Element<'a,Message> {
                match self {
                    #(
                        #name::#variant_names(sub_message)=>sub_message.view(state).map(#name::#variant_names),
                    )*
                }
                .map(Message::Action)
            }
        }
    };

    TokenStream::from(expanded)
}
