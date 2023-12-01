mod instruction;
mod channel;

use channel::*;
use instruction::*;

fn main() {

    let epsilon = (0.1 + 0.2 - 0.3) * 1e-4;
    println!("epsilon = {epsilon}");
    println!("epsilon == 0.0 is {}", epsilon == 0.0);
    // println!("diff == diff after conversion is {}", diff == ((diff as f32) as f64));
    // println!("diff - diff after conversion = {}", diff - ((diff as f32) as f64));

    let samp_rate = 1e6;
    let clock_cycle = 1.0 / samp_rate;
    let mut my_chan = Channel::new(TaskType::AO, "ao0", samp_rate, 0.0);

    // // Test of collision on the left ===========================================================
    // // (1) Serious collision
    // my_chan.add_instr(Instruction::new_const(0.1), 0.0, Some((1.0, true)));
    // my_chan.add_instr(Instruction::new_const(0.2), 0.5, None);  // Some((1.0, true))
    //
    // // (2) 1-tick collision
    // let mut t = 0.0;
    // let dur = 1.5 * clock_cycle;
    // for i in 0..1e4 as usize {
    //     my_chan.add_instr(Instruction::new_const(i as f64), t, Some((dur + epsilon, true)),);
    //     // println!("Added pulse {i}");
    //     t += dur;
    // }

    // // Test of collision on the right ==========================================================
    // // (1) Serious collision
    // my_chan.add_instr(Instruction::new_const(0.1), 1.0, Some((1.0, true)));
    // my_chan.add_instr(Instruction::new_const(0.2), 0.5, Some((1.0, true)));
    //
    // // (2) 1-tick collision
    // // let mut t = 0.0;
    // // let dur = 1.5 * clock_cycle;
    // // for i in 0..1e4 as usize {
    // //     my_chan.add_instr(Instruction::new_const(i as f64), t + dur, Some((dur, true)),);
    // //     my_chan.add_instr(Instruction::new_const(i as f64), t, Some((dur + epsilon, true)),);
    // //     t += 2.0 * dur;
    // // }

    // Tests with very short pulses ============================================================

    // // (1) Pulse collapse
    // // t = 0.1 * clock_cycle, dur = 0.8 * clock_cycle (t_end = 0.9*clock_cycle) - should give start_pos = 0 and stop_pos = 1
    // // t = 0.6 * clock_cycle, dur = 0.8 * clock_cycle (t_end = 1.4*clock_cycle) - should collapse and panic
    //
    // my_chan.add_instr(Instruction::new_const(1.0), 0.1*clock_cycle, Some((0.8*clock_cycle, true)));
    // my_chan.add_instr(Instruction::new_const(2.0), 0.6*clock_cycle, Some((0.8*clock_cycle, true))); // thread 'main' panicked at 'Too short pulse! 1, 1'

    // // (2) Collision cannot be resolved by trimming - too short pulse
    // // 0.0 --- 1.5+epsilon
    // // 1.5 --- 2.5-epsilon
    // my_chan.add_instr(Instruction::new_const(1.0), 0.0*clock_cycle, Some((1.5*clock_cycle + epsilon, true)));
    // my_chan.add_instr(Instruction::new_const(2.0), 1.5*clock_cycle - epsilon, None);  // Some((1.0*clock_cycle - epsilon, false))
}