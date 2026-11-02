fn main() {
    slint_build::compile("ui/app-window.slint")
        .expect("Não foi possível compilar a interface Slint");
}
