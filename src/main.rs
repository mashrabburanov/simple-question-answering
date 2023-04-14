use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

//#[derive(Default)]
struct MyEguiApp {
    pub qa_model: QuestionAnsweringModel,
    pub context: String,
    pub question: String,
    pub model_answer: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        MyEguiApp {
            qa_model: QuestionAnsweringModel::new(Default::default()).unwrap(),
            context: String::from(""),
            question: String::from(""),
            model_answer: String::from(""),
        }
        
        //Self::default()
    }

    fn run_model(&mut self) {
        let context = self.context.clone();
        let question = self.question.clone();
        let answers = self.qa_model.predict(&[QaInput { question, context }], 1, 32);

        self.model_answer = answers[0][0].answer.clone();
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Question answering");
           ui.label("context:");
           ui.text_edit_multiline(&mut self.context);
           ui.label("question:");
           ui.text_edit_multiline(&mut self.question);
           ui.label("answer: ");
           ui.label(self.model_answer.clone());
           if ui.button("Run").clicked() {
                self.run_model();
           }
       });
   }
}