// Base Cortex-M abstraction
use cortex_m::peripheral::{SCB, SYST};

// BSP stuff
use bsp::hal;
use feather_m4 as bsp;

// Hardware Abstraction Libs
use hal::pac::CorePeripherals;

pub const IDLE_STACK_SIZE: usize = 256;
pub const TASK_STACK_SIZE: usize = 2048;

/**
* Create a common structure useful for sharing base hardware resources with kernel and tasks
*   - system control block (from cortex-m)
*   - system tick (from cortex-m)
*   - clocks (from feather-m4-bsp)
*/
pub struct KernelResources {
    pub scb: SCB,
    pub systick: SYST,
}

pub fn setup() -> KernelResources {
    let core = CorePeripherals::take().unwrap();
    let scb = core.SCB;
    let systick = core.SYST;

    KernelResources { scb, systick }
}
