### Formula
	- The **Base Sensitivity Factor (BSF)** is used to normalize sensitivity across different games by accounting for factors like **DPI**, **in-game sensitivity**, **horizontal 360° turn distance**, and **PPI**.
	- $$
	  BSF = \frac{{DPI \times \text{in-game sensitivity}}}{{\text{horizontal 360° distance (in pixels)} \times PPI}}
	  $$
- ### Parameters
	- `DPI`: The mouse DPI (dots per inch).
	- `in-game sensitivity`: The in-game sensitivity setting in the game.
	- `horizontal 360° distance (in pixels)`: The number of pixels required to make a horizontal 360° turn in the game.
	- `PPI`: The PPI (pixels per inch) of the screen.
- ### Explanation
	- **BSF** represents the relationship between the mouse's **DPI**, the **in-game sensitivity**, and **how much pixel movement is required for a 360° turn**.
	- It divides the mouse's **DPI** and the **in-game sensitivity** by the **horizontal 360° turn distance (in pixels)** and the screen's **PPI**.
	- **horizontal 360° distance (in pixels)** can be used as a universal value belonging to a specific game, meaning that it can be shared among people with different setups and represents a game's internal sensitivity mechanics.