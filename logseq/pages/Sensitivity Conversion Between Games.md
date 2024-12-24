### Formula
	- This formula combines the results of **BSF**, **FRSN** and **ARSA** to provide full sensitivity conversion between two games/systems.
	- $$
	  \text{converted sensitivity} = \left( \frac{{\text{original in-game sensitivity} \times \text{target BSF}}}{{\text{original BSF}}} \right) \times \text{original to target FRSN} \times \text{original to target ARSA}
	  $$
- ### Parameters
	- `original in-game sensitivity`: The in-game sensitivity of the game you're converting from.
	- `original BSF`: [[Base Sensitivity Factor (BSF)]]
	- `target BSF`: [[Base Sensitivity Factor (BSF)]]
	-
	- `original to target FRSN`: [[FOV-Resolution Sensitivity Normalizer (FRSN)]]
	- `original to target ARSA`: [[Aspect Ratio Sensitivity Adjuster(ARSA)]]
- ### Rust Example
  id:: 676a9147-49cd-45fb-b712-c287ba479ed5
	- [Rust Playground](https://play.rust-lang.org/?gist=5a3eab1a5e428d14675fa387f39a23a6)
	- ```rust
	  fn convert_sensitivity(
	      original_in_game_sensitivity: f64,
	      original_bsf: f64,
	      target_bsf: f64,
	      frsn: f64,
	      arsa: f64,
	  ) -> f64 {
	      let bcs = original_in_game_sensitivity * original_bsf;
	      let aas = bcs / target_bsf;
	  
	      aas * frsn * arsa
	  }
	  ```