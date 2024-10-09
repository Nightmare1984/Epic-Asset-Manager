use std::process::{Command, Stdio};
use gtk4::prelude::*;
use gtk4::{Button, Box, Label};

pub struct PluginCompilerWidget {
    compile_button: Button,
    status_label: Label,
}

impl PluginCompilerWidget {
    pub fn new() -> Self {
        PluginCompilerWidget {
            compile_button: Button::new_with_label("Compile Plugin"),
            status_label: Label::new(None),
        }
    }

    pub fn set_status(&self, status: &str) {
        self.status_label.set_label(status);
    }

    pub fn connect(&self, compile_callback: fn() -> String) {
        self.compile_button.connect_clicked(move |_| {
            let status = compile_callback();
            self.set_status(&status);
        });
    }

    pub fn add_to_box(&self, box_: &mut Box) {
        box_.add(&self.compile_button);
        box_.add(&self.status_label);
    }
}

fn compile_plugin() -> String {
    let output = Command::new("sh")
        .arg("plugin_compiler.sh")
        .output()
        .expect("Failed to execute plugin-compiler.sh");

    if output.status.success() {
        "Plugin compiled successfully".to_string()
    } else {
        format!("Plugin compilation failed with status code: {}", output.status.code().unwrap_or(0))
    }
}

fn main() {
    // Create a new GTK application
    let app = gtk::Application::builder()
        .application_id("com.example.plugin_compiler_widget")
        .build();

    // Connect to the "activate" signal
    app.connect_activate(move |app| {
        // Create a new plugin compiler widget
        let widget = PluginCompilerWidget::new();

        // Set the status label to "Plugin not compiled yet"
        widget.set_status("Plugin not compiled yet");

        // Connect the compile button to the compile_plugin function
        widget.connect(compile_plugin);

        // Create a new box to hold the widget
        let box_ = Box::new(Orientation::Vertical, 5);

        // Add the widget to the box
        widget.add_to_box(&mut box_);

        // Create a new window and add the box to it
        let window = Window::new(WindowType::Toplevel);
        window.set_child(Some(&box_));

        // Show the window
        window.show_all();
    });

    // Run the GTK application
    app.run();
}