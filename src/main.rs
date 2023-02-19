use druid::{
    widget::{Button, Container, Flex, Label, LensWrap, List, Split, TextBox},
    AppLauncher, Color, Data, Lens, Widget, WindowDesc,
};
use im::{vector, Vector};

#[derive(Clone, Data, Lens)]
struct TodoList {
    items: Vector<String>,
    next_item: String,
}

// create the widget tree
fn build_ui() -> impl Widget<TodoList> {
    Split::columns(
        Container::new(
            // Dynamic list of Widgets
            LensWrap::new(
                // actual list is processed here
                List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
                TodoList::items,
            ),
        )
        .border(Color::grey(0.6), 2.0),
        Container::new(
            Flex::column()
                .with_flex_child(
                    // new item button
                    Button::new("Add item").on_click(|_, data: &mut TodoList, _| {
                        if !data.next_item.is_empty() {
                            data.items.push_back(data.next_item.clone());
                            data.next_item = String::new();
                        }
                    }),
                    1.0,
                )
                .with_flex_child(
                    // text zone for new items
                    LensWrap::new(TextBox::new(), TodoList::next_item),
                    1.0,
                ),
        )
        .border(Color::grey(0.6), 2.0),
    )
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("First Druid app");

    let initial_data = TodoList {
        items: vector![
            "first item".into(),
            "second item".into(),
            "third item".into(),
            "fourth item".into(),
            "foo".into(),
            "bar".into(),
        ],
        next_item: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
