mod instruction;
mod channel;

use channel::*;
use instruction::*;

fn main() {

    let diff = (0.1 + 0.2 - 0.3) * 1e-4;
    println!("diff = {diff}");
    println!("diff == 0.0 is {}", diff == 0.0);
    // println!("diff == diff after conversion is {}", diff == ((diff as f32) as f64));
    // println!("diff - diff after conversion = {}", diff - ((diff as f32) as f64));

    let samp_rate = 1e6;
    let clock_cycle = 1.0 / samp_rate;
    let mut my_chan = Channel::new(TaskType::AO, "ao0", samp_rate, 0.0);

    // // Test of collision on the left
    // let mut t = 0.0;
    // let dur = 1.5 * clock_cycle;
    // for i in 0..1e4 as usize {
    //     my_chan.add_instr(Instruction::new_const(i as f64), t, Some((dur + diff, true)),);
    //     // println!("Added pulse {i}");
    //     t += dur;
    // }

    // // Test of collision on the right
    // let mut t = 0.0;
    // let dur = 1.5 * clock_cycle;
    // for i in 0..1e4 as usize {
    //     my_chan.add_instr(Instruction::new_const(i as f64), t + dur, Some((dur, true)),);
    //     my_chan.add_instr(Instruction::new_const(i as f64), t, Some((dur + diff, true)),);
    //     t += 2.0 * dur;
    // }

    // Tests with very short pulses
    // (1) Pulse collapse
    // t = 0.1 * clock_cycle, dur = 0.8 * clock_cycle (t_end = 0.9*clock_cycle) - should give start_pos = 0 and stop_pos = 1
    // t = 0.6 * clock_cycle, dur = 0.8 * clock_cycle (t_end = 1.4*clock_cycle) - should collapse and panic

    // (2) Collision cannot be resolved by trimming - too short pulse
    // 0.0 --- 1.5
    // 1.5-dur --- 2.4
}