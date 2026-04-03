use tgui::{
    children, Align, Application, Color, Column, Element, Insets, Observable, Overflow, Stack,
    Text, ViewModelContext,
};

fn main() -> Result<(), tgui::TguiError> {
    Application::new()
        .window_size(800, 500)
        .with_view_model(Test::new)
        .root_view(Test::view)
        .run()
}

struct Test {
    text: Observable<String>,
}

impl Test {
    fn new(context: &ViewModelContext) -> Self {
        Self {
            text: context.observable("This example keeps things intentionally simple: one window, one card, and a small static widget tree.".to_string())
        }
    }

    fn view(&self) -> Element<Self> {
        Stack::new()
            .fill_size()
            .align(Align::Center)
            .child(children![Column::new()
                .overflow_x(Overflow::Scroll)
                .size(500.0, 300.0)
                .border(4.0, Color::WHITE)
                .border_radius(10.0)
                .gap(20.0)
                .padding(Insets::all(5.0))
                .child(children![
                    Text::new(self.text.binding()).font_size(20.0),
                    Stack::new().size(600.0, 100.0).background(Color::BLACK)
                ])])
            .into()
    }
}
