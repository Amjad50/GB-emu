gb_tests!(
    inf; // clock until infinite loop

    blargg_cpu_instrs,
    "blargg-gb-tests/cpu_instrs/cpu_instrs.gb",
    2413260581157633908;

    blargg_instr_timing,
    "blargg-gb-tests/instr_timing/instr_timing.gb",
    17422352976553522419;

    blargg_halt_bug,
    "blargg-gb-tests/halt_bug.gb",
    8499482420170558727;

    // dmg_sound individual temporary for now, until all passes, we can use
    // the full one
    blargg_dmg_sound_01_registers,
    "blargg-gb-tests/dmg_sound/rom_singles/01-registers.gb",
    10531887144713037986;

    blargg_dmg_sound_02_len_ctr,
    "blargg-gb-tests/dmg_sound/rom_singles/02-len ctr.gb",
    15236646370068726987;

    blargg_dmg_sound_03_trigger,
    "blargg-gb-tests/dmg_sound/rom_singles/03-trigger.gb",
    362609057241937141;
);
