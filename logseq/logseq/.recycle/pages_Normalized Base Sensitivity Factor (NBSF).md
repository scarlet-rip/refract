### Formula
	- The **Normalized Base Sensitivity Factor (NBSF)** adjusts **BSF** based on **ARSA** and **FRSF**
	- $$
	  FRSN = \left( \frac{\text{target game FOV}}{\text{original game FOV}} \right) \times \left( \frac{\text{target horizontal screen resolution}}{\text{original horizontal screen resolution}} \right)
	  $$
- ### Parameters
	- `target game FOV`: The field of view (FOV) of the target game.
	- `original game FOV`: The field of view (FOV) of the original game.
	- `target horizontal screen resolution`: The resolution of the target system's screen in horizontal pixels.
	- `original horizontal screen resolution`: The resolution of the original system's screen in horizontal pixels.
- ### Rust Example
	- [Rust Playground](1.2857142857142858)
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