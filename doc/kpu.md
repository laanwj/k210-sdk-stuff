KPU
===

Some notes about the K210 KPU, which is definitely the weirdest, possibly
most interesting peripheral on this SoC. Documentation doesn't seem to be
available, so the information here has been reconstructed from various vendor
source code.

This kind of custom hardware is pretty much impossible to understand without
knowledge of the domain, in this case Convolutional Neural Networks on images.
My understanding of this is rudimentary (my last brush with it was in uni) so
I may be missing some obvious clues here and there.

From the datasheet
==================

The Kendryte datasheet has the following information on the KPU:

> KPU is a general-purpose neural network processor with built-in convolution,
> batch normalization, activation, and pooling operations. It can detect faces or
> objects in real time. The specific characteristics are as follows:
> 
> - Supports the fixed-point model that the mainstream training framework trains
> according to specific restriction rules
> - There is no direct limit on the number of network layers, and each layer of
> convolutional neural network parameters can be configured separately, including
> the number of input and output channels, and the input and output line width
> and column height
> - Support for 1x1 and 3x3 convolution kernels

1×1 and 3×3 is not a very wide range of supported convolutions, but maybe the most
common ones in this specific application area…

> - Support for any form of activation function

This is definitely true, Normalization functions seem to be represented as an array of 16
segments (`kpu_activate_table_t`).

> - The maximum supported neural network parameter size for real-time work is 5MiB
> to 5.9MiB
> - The maximum supported network parameter size when working in non-real time is
> (flash size - software size)

The flash size specs are somewhat of a red herring as they relate to software
instead of hardware: the KPU does not have logic for loading parameters from
flash.

Some other source mentions:

> 64 KPU which are 576bit width, supports convolution kernel. Offers
> 0.25TOPS@0.3W,400MHz, and when you overclock to 800MHz, it offers 0.5TOPS,
> meaning you can do object recognition 60fps@VGA.

Clock speed
===========

The KPU is clocked from PLL1, with a divisor between 1 and 16.
The usual clock speed in the Sipeed examples is 300, sometimes 400 MHz.
According to some mentions in the data sheet it's possible to clock it to 800 MHz.

Overall execution flow
======================

The overall execution flow is that the KPU runs a neural network layer by
layer. This happens in a sequential fashion. Each layer can be considered a
separate set of instructions for the KPU.

A layer can receive its input in the "AI" memory area (2MB of the memory is reserved for this,
from 0x40600000 to 0x407fffff) as well as write its output there. The input and
output can consist of multiple channels (R/G/B for example).

It is possible to set an interrupt to notify the host CPU when a specific layer has
finished executing.

Looking at `lib/drivers/kpu.c` in the SDK, function `ai_step`, many types of CNN layers are
implemented in software instead of executed by the KPU. I suppose they accelerated the
most common multiplication-intensive layers in hardware, which is `KL_K210_CONV`.

Peripheral layout
=================

The register layout of the peripheral is as follows. Source: `lib/drivers/include/kpu.h`.
All registers are 64-bit.

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `layer_argument_fifo` | Layer arguments (instructions) are submitted here         |
| 0x08  | `interrupt_status` | Status of pending interrupts                                 |
| 0x10  | `interrupt_raw`   |                                                               |
| 0x18  | `interrupt_mask`  | Specifies which global interrupts are enabled                 |
| 0x20  | `interrupt_clear` | Clear pending interrupts                                      |
| 0x28  | `fifo_threshold`  | FIFO interrupt thresholds                                     |
| 0x30  | `fifo_data_out`   | Data output FIFO read register                                |
| 0x38  | `fifo_ctrl`       | Flush FIFOs                                                   |
| 0x40  | `eight_bit_mode`  | Enable 8-bit instead of 16-bit precision                      |

Layer format
============

KPU neural network layers are represented by a series of 12 64-bit values,
submitted to the layer argument FIFO one by one. The overall structure of the bit fields is
available in `lib/drivers/include/kpu.h`.

