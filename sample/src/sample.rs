//fltkを利用するために必要
use fltk::{app, button::Button,enums::FrameType, frame::Frame, image::SvgImage, prelude::*, window::Window};

//利用する際は関数名の前のアンダーバーを削除
fn _hello_rust1(){
    let app = app::App::default();
    //Window サイズ (表示位置x,表示位置y,width,height, window name)
    let mut wind = Window::new(100, 100, 400, 300, "Hello Rust");
    //文字が入力される場所。""で初期化されている。
    let mut frame = Frame::new(0, 0, 400, 200, "");
    //ボタンを設置
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    //ボタンがクリックされた時にframeに"Hello Rust!"と表示する
    but.set_callback(move |_| frame.set_label("Hello Rust!")); 
    app.run().unwrap();
}

//利用する際は関数名の前のアンダーバーを削除
fn _hello_rust2(){
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello Rust");
    let _but1 = Button::new(10, 10, 80, 40, "Button 1");
    let _but2 = Button::new(10, 50, 80, 40, "Button 2");
    let _but3 = Button::default()
        .with_pos(90, 10)
        .with_size(80, 40)
        .with_label("Button 3");
    wind.end();
    wind.show();
    app.run().unwrap();
}

fn _image_load() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    //window size
    let mut wind = Window::new(100, 100, 400, 300, "Hello Rust");
    //windの中心に四角い枠を生成(w,h)
    let mut frame = Frame::default().with_size(360, 260).center_of(&wind);
    frame.set_frame(FrameType::EngravedBox);
    //画像を読み込む
    let image_path = "screenshots/RustLogo.svg";
    let mut _image = SvgImage::load(image_path).unwrap();
    //let mut image = SvgImage::load(image_path).unwrap();
    //(scale x,scale y) 小さい方に合わせる
    //image.scale(200, 200, true, true);
    //frame.set_image(Some(image));
    //window サイズを修正できるか
    wind.make_resizable(false);
    wind.end();
    wind.show();
    //包括している何かしらの値を取り出すもの
    app.run().unwrap();
}

fn main() {
    _hello_rust1();
    _hello_rust2();
    //_image_load();
}