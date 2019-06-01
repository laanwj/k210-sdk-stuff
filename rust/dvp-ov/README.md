# `dvp-ov`

A straightforward passthrough test for video handling, based on `dvp_ov` in the
SDK: read frames from the OV2640 image sensor and display them on the LCD.

The performance is likely worse (don't know by how much) than the C
implementation because currently, no interrupts and DMA are used.
