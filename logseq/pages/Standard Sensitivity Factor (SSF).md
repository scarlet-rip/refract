### Formula
	- The **Standard Sensitivity Factor (SSF)** adjusts **BSF** based on the differences in Field of View (FOV) and monitor resolution between the original and target game/system.
	- $$
	  SSF = BSF \times \left( \frac{\text{target game FOV}}{\text{original game FOV}} \right) \times \left( \frac{\text{original horizontal screen resolution}}{\text{target horizontal screen resolution}} \right)
	  $$
- ### Parameters
	- `BSF`: [[Base Sensitivity Factor (BSF)]]
	- `target game FOV`: The field of view (FOV) of the target game.
	- `original game FOV`: The field of view (FOV) of the original game.
	- `original horizontal screen resolution`: The resolution of the original system's screen in horizontal pixels.
	- `target horizontal screen resolution`: The resolution of the target system's screen in horizontal pixels.