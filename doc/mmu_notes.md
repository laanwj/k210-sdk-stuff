Notes on K210 MMU
=================

The K210 has a MMU based on the RISC-V privileged spec 1.9.1.

This means that:

- The `sfence.vm` (bit pattern `0x10400073`) instruction should be used to
  synchronize updates to the page tables with current execution, not
  `sfence.vma` (bit pattern `0x12000073`). See spec section 4.2.1.

- The `sptbr` CSR (`0x180`, Supervisor Page-Table Base Register) specifies the
  current root page table.  In the newer spec this register is called `satp`
  and is completely different.

- In spec v1.9.1, `sstatus.SUM` is still `PUM` which has opposite meaning.

- To switch on paging, set the `VM` bitfield (offset 24:28) of `mstatus` to `Sv39` (9).

Also, [reportedly](https://www.reddit.com/r/RISCV/comments/fguddz/linux011_with_mmu_for_k210_riscv/fk9otke/) interrupts do not work as expected in S mode, making that
mode more or less useless. Only M and U mode can be used in practice.

To access paged memory from M mode, the `MPRV` bit in `mstatus` can be set, with `MPP` set to
`0` (U mode). From the spec (section 3.1.9):

> The MPRV bit modifies the privilege level at which loads and stores execute. When MPRV=0,
> translation and protection behave as normal. When MPRV=1, data memory addresses are trans-
> lated and protected as though the current privilege mode were set to MPP. Instruction address-
> translation and protection are unaffected

Examples
---------

There is a little bit of example code for using the MMU on the K210:

- [https://github.com/lizhirui/K210-Linux0.11.git](Linux0.11 with MMU)
- [https://github.com/44670/libk9/blob/develop/k9/CacheEngine.c](CacheEngine.c)
- [https://github.com/oscourse-tsinghua/rcore_plus/issues/34#issuecomment-485145060](Some notes on porting a rust-based OS to K210)
