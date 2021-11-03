use std::ops::{Add, Sub};
use tauri::{PhysicalPosition, PhysicalSize, Pixel, Position, Size, Window, Wry};

fn emit_rust_event(position: PhysicalPosition<f64>, size: PhysicalSize<f64>, window: Window<Wry>) {
  let window_x: i32 = position.x.cast();
  let window_y: i32 = position.y.sub(size.height).cast();
  let monitors = window.available_monitors().expect("empty monitor");
  for monitor in monitors {
    let top_left: PhysicalPosition<i32> = *monitor.position();
    let mut top_right: PhysicalPosition<i32> = top_left;
    top_right.x = top_left.x.add(monitor.size().width.cast::<i32>());
    if top_left.y.eq(&window_y) && top_right.x.ge(&window_x) && top_left.x.le(&window_x) {
      let factor = monitor.scale_factor();
      let logical_position = monitor.position().to_logical::<f64>(factor);
      let logical_size = monitor.size().to_logical::<f64>(factor);
      window
        .set_position(Position::Logical(logical_position))
        .unwrap();
      window.set_size(Size::Logical(logical_size)).unwrap();
      println!("托盘位置：{:?}", position.to_logical::<f64>(factor));
      println!("显示器位置：{:?}", logical_position);
      window
        .emit(
          "rust-event",
          Payload {
            message: "FullEvent".into(),
            size: size.to_logical(factor),
            position: position.to_logical(factor),
            screen_position: logical_position,
          },
        )
        .expect("failed to emit");
      window.show().unwrap();
      window.set_focus().unwrap();
    }
  }
}
