fn main() {
    use rand::seq::SliceRandom;
    use slint::ComponentHandle;
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    //fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    //duplicate them to ensure that we have pairs
    tiles.extend(tiles.clone());

    //randomly mix the tiles
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run().unwrap();
}

slint::slint! {

    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
    }

  component MemoryTile inherits Rectangle {
    callback clicked;
    in property <bool> open_curtain;
    in property <bool> solved;
    in property <image> icon;


      border-radius: 8px;
      width: 64px;
      height: 64px;
      background: solved ? #34ce57: #3960D5;
      animate background {
           duration: 800ms;
      }

      Image {
        source: icon;
        width: parent.width;
        height: parent.height;
      }

      Rectangle {
        background: #193076;
        x: 0px;
        width: open_curtain? 0px : (parent.width / 2);
        height: parent.height;
        animate width {duration: 200ms;easing: ease-in;}
      }

      Rectangle {
        background: #193076;
        x: open_curtain ? parent.width : (parent.width / 2);
        width: open_curtain? 0px : (parent.width / 2);
        height: parent.height;
        animate width {duration: 200ms;easing: ease-in;}
        animate x {duration: 200ms;easing: ease-in; }
      }

      TouchArea {
        clicked => {
            //delegate to the user of this element
            root.clicked();
        }
      }
  }

  export component MainWindow inherits Window {
      width: 326px;
      height: 326px;
      title: "Memory Game by José Augusto";


      callback check_if_pair_solved();
      in property <bool> disable_tiles;



    in-out property <[TileData]> memory_tiles:[
        {image: @image-url("icons/at.png")},
        {image: @image-url("icons/balance-scale.png")},
        {image: @image-url("icons/bicycle.png")},
        {image: @image-url("icons/bus.png")},
        {image: @image-url("icons/cloud.png")},
        {image: @image-url("icons/cogs.png")},
        {image: @image-url("icons/motorcycle.png")},
        {image: @image-url("icons/video.png")},
    ];

    for title[i] in memory_tiles: MemoryTile{
        x: mod(i, 4) * 74px;
        y: floor(i / 4) * 74px;
        width: 64px;
        height: 64px;
        icon: title.image;
        open_curtain: title.image_visible || title.solved;
        solved: title.solved;
        clicked => {
           if (!root.disable_tiles){
            title.image_visible = !title.image-visible;
            root.check_if_pair_solved();
           }
        }
    }
  }

}
