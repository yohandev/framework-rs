use framework::prelude::*;

fn main()
{
    framework::run::<MyGuiSketch>();
}

struct MyGuiSketch(f32);

impl Sketch for MyGuiSketch
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("GUI!", v![600, 400]);

        MyGuiSketch(0.0)
    }

    fn gui(&mut self, gui: &mut Gui)
    {
        gui
            .window("Welcome to eGUI!")
            .resizable(true)
            .build(|ui|
            {
                ui.label("Hi!");
                if ui.button("click me!").clicked()
                {
                    ui.label("why did you click me... ;_;");
                }
                ui
                    .slider(&mut self.0, -100.0..=100.0)
                    .text("distance")
                    .suffix("m")
                    .build();
            });
    }
}