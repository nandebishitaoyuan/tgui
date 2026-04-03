use tgui::{Application, Color, Column, Element, Observable, ViewModelContext};

fn main() -> Result<(), tgui::TguiError> {
    Application::new()
        .window_size(800, 500)
        .with_view_model(Test::new)
        .root_view(Test::view)
        .run()
}


struct Test {
    text: Observable<String>
}

impl Test {
    fn new(context: &ViewModelContext) -> Self {
        Self {
            text: context.observable(String::new())
        }
    }

    fn view(&self) -> Element<Self> {
        Column::new()
            .size(500.0, 300.0)
            .border(2.0, Color::rgb(255, 255, 255))
            .border_radius(6.0)
            .into()
    }
}