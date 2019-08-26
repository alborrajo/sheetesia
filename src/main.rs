use opencv::core::*;
use opencv::imgcodecs::*;
use opencv::videoio::*;

use opencv::highgui::*;

mod piano;
use crate::piano::piano::*;

fn main() {
	// Load files
	let template: Mat = imread("res/template.png", IMREAD_COLOR).unwrap();
	let mut video: VideoCapture = VideoCapture::new_from_file_with_backend("in/megalovania.mp4", CAP_ANY).unwrap();
	
	if !video.is_opened().unwrap() || template.empty().unwrap() {
		println!(":(");
		std::process::exit(1);
	}
	println!("Loaded video: {}","in/megalovania.mp4");

	// Make window
	named_window("Gabo", WINDOW_AUTOSIZE).unwrap();
	
	// ----------------
	// READ FIRST FRAME
	let mut frame: Mat = Mat::default().unwrap();
	video.read(&mut frame).unwrap();
	
	// Find piano
	println!("Finding piano...");
	let mut piano: Piano = Piano::new(&frame, &template);	

	// Draw detected piano
	let frame_draw_notes: Mat = piano.draw_notes(&frame);
	imshow("Gabo", &frame_draw_notes).unwrap();
	wait_key(0).unwrap();
	
	// ----------
	// PLAY VIDEO
	println!("Detecting notes!");

	// Vector storing each note and the color it had on the previous frame
	//	Initialize with each note's default color
	let mut previous_frame_note_colors: Vec<Vec<Vec3b>> = Vec::new();
	for octave in &piano.octaves {
		let mut note_colors_vec: Vec<Vec3b> = Vec::new();
		for note in &octave.notes {
			note_colors_vec.push(note.default_color);
		}
		previous_frame_note_colors.push(note_colors_vec);
	}

	// LOOP UNTIL THE VIDEO ENDS
	let between_frames_thresh: i32 = 150;
	let color_thresh: i32 = 200;
	loop {
		if !video.read(&mut frame).unwrap() {
			println!(":(");
			break;
		}
		
		// For each note in every octave
		let mut octave_index = 0;
		for octave in &mut piano.octaves {
			let mut note_index = 0;
			for mut note in &mut octave.notes {
				let note_color: Vec3b = *frame.at_2d(note.location.y, note.location.x).unwrap();
				
				// Skip check if the color is close to the one in the previous frame
				let mut diff_with_previous_frame: i32 = 0;
				for i in 0..3 {
					diff_with_previous_frame += (note_color[i] as i32 - previous_frame_note_colors[octave_index][note_index][i] as i32).abs();
				}

				// If the color isn't close to the one in the previous frame
				//	Check if the color is close to the default color (Key released) or not (key pressed)
				if diff_with_previous_frame > between_frames_thresh {	
					let mut diff_with_default_color: i32 = 0;
					for i in 0..3 {
						diff_with_default_color += (note_color[i] as i32 - note.default_color[i] as i32).abs();
					}

					let result: Result<bool, bool> = note.set_pressed(diff_with_default_color > color_thresh);
					match result {
						Ok(pressed) => {
							if pressed {
								println!("{}\tPRESSED", note.to_string());
							} else {
								println!("{}\tRELEASED", note.to_string());
							}
						},
						Err(_) => {}
					};
				}

				// Update previous frame pixel color
				previous_frame_note_colors[octave_index][note_index] = note_color;
				note_index = note_index+1;
			}
			octave_index = octave_index+1;
		}
		
		imshow("Gabo", &frame).unwrap();
		if wait_key(50).unwrap() == 27 {break;} // Exit if ESC (27) is pressed
	}
	
	// Cleanup
	video.release().unwrap();
	destroy_window("Gabo").unwrap();
}
