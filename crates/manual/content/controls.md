This chapter lists every keyboard and mouse control the simulator recognizes, with the
exact numeric behavior behind each one.

| Control | Effect |
|---|---|
| <kbd>Space</kbd> | Toggles pause. |
| Scroll wheel | Zooms the camera in or out. |
| Right mouse button, drag | Pans the camera. |
| Middle mouse button, click | Resets pan and zoom. |
| Left mouse button, click on a particle | Toggles the tag on that particle. |

## Pause

<kbd>Space</kbd> toggles the pause state. The simulator ignores <kbd>Space</kbd> while a
text field in the Parameters window has focus, so typing a space into the **Config Path**
or **History Path** field does not pause the run.

## Zoom

The scroll wheel changes a zoom factor by 0.05 per wheel notch, clamped to the range 1 to
10. The camera's transform scale is:

```
scale = 1 / (zoom * CAMERA_ZOOM)
```

`CAMERA_ZOOM` is a constant equal to 2.5. At the default zoom factor of 1, the scale is
0.4; at the maximum zoom factor of 10, the scale is 0.04.

## Pan

Holding the right mouse button and moving the mouse pans the camera. The pan offset
accumulates for as long as the button stays down, and scales with the current zoom, so
panning covers the same apparent screen distance at any zoom level.

> [!NOTE]
> The camera always follows the ship first, then adds the pan offset on top. Panning
> never detaches the view from the ship; it only shifts the view around it.

## Reset pan and zoom

Clicking the middle mouse button sets the zoom factor to 1 and the pan offset to zero, in
one action. It does not move the ship or change any physics parameter.

## Tag a particle

Left-clicking a particle toggles its tag. A tagged particle turns magenta and stays
visible even when **Hide untagged particles** is on, on the Visuals tab of the Parameters
window. See [Tag particles](tagging.html) for what tagging is for.

> [!NOTE]
> There is no gamepad support and no key rebinding. The simulator reads only the mouse
> and the keyboard, and every binding in the table above is fixed.
