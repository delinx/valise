// Data types

#![allow(dead_code)]
// Structures
pub struct CheckboxItem {
    pub checked: bool,
    pub content: String,
}

pub struct CheckboxList {
    pub items: Vec<CheckboxItem>,
}

pub struct Tag {
    pub title: String,
    pub color: Option<u64>,
}

pub struct Card {
    pub content: String,
    pub due_date: Option<u64>,
    pub checkbox_list: Option<CheckboxList>,
    pub tags: Vec<Tag>,
}

pub struct CardList {
    pub title: String,
    pub cards: Vec<Card>,
}

impl CheckboxItem {
    pub fn new(checked: bool, content: &str) -> Self {
        CheckboxItem {
            checked,
            content: content.to_owned(),
        }
    }
}

// Implementation
impl CheckboxList {
    pub fn new() -> Self {
        CheckboxList { items: Vec::new() }
    }

    pub fn get_total(&self) -> usize {
        return self.items.len();
    }

    pub fn get_checked(&self) -> usize {
        return self.items.iter().filter(|item| item.checked).count();
    }
}

impl Tag {
    pub fn new(title: &str) -> Self {
        Tag {
            title: title.to_owned(),
            color: None,
        }
    }

    pub fn with_color(mut self, color: u64) -> Self {
        self.color = Some(color);
        self
    }
}

impl Card {
    pub fn new(content: &str) -> Self {
        Card {
            content: content.to_owned(),
            due_date: None,
            checkbox_list: None,
            tags: Vec::new(),
        }
    }

    pub fn with_due_date(mut self, date: u64) -> Self {
        self.due_date = Some(date);
        self
    }

    pub fn with_checkbox_list(mut self, checkbox_list: CheckboxList) -> Self {
        self.checkbox_list = Some(checkbox_list);
        self
    }
}

// ToString trait
impl ToString for CheckboxItem {
    fn to_string(&self) -> String {
        format!("(checked: '{}', content: '{}')", self.checked, self.content)
    }
}

impl ToString for CheckboxList {
    fn to_string(&self) -> String {
        let item_strings: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
        format!("[{}]", item_strings.join(", "))
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        format!(
            "(color:'{}', title:'{}')",
            match self.color {
                Some(cl) => format!("0x{:X}", cl),
                None => "None".to_string(),
            },
            self.title
        )
    }
}

impl ToString for Card {
    fn to_string(&self) -> String {
        format!(
            "(tags:\"{}\", due_date:\"{}\", content:\"{}\", checkbox_list:\"{}\")",
            self.tags
                .iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            match self.due_date {
                Some(cl) => format!("{}", cl),
                None => "None".to_string(),
            },
            self.content,
            self.checkbox_list
                .iter()
                .map(|cbitem| cbitem.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
