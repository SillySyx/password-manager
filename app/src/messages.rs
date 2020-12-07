use crate::views::Views;

#[derive(Debug, Clone)]
pub enum Messages {
    ChangeView { view: Views },
    UnlockApp { key: String },
    AddPasswordMessage { name: String, description: String, category: String, password: String },
    EditPassword { name: String },
    UpdatePassword { entry: String, name: String, description: String, category: String, password: String },
    CopyPassword { name: String },
    RemovePassword { name: String },
    GeneratePassphraseForAddView,
    AddViewInputKeyChanged { input: &'static str, value: String },
    EditViewInputKeyChanged { input: &'static str, value: String },
    UnlockViewInputKeyChanged { value: String },
}