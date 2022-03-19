use playdate::*;

/// A testing function to dump new functionality into for manual verification.
pub async fn _run(mut api: playdate::Api) -> ! {
  let system = &api.system;
  let graphics = &mut api.graphics;

  let grey50: LCDPattern = [
    // Bitmap
    0b10101010, 0b01010101, 0b10101010, 0b01010101, 0b10101010, 0b01010101, 0b10101010, 0b01010101,
    // Mask
    0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
  ];
  graphics.clear(&grey50);

  let bmp = graphics.new_bitmap(100, 40, LCDSolidColor::kColorWhite);
  graphics.draw_bitmap(&bmp, 5, 9, LCDBitmapFlip::kBitmapUnflipped);
  drop(bmp);

  graphics.draw_text("Bloop", PDStringEncoding::kASCIIEncoding, 30, 20);

  let mut copy = graphics.copy_frame_buffer_bitmap();

  for y in 20..30 {
    for x in 10..20 {
      copy.pixels_mut().set(x, y, false);
    }
  }
  graphics.draw_bitmap(&copy, 0, 30, LCDBitmapFlip::kBitmapUnflipped);

  // working image
  let yo_path = "images/yo";
  let load = graphics.load_bitmap(yo_path);
  if let Ok(bitmap) = load {
    graphics.draw_bitmap(&bitmap, 100, 80, LCDBitmapFlip::kBitmapUnflipped);
  }

  // broken image
  let broken_path = "images/wat";
  let load = graphics.load_bitmap(broken_path);
  if let Err(error) = load {
    system.log(error);
  }

  let display = &mut api.display;
  display.set_inverted(true);
  display.set_flipped(true, false);
  display.set_scale(2);

  let list_files_in = |path: &str| match api.file.list_files(path) {
    Ok(files) => {
      api.system.log(format!("{}/ files:", path));
      for fname in files {
        api.system.log(format!("  {:?}", fname))
      }
    }
    Err(e) => api.system.log(format!("ERROR: {}", e)),
  };
  let make_dir = |path: &str| match api.file.make_folder(path) {
    Ok(()) => system.log(format!("mkdir {}", path)),
    Err(e) => system.log(e),
  };
  let rename = |from: &str, to: &str| match api.file.rename(from, to) {
    Ok(()) => {
      system.log(format!("renamed {} to {}", from, to));
      list_files_in("myfolder");
    }
    Err(e) => system.log(e),
  };
  let delete_recursive = |path: &str| match api.file.delete_recursive(path) {
    Ok(()) => system.log(format!("deleted {} recursive", path)),
    Err(e) => system.log(e),
  };
  let stat = |path: &str| match api.file.stat(path) {
    Ok(stats) => system.log(format!("stat {}: {:?}", path, stats)),
    Err(e) => system.log(e),
  };
  let write_file = |path: &str, stuff: &[u8]| match api.file.write_file(path, stuff) {
    Ok(()) => system.log(format!("wrote {}", path)),
    Err(e) => system.log(e),
  };
  let read_file = |path: &str| match api.file.read_file(path) {
    Ok(content) => system.log(format!("read {}: {:?}", path, String::from_utf8(content))),
    Err(e) => system.log(e),
  };

  list_files_in("images");

  make_dir("myfolder");
  make_dir("myfolder/two");
  list_files_in("myfolder");
  list_files_in("myfolder/two");

  rename("myfolder/two", "myfolder/three");
  stat("myfolder/three");

  write_file("myfolder/three", b"bees\n");
  write_file("myfolder/three/bears.txt", b"want honey\n");
  read_file("myfolder/three/bears.txt");
  read_file("myfolder/three/no_bears.txt");

  delete_recursive("myfolder");

  system.log(format!(
    "Entering main loop at time {}",
    api.system.current_time()
  ));
  let fw = system.frame_watcher();
  loop {
    let inputs = fw.next().await;
    for (button, event) in inputs.buttons().all_events() {
      match event {
        playdate::ButtonEvent::Push => {
          api.system.log(format!(
            "{:?} pushed on frame {}",
            button,
            inputs.frame_number()
          ));
        }
        playdate::ButtonEvent::Release => {
          api.system.log(format!(
            "{:?} released on frame {}",
            button,
            inputs.frame_number()
          ));
        }
      }
    }

    api.graphics.draw_fps(400 - 15, 0);
  }
}