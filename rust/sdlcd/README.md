# `sdlcd`

This example streams raw data from the SD card to the LCD, frame by frame.

The following commands can be used to scale and transcode a video to
`320x240xRGB565`, and write it to a SD card:

```sh
ffmpeg -i input.mp4  -vf scale=320:240 -vcodec rawvideo -f rawvideo -pix_fmt rgb565le test.vid
dd if=test.vid of=/dev/mmcblk… bs=153600
```
