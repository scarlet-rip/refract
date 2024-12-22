- $$
  \text{360° Turn Distance in cm} = \frac{\text{Horizontal distance traveled in one 360° turn in pixels}}{\text{Monitor PPI}} \times \text{CM\_IN\_INCH}
  $$
- **Horizontal distance traveled in one 360° turn in pixels**: The distance in pixels that the cursor travels horizontally when you perform a full 360° turn in a 3D game.
- **Monitor PPI (Pixels Per Inch)**: The pixel density of your monitor, measured in pixels per inch.
- **CM_IN_INCH**: A constant equal to **2.54** (centimeters in an inch).
- The **360° Turn Distance in cm** is game-specific and constant for all users of the game, as it is determined by the game's internal sensitivity mechanics.
  
  It should be stored in the database for each game. This allows users to input their **DPI** and **PPI** to dynamically match their sensitivities across different games.