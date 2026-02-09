**Name:** J  

---

## Overview

This assignment implements a tilt-based “level” using the BBC micro:bit and the onboard LSM303AGR accelerometer. The program reads real-time acceleration data over I2C and maps the X and Y tilt values to a 5×5 LED display grid. A single illuminated LED represents the board’s orientation, behaving like a digital bubble level.

The system operates in two modes:

- **Coarse mode** – larger movement range (±500 mg)
- **Fine mode** – smaller movement range (±50 mg)

Button A enables coarse mode, and Button B enables fine mode.

---

## Implementation Details

The project uses:

- `#![no_std]` and `#![no_main]`
- `cortex_m_rt` for the entry point
- `embedded_hal` traits for delay and input handling
- `lsm303agr` crate for accelerometer communication
- `microbit` board support crate
- RTT for debugging output

The accelerometer is initialized over the internal I2C bus using `Twim`. It is configured in High Resolution mode at 10 Hz output data rate.

Inside the main loop:

1. The program waits until new accelerometer data is available.
2. X and Y acceleration values (in milli-g) are read.
3. If the board is upside down (positive Z acceleration), the display is blanked.
4. Otherwise:
   - The acceleration values are scaled into LED coordinates (0–4).
   - A single LED is illuminated at the computed position.
5. The display updates every 200 ms.

Scaling is handled by a helper function that maps `-range..+range` to `0..4`, clamping values outside the range.

---

## What Went Well

The hardware abstraction layers made the I2C and accelerometer setup straightforward once configured correctly. The `lsm303agr` crate handled most of the lower-level sensor communication cleanly.

Refactoring the LED logic improved clarity by:
- Creating a fresh LED grid each loop iteration
- Reducing duplicate clearing logic
- Centralizing the display update

This made the program easier to reason about and eliminated compiler warnings about unused assignments.

---

## Challenges

One subtle issue involved understanding Rust’s `unused_assignments` warning. An initial LED array assignment was being overwritten before being read. Moving the LED grid initialization inside the loop resolved the warning and improved structure.

Another consideration was choosing appropriate scaling ranges. Fine mode required significantly tighter bounds to feel responsive without saturating the display.

---

## Observations

- The micro:bit display feels surprisingly expressive even with only 5×5 resolution.
- Small structural refactors significantly improve readability in embedded Rust.
- Waiting for fresh sensor data before reading prevents unnecessary polling and keeps updates consistent.
- Mode switching could be made more responsive by adjusting the display delay timing.

Overall, the implementation behaves like a functional digital level and demonstrates clean embedded Rust structure with proper hardware abstraction and safe code practices.