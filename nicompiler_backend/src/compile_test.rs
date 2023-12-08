mod instruction;
mod channel;

use crate::instruction::*;
use crate::channel::*;

fn main() {
    let mut my_chan = Channel::new(TaskType::AO, "ao0", 1e6, -10.0);

    let clock_period = my_chan.clock_period();

    // Padding from 0 to the first instruction + padding after with "don't keep val".
    // // - no gap to 0
    // my_chan.add_instr(
    //     Instruction::new_sine(1.234, Some(1.0), None, None),
    //     0.0, Some((1.0, false))
    // );
    // // - finite gap to 0
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(1.0), None, None),
        1.0, Some((1.0, false))
    );

    // Padding after instruction - keep val
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(2.0), None, None),
        3.0, Some((1.0, true))
    );

    // "go-something" instruction
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(3.0), None, None),
        5.0, None
    );

    // Back-to-back pulses (no padding between)
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(4.0), None, None),
        6.0, Some((1.0, true))
    );
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(5.0), None, None),
        7.0, Some((1.0, true))
    );

    // Test of identical instruction merging
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(1.0), None, None),
        9.0, Some((1.0, false))
    );
    // distinguishes pulses for
    //      `freq = 1.234 + 1.0e-15`
    // and merges into one for
    //      `freq = 1.234 + 1.0e-18`
    my_chan.add_instr(
        Instruction::new_sine(1.234 + 1.0e-15, Some(1.0), None, None),
        10.0, Some((2.0, false))
    );
    my_chan.constant(-5.0, 12.0, None);

    // // Final pulse - padding to stop_pos
    // // - specified end, gap to stop_pos
    my_chan.add_instr(
        Instruction::new_sine(1.234, Some(6.0), None, None),
        14.0, Some((1.0, true))
    );
    // // - specified end, no gap to stop_pos
    // my_chan.add_instr(
    //     Instruction::new_sine(1.234, Some(6.0), None, None),
    //     14.0, Some((2.0, true))
    // );
    // // - no specified end
    // my_chan.constant(5.0, 14.0, None);
    // // - no specified end right before stop_pos
    // my_chan.constant(10.0, 16.0-0.000001, None);

    my_chan.compile((16.0 * my_chan.samp_rate()).round() as usize);

    for idx in 0..my_chan.instr_val().len() {
        let func = my_chan.instr_val()[idx].clone();
        let end_pos = my_chan.instr_end()[idx];

        println!("{idx}: {func} - {end_pos}");
    };
}