### Formula
	- This function combines the calculations of **SSF**, and **ARSA** to provide a full sensitivity conversion between two games/systems, taking into account **DPI**, **FOV**, **screen resolution**, and **aspect ratio** differences.
	- $$
	  \text{converted in-game sensitivity} = \text{original in-game sensitivity} \times SSF \times ARSA
	  $$
- ### Parameters
	- `original in-game sensitivity`: The in-game sensitivity of the game you're converting from.
	- `SSF`: [[Standard Sensitivity Factor (SSF)]].
	- `ARSA`: [[Aspect Ratio Sensitivity Adjustment (ARSA)]].
- ### Explanation
	- This formula applies **SSF** and **ARSA** to the original sensitivity value from the original game.