# Electronics
This is the electrical control board on which the firmware runs.

It's based on an `STM32F730R8T6`. The board receives stable `24V` from an external power supply unit and requires around `10A` of current.

On the board there's a switching regulator that converts the `24V` to `3.3V`, which is used by: `thermistors`, `display`, `MLX90614` and microcontroller.

## Schematic
This is the electrical schematic of the board:

![Schematic](https://github.com/Angelo13C/hot-plate/assets/55251189/c5c5dba4-4022-42da-989c-0a8ffcb93cc6)

## PCB
The PCB (dimensions: 52mm x 52mm) has 4 layers (PWR/SIGNAL, GND, GND, PWR/SIGNAL). This is an image of the top layer:

![image](https://github.com/Angelo13C/hot-plate/assets/55251189/57884144-48e2-4ce5-b8dc-deaf271366af)
