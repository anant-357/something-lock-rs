fn main() {
    glib_build_tools::compile_resources(&["src/ui"], "src/ui/ui.gresource.xml", "ui.gresource");
}
