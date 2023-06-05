mod data;

fn main() {
    println!("- Valise -\n");

    let ci: data::CheckboxItem = data::CheckboxItem::new(true, "Example content");
    let ci2: data::CheckboxItem = data::CheckboxItem::new(false, "content box");

    let mut cl: data::CheckboxList = data::CheckboxList::new();
    cl.items.push(ci);
    cl.items.push(ci2);

    let tg1: data::Tag = data::Tag::new("I am a tag").with_color(0xFF00FF);
    let tg2: data::Tag = data::Tag::new("Delinx");

    let mut card1: data::Card = data::Card::new("Nothing here");
    let mut card2: data::Card = data::Card::new("Nothing here")
        .with_due_date(8409200)
        .with_checkbox_list(cl);
    card1.tags.push(tg1);
    card1.tags.push(tg2);
    card1.content = "Hello world I am content".to_string();
    card2.content = "I am content of card 2".to_string();

    print!("{}\n\n{}\n", card1.to_string(), card2.to_string());
}
