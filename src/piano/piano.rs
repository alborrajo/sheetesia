use opencv::core::*;
use opencv::imgproc::*;

use crate::piano::octave::*;

pub const NOTE_C 		: usize = 0;
pub const NOTE_CSHARP	: usize = 1;
pub const NOTE_D 		: usize = 2;
pub const NOTE_DSHARP 	: usize = 3;
pub const NOTE_E 		: usize = 4;
pub const NOTE_F 		: usize = 5;
pub const NOTE_FSHARP 	: usize = 6;
pub const NOTE_G 		: usize = 7;
pub const NOTE_GSHARP 	: usize = 8;
pub const NOTE_A 		: usize = 9;
pub const NOTE_ASHARP 	: usize = 10;
pub const NOTE_B 		: usize = 11;


pub struct Piano {
	pub octaves: Vec<Octave>,
}

impl Piano {
	
	pub fn new(image: &Mat, unscaled_template: &Mat) -> Piano {

		// Scale template to fit the full image
		let template: Mat = get_scaled_template(&image, &unscaled_template);

		// Matrix to store the match_tempalte results
		let mut result: Mat = Mat::new_rows_cols_with_default(image.rows().unwrap()-template.rows().unwrap()+1, image.cols().unwrap()-template.cols().unwrap()+1, CV_32FC1, Scalar::all(0.0)).unwrap();
			
		// Do the Matching
		match_template(&image, &template, &mut result, TM_SQDIFF_NORMED, &Mat::default().unwrap()).unwrap();
		
		// Vector to store the found octaves
		let mut octaves: Vec<Octave> = Vec::new();
		
		// Threshold the result so finding octaves is easier
		// Loop incrementing the thresh each time until matches are found
		let mut thresh: f64 = 0.0;
		let thresh_step: f64 = 0.02;
		loop {
			let mut threshold_result = Mat::default().unwrap();
			threshold(&result, &mut threshold_result, thresh, 1.0, THRESH_BINARY).unwrap();
			
			// Loop to store in the octaves vector every found match
			loop {
				// Localizing the best match with minMaxLoc
				let mut min_val: f64 = 0.0;
				let mut max_val: f64 = 0.0;
				let mut min_loc: Point = Point {x: 0, y: 0};
				let mut max_loc: Point = Point {x: 0, y: 0};

				min_max_loc(&threshold_result, &mut min_val, &mut max_val, &mut min_loc, &mut max_loc, &Mat::default().unwrap()).unwrap();
				
				// Stop if there are no matches
				if max_val==min_val {break;}
				
				// Find the notes of the match and save them to the vector
				octaves.push(Octave::new(min_loc, &image, &template));

				// Filling the found point with a white rectangle
				// This is done so min_max_loc can locate the next instance of the template when it loops
				rectangle_points(&mut threshold_result,
					Point { x: min_loc.x - 10, y: 0 },
					Point { x: min_loc.x + template.cols().unwrap() - 10 , y: image.rows().unwrap() },
					Scalar::all(1.0), -1, 4, 0)
				.unwrap();
			}
			
			// Stop looping if matches have been found
			if !octaves.is_empty() {
				break;
			}
			// Panic if no matches are found ever
			if thresh > 1.0 {
				panic!();
			}
			// Otherwise, increase thresh and loop
			thresh = thresh + thresh_step;
			
		}

		// Build Piano struct
		let mut piano: Piano = Piano {
			octaves: octaves
		};
		piano.sort_octaves(&image); // Sort it and assign note codes
		
		return piano;
	}
	
	
	fn sort_octaves(&mut self, image: &Mat) {
		// Convert image to grayscale
		let mut image_gray: Mat = Mat::default().unwrap();
		cvt_color(&image, &mut image_gray, COLOR_RGB2GRAY, 0).unwrap();

		// Order octaves from lowest to highest
		//	Sort by x coordinate, the more to the left it is, the lowest the octave is
		self.octaves.sort_by(|a, b| a.notes[0].location.x.cmp(&b.notes[0].location.x));

		// FIND MIDDLE C:
		// For each C of each octave check for a gray dot
		let mut middle_c_octave_index = 0;
		for (index, octave) in self.octaves.iter().enumerate() {
			let note = &octave.notes[NOTE_C];
			
			// Probe for a gray pixel
			let thresh = 10;
			for y in note.location.y..image.rows().unwrap()-20 {
				let pixel: Vec3b = *image.at_2d(y, note.location.x).unwrap();
				
				// If the difference between the note's default color and the current pixel is greater than the threshold
				//	The pixel is considered gray
				//	Therefore, this is the middle C
				if (note.default_color[0] as i32 - pixel[0] as i32) > thresh {
					middle_c_octave_index = index;
					break;
				}
			}
		}

		// Update note codes		
		let index_to_octave_number: u8 = /* middle C = C4 + 1 because midi*/ 5 - middle_c_octave_index as u8; // Add this to an octave index to get the octave number
		
		// Update note codes
		let mut octave_index = 0;
		for octave in &mut self.octaves {
			for mut note in &mut octave.notes {
				note.code = ((octave_index+index_to_octave_number) * 12) as u8 + note.code;
			}
			octave_index = octave_index + 1;
		}
	}
	
	pub fn draw_notes(&self, image: &Mat) -> Mat {
		let mut image_show_octaves = Mat::default().unwrap();
		image.copy_to(&mut image_show_octaves).unwrap();
		
		for octave in &self.octaves {			
			// Draw octave line
			rectangle_points( &mut image_show_octaves,
				octave.notes[NOTE_C].location, octave.notes[NOTE_B].location,
				Scalar::new(255.0, 170.0, 50.0, 255.0), 2, 8, 0)
			.unwrap();

			// Draw each note
			for note in &octave.notes {
				circle( &mut image_show_octaves, note.location, 4, Scalar::new(202.0, 170.0, 70.0,255.0), -1, 8, 0).unwrap();
			}
		}
		
		return image_show_octaves;
	}

}

fn get_scaled_template(image: &Mat, template: &Mat) -> Mat {
	// Make Mat to store the scaled image
	let mut image_scaled: Mat = Mat::default().unwrap();

	// Variables to store the best match size and match value
	let mut found_scaled_width = template.size().unwrap().width;
	let mut found_min_val: f64 = 1.0;
	
	// Loop from the size of the unscaled template to the size of the image
	for scaled_width in template.size().unwrap().width..image.size().unwrap().width {
		
		// Resize the image to scale_width x scale_height
		let scale_proportion: f64 = scaled_width as f64 / image.size().unwrap().width as f64;
		let scaled_height = (image.size().unwrap().height as f64 * scale_proportion) as i32;
		resize(&image, &mut image_scaled, Size {width: scaled_width, height: scaled_height}, 0.0, 0.0, INTER_NEAREST).unwrap();
		
		// Do the Matching
		let mut result_scaled: Mat = Mat::default().unwrap();
		match_template(&image_scaled, &template, &mut result_scaled, TM_SQDIFF_NORMED, &Mat::default().unwrap()).unwrap();
		
		// Localizing the best match with minMaxLoc
		let mut min_val: f64 = 0.0;
		let mut max_val: f64 = 0.0;
		let mut min_loc: Point = Point {x: 0, y: 0};
		let mut max_loc: Point = Point {x: 0, y: 0};

		min_max_loc(&result_scaled, &mut min_val, &mut max_val, &mut min_loc, &mut max_loc, &Mat::default().unwrap()).unwrap();
		
		// Store match if it's better than the previously stored match (Lower = Better)
		if min_val < found_min_val {
			found_scaled_width = scaled_width;
			found_min_val = min_val;			
		}
		
		// Exit if the match is as good as it can get
		if min_val == 0.0 {break;}
		
		// Heuristic to avoid scaling up for nothing
		//	Exit if it's been a long time since a good match has been found
		if scaled_width - found_scaled_width > 100 {break;}		
	}
	
	
	let mut scaled_template: Mat = Mat::default().unwrap();
	let scale = image.size().unwrap().width / found_scaled_width;
	
	resize(&template, &mut scaled_template,
		Size {width: template.size().unwrap().width * scale , height: 1},
		0.0, 0.0, INTER_NEAREST)
	.unwrap();
	
	return scaled_template;
}