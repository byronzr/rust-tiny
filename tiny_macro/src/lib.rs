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
            pub fn subview(&self,state: &State) -> iced::Element<Message> {
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
