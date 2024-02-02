fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
  component MemoryTile inherits Rectangle {
      width: 64px;
      height: 64px;
      background: #3960D5;

      Image {
        source: @image-url("icons/bus.png");
        width: parent.width;
        height: parent.height;
      }
  }

  export component MainWindow inherits Window {
    MemoryTile {}
  }

}
