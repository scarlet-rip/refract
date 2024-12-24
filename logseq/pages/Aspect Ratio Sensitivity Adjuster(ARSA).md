### Formula
	- The **Aspect Ratio Sensitivity Adjuster (ARSA)** adjusts **BSF** based on the difference in aspect ratio between the original system's screen and the target system's screen.
	- $$
	  ARSA = \left( \frac{\text{target screen width}}{\text{target screen height}} \right) \div \left( \frac{\text{original screen width}}{\text{original screen height}} \right)
	  $$
- ### Parameters
	- `original screen width`: The width of the original system's screen resolution in pixels.
	- `original screen height`: The height of the original system's screen resolution in pixels.
	- `target screen width`: The width of the target system's screen resolution in pixels.
	- `target screen height`: The height of the target system's screen resolution in pixels.
- ### Rust Example
	- [Rust Playground](https://play.rust-lang.org/?gist=25eb0fb7c738d023dd4df2d20cfa0d36)
	- ```rust
	  fn calculate_arsa(
	      target_screen_width: u16,
	      target_screen_height: u16,
	      original_screen_width: u16,
	      original_screen_height: u16,
	  ) -> f64 {
	      let tar = target_screen_width as f64 / target_screen_height as f64;
	      let oar = original_screen_width as f64 / original_screen_height as f64;
	  
	      tar / oar
	  }
	  ```