It looks like the generation of models is supposed to be done offline by a tool called [nnscase](https://github.com/kendryte/nncase),
which compiles TensorFlow models to a specific internal representation.
The k210-specific code parts are [k210_ops.cpp](https://github.com/kendryte/nncase/tree/master/src/codegen/ops/k210/k210_ops.cpp)
and [k210_sim_types.h](https://github.com/kendryte/nncase/blob/master/src/common/include/runtime/k210/k210_sim_types.h)
and [k210_ops_body.h](https://github.com/kendryte/nncase/blob/master/src/common/include/runtime/k210/k210_ops_body.h)
(serialization and deserialization).
src/common/include/kernels/k210/k210_kernels.h (emulation)

0 `interrupt_enabe`
-------------------

(register and field names are from the SDK header files, typos are as-is)

    bit    name
    ------ ----------------------
    0      `int_en`               Generate interrupt after layer computation finished
    1      `ram_flag`             ?
    2      `full_add`             Set in `kpu_conv2d_output_full_add`
    3      `depth_wise_layer`     Is a "depth-wise" layer (1 if enabled)
    4..63  reserved

"depth-wise" affects many of the computations: it likely means that the layer
computation mixes multiple channels so that they cannot be processed one by
one.

1 `image_addr`
--------------

    bit    name
    ------ ----------------------
    0..14  `image_src_addr`       Image source address
    15     reserved
    16..30 `image_dst_addr`       Image destination address
    31..63 reserved

`image_src_addr` and `image_dst_addr` are specified in 64-byte units relative to the base of "AI" memory.

2 `image_channel_num`
---------------------

    bit    name
    ------ ----------------------
    0..9   `i_ch_num`             Number of input channels (minus one)
    10..31 reserved
    32..41 `o_ch_num`             Number of output channels (minus one)
    42..47 reserved
    48..57 `o_ch_num_coef`        Number of output channel coefficients (minus one)
    58..63 reserved

3 `image_size`
--------------

    bit    name
    ------ ----------------------
    0..9   `i_row_wid`            Input row width (minus one)
    10..18 `i_col_high`           Input column height (minus one)
    19..31 reserved
    32..41 `o_row_wid`            Output row width (minus one)
    42..50 `o_col_high`           Output column height (minus one)
    51..63 reserved

4 `kernel_pool_type_cfg`
------------------------

    bit    name
    ------ ----------------------
    0..2   `kernel_type`      `filter_type_t` (see below)
    3      `pad_type`         Always 1
    4..7   `pool_type`        `pool_type_t` (see below)
    8      `first_stride`     ?
    9      `bypass_conv`      ?
    10     `load_para`        Load parameters (1 if enabled)
    11..15 reserved
    16..23 `dma_burst_size`   Always 15
    24..31 `pad_value`        Padding value
    32..63 `bwsx_base_addr`   Batch normalization array base address (8-aligned, `kpu_batchnorm_argument_t`)

`kpu_filter_type`:

    value   enum
    ------  -----------
    0       1x1
    1       3x3

`kpu_pool_type`:

    value   enum           description
    ------  -------------- ------------------
    0       bypass         bypass pooling (filter size 1×1, stride 1)
    1       max_2_s2       max pooling (filter size 2×2, stride 2)
    2       mean_2_s2      mean pooling (filter size 2×2, stride 2)
    3       max_4_s4       max pooling (filter size 4×4, stride 4)
    4       mean_4_s4      mean pooling (filter size 4×4, stride 4)
    5       left_top_2_s2  pick left top (filter size 2×2, stride 2)
    6       right_top_2_s2 pick right top (filter size 2×2, stride 2)
    7       left_top_4_s4  pick left top (filter size 4×4, stride 4)
    8       mean_2_s1      mean pooling (filter size 2×2, stride 1)
    9       max_2_s1       max pooling (filter size 2×2, stride 1)

See `kpu_pool2d` in `src/common/include/kernels/k210/k210_kernels.h`,
as well as `src/common/include/runtime/k210/k210_runtime_op_utility.h` in nncase.

5 `kernel_load_cfg`
-------------------

    bit    name
    ------ ----------------------
    0      `load_coor`       Always 1
    1..6   `load_time`       Parameter load frequency (0=once, 1=per channel?)
    7..14  reserved
    15..31 `para_size`       Parameter (weights) size
    32..63 `para_start_addr` Parameter (weights) start address (128-aligned, one byte per weight)

6 `kernel_offset`
-----------------

    bit    name
    ------ ----------------------
    0..3   `coef_column_offset`  ?
    4..15  `coef_row_offset`     ?
    16..63 reserved

7 `kernel_calc_type_cfg`
------------------------

    bit    name
    ------ ----------------------
    0..14  `channel_switch_addr`  In layout channel length
    15     reserved
    16..19 `row_switch_addr`      In layout row length
    20..27 `coef_size`            ?
    28..30 `coef_group`           ?
    31     `load_act`             Load activation function (1 is enabled)
    32..63 `active_addr`          Activation function address (256-aligned `kpu_activate_table_t`)

8 `write_back_cfg`
------------------

    bit    name
    ------ ----------------------
    0..14  `wb_channel_switch_addr`  Out layout channel length
    15     reserved
    16..19 `wb_row_switch_addr`      Out layout row length
    20..22 `wb_group`                Out layout number of groups
    23..63 reserved

9 `conv_value`
--------------

    bit    name
    ------ ----------------------
    0..3   `shr_w`                   Convolution value shift right w
    4..7   `shr_x`                   Convolution value shift right x
    8..31  `arg_w`                   Convolution value w multiplier
    32..55 `arg_x`                   Convolution value x multiplier
    56..63 reserved

10 `conv_value2`
----------------

    bit    name
    ------ ----------------------
    0..39  `arg_add`                 Convolution value addition/bias
    40..63 reserved

11 `dma_parameter`
------------------

    bit    name
    ------ ----------------------
    0      `send_data_out`           Send data out to DMA (main memory)
    1..15  reserved
    16..31 `channel_byte_num`        Number of bytes per out channel (minus one)
    32..63 `dma_total_byte`          Number of bytes total out (minus one)
