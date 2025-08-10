fn main() {
    println!("cargo:rustc-link-search=native=C:/Users/admar/Desktop/multiplayer-fps/game/target/debug");
    println!("cargo:rustc-link-lib=dylib=SDL2");
}
// This build script links the SDL2 library dynamically