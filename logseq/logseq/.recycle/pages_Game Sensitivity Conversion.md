### Formula
	- This function combines the calculations of **BSF**, **SSF**, and **ARSA** to provide a full sensitivity conversion between two games/systems, taking into account **DPI**, **FOV**, **screen resolution**, and **aspect ratio** differences.
	- $$
	  \text{converted sensitivity} = \text{original sensitivity} \times SSF \times ARSA
	  $$
- ### Parameters
- ### Explanation
  The **converted sensitivity** takes into account the original game's sensitivity, the differences in FOV and resolution, and the aspect ratio between the two games.
  First, the **MMF** is calculated for the original game.
  Then, the **SSF** is calculated to adjust for the FOV and resolution differences.
  Finally, the **ARSA** is applied to adjust for the aspect ratio differences between the original and target games.
- ## Game Sensitivity Conversion
  
  This formula is used to convert sensitivity settings from one game to another, adjusting for differences in DPI, FOV, resolution, and aspect ratio between the original and target games.
- ### Formula:
  $$ \text{Converted Sensitivity} = \text{original\_sens} \times \text{SSF} \times \text{ARSA} $$
- ### Parameters:
- `original_sens`: The sensitivity in the original game (the game you're converting from).
- `SSF`: The **Standard Sensitivity Factor** that adjusts the sensitivity based on FOV and resolution differences between the original and target games.
- `ARSA`: The **Aspect Ratio Sensitivity Adjustment** that adjusts the sensitivity based on aspect ratio differences between the system and target game's screen.
- ### Description:
  This formula applies the previously calculated **SSF** (Standard Sensitivity Factor) and **ARSA** (Aspect Ratio Sensitivity Adjustment) to the original sensitivity value from the original game, ensuring a consistent sensitivity feel in the target game.