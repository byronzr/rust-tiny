mod update;
mod view;

#[derive(Debug, Clone)]
pub enum SubMessage {
    None,
    Test,
}

impl super::PageAction<SubMessage> for SubMessage {
    fn update(&self, idx: &mut usize, state: &mut super::State) -> iced::Command<SubMessage> {
        // 如果编译过程中发现 update::proccess 同样会有数据涉及生命周期的问题，可能还需要加上生命周期的注解
        // 本例中没有进行任何操作
        return update::proccess(idx, state);
    }

    fn view<'a>(&self, state: &'a super::State) -> iced::Element<'a, SubMessage> {
        // 将 state 传递给 view::show 函数
        // view::show 的返回，会涉及到生命周期的，所以需要作生命周期的注解
        // 而生命周期的注解涉及到 PageAction Trait,还有 proc_macro 的实现
        view::show(state)
    }
}
