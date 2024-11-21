from manim import *

class RustMacroAnimation(Scene):
    def construct(self):
        # Introduction
        title = Text("Rust Macro Example").scale(1.5)
        description = Text("Understanding the create_function macro", font_size=36)
        self.play(Write(title))
        self.wait(1)
        self.play(Transform(title, description))
        self.wait(2)
        self.play(FadeOut(title))

        # Macro Definition
        macro_code = """
macro_rules! create_function {
    ($func_name:ident, $body:expr) => {
        fn $func_name() {
            println!("Executing function {}", stringify!($func_name));
            $body;
        }
    };
}
"""
        macro_text = Code(code=macro_code, language="rust", font="Monospace", font_size=24)
        self.play(Write(macro_text))
        self.wait(2)

        # Macro Invocation
        invocation_code = """
create_function!(example_func, { 
    println!("This is the function body");
});
"""
        invocation_text = Code(code=invocation_code, language="rust", font="Monospace", font_size=24)
        self.play(Transform(macro_text, invocation_text))
        self.wait(2)

        # Generated Function
        generated_code = """
fn example_func() {
    println!("Executing function {}", stringify!(example_func));
    println!("This is the function body");
}
"""
        generated_text = Code(code=generated_code, language="rust", font="Monospace", font_size=24)
        self.play(Transform(macro_text, generated_text))
        self.wait(2)

        # Main Function
        main_code = """
fn main() {
    example_func();
}
"""
        main_text = Code(code=main_code, language="rust", font="Monospace", font_size=24)
        self.play(Transform(macro_text, main_text))
        self.wait(2)

        # Execution Output
        output_text = Text("Output:\nExecuting function example_func\nThis is the function body", font_size=24)
        self.play(Transform(macro_text, output_text))
        self.wait(2)

        # End
        end_text = Text("End of Animation", font_size=36)
        self.play(Transform(macro_text, end_text))
        self.wait(2)
        self.play(FadeOut(macro_text))
