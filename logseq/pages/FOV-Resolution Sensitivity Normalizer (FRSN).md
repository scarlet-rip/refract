### Formula
	- The **FOV-Resolution Sensitivity Normalizer (FRSN)** adjusts **Converted Sensitivity** based on the differences in Field of View (FOV) and monitor resolution between the original and target game/system.
	  id:: 6767e288-def9-41c4-8f94-cafeff9fb0d4
	- $$
	  FRSN = \left( \frac{\text{target game FOV}}{\text{original game FOV}} \right) \times \left( \frac{\text{target horizontal screen resolution}}{\text{original horizontal screen resolution}} \right)
	  $$
- ### Parameters
	- `target game FOV`: The field of view (FOV) of the target game.
	- `original game FOV`: The field of view (FOV) of the original game.
	- `target horizontal screen resolution`: The resolution of the target system's screen in horizontal pixels.
	- `original horizontal screen resolution`: The resolution of the original system's screen in horizontal pixels.
- ### Rust Example
	- [Rust Playground](https://play.rust-lang.org/?gist=2ea10902f732ccde3a1b5b04beac46e5)
	- ```rust
	  fn calculate_frsn(
	      target_game_fov: u16,
	      original_game_fov: u16,
	  	target_horizontal_screen_resolution: u16,
	      original_horizontal_screen_resolution: u16,
	  ) -> f64 {
	      let ff = (target_game_fov as f64) / (original_game_fov as f64);
	      let rf = (target_horizontal_screen_resolution as f64)
	          / (original_horizontal_screen_resolution as f64);
	  
	      ff * rf
	  }
	  ```