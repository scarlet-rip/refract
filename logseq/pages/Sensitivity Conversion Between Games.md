### Formula
	- $$
	  \text{converted sensitivity} = \text{original in-game sensitivity} \times \left( \frac{\text{target pixels per 360°}}{\text{original pixels per 360°}} \right)
	  $$
- ### Parameters
	- `original in-game sensitivity`: The sensitivity setting in the game you are converting from.
	- `target pixels per 360°`: The number of pixels corresponding to a horizontal 360° turn in the target game.".
	- `original pixels per 360°`: The number of pixels corresponding to a horizontal 360° turn in the original game.
- ### Rust Example
  id:: 676a9147-49cd-45fb-b712-c287ba479ed5
	- [Rust Playground](https://play.rust-lang.org/?gist=5fb8567d3e619c92e096ffc33186f583)
	- ```rust
	  fn convert_sensitivity(
	      original_in_game_sensitivity: f64,
	      original_pixels_per_360: u16,
	      target_pixels_per_360: u16,
	  ) -> f64 {
	      let d360_difference = target_pixels_per_360 as f64 / original_pixels_per_360 as f64;
	  
	      original_in_game_sensitivity * d360_difference
	  }
	  ```