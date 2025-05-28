use std::collections::VecDeque;
use crate::error::MemoryError;
use super::Device;

// Define the console's memory-mapped register offsets relative to its base_address
const DATA_REGISTER_OFFSET: u16 = 0x00;
const STATUS_REGISTER_OFFSET: u16 = 0x01;
const CONTROL_REGISTER_OFFSET: u16 = 0x02;
const CONSOLE_DEVICE_SIZE: u16 = 4; // We now use 3 bytes for the device registers

// Status Register Bits
const RX_READY_BIT: u8 = 0b0000_0001; // Bit 0: Data available for CPU to read
const TX_READY_BIT: u8 = 0b0000_0010; // Bit 1: Console ready for CPU to send data

// Control Register Bits
const RX_INTERRUPT_ENABLE_BIT: u8 = 0b0000_0001; // Bit 0: Enable RX_READY interrupt
const TX_INTERRUPT_ENABLE_BIT: u8 = 0b0000_0010; // Bit 1: Enable TX_READY interrupt (optional for simple console)


#[derive(Debug)]
pub struct CupanaConsole {
    base_address: u16,
    input_buffer: VecDeque<u8>,
    // output_buffer: VecDeque<u8>, // Already present in your file for host output

    // --- Control Register State ---
    rx_interrupt_enabled: bool,
    tx_interrupt_enabled: bool, // For TX interrupt

    // --- Interrupt simulation state ---
    ticks_until_input_event: u32,
    // No longer need 'input_interrupt_pending' as check_interrupt will use status and enable flags
}

impl CupanaConsole {
    pub fn new(base_address: u16) -> Self {
        Self {
            base_address,
            input_buffer: VecDeque::new(),
            // output_buffer: VecDeque::new(), // From original file
            rx_interrupt_enabled: false, // RX interrupts disabled by default
            tx_interrupt_enabled: false, // TX interrupts disabled by default
            ticks_until_input_event: 15, // Simulate input after N "ticks" (calls to check_interrupt)
        }
    }

    // Simulates an external event (e.g., key press on host) putting data into the console's input buffer
    fn simulate_external_input_event(&mut self) {
        if self.input_buffer.len() < 4 { // Limit buffer size for simulation
            let simulated_char = b'S'; // Serial 'S'
            self.input_buffer.push_back(simulated_char);
            println!("[Console Device] Simulated SERIAL input: '{}' placed in buffer. RX_READY is now potentially set.", simulated_char as char);
        }
    }

    // Helper to get the current status register value
    fn get_status_register_value(&self) -> u8 {
        let mut status: u8 = 0;
        if !self.input_buffer.is_empty() {
            status |= RX_READY_BIT;
        }
        // For this simple console, assume it's always ready to transmit (print to host)
        status |= TX_READY_BIT;
        status
    }

    // Helper to get the current control register value
    fn get_control_register_value(&self) -> u8 {
        let mut control: u8 = 0;
        if self.rx_interrupt_enabled {
            control |= RX_INTERRUPT_ENABLE_BIT;
        }
        if self.tx_interrupt_enabled {
            control |= TX_INTERRUPT_ENABLE_BIT;
        }
        control
    }
}

impl Device for CupanaConsole {
    fn aabb(&self) -> (u16, u16) {
        (self.base_address, self.base_address + CONSOLE_DEVICE_SIZE)
    }

    fn read_u8(&mut self, addr_offset: u16) -> Result<u8, MemoryError> {
        match addr_offset {
            DATA_REGISTER_OFFSET => {
                if let Some(val) = self.input_buffer.pop_front() {
                    println!("[Console Device] CPU read data: '{}' (0x{:02X})", val as char, val);
                    // RX_READY bit will effectively be cleared in next status read if buffer is now empty.
                    // If an interrupt was pending for this data, the ISR should handle it.
                    Ok(val)
                } else {
                    Ok(0) // No data available, CPU reads 0.
                }
            }
            STATUS_REGISTER_OFFSET => {
                let status = self.get_status_register_value();
                Ok(status)
            }
            CONTROL_REGISTER_OFFSET => {
                Ok(self.get_control_register_value())
            }
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }

    fn write_u8(&mut self, addr_offset: u16, val: u8) -> Result<(), MemoryError> {
        match addr_offset {
            DATA_REGISTER_OFFSET => { // CPU sends data to console (output)
                print!("{}", val as char); // Directly print to host console
                // If we were managing TX_READY state more complexly (e.g. with a buffer),
                // we might set a TX interrupt pending flag here if tx_interrupt_enabled.
                Ok(())
            }
            CONTROL_REGISTER_OFFSET => { // CPU configures the console
                self.rx_interrupt_enabled = (val & RX_INTERRUPT_ENABLE_BIT) != 0;
                self.tx_interrupt_enabled = (val & TX_INTERRUPT_ENABLE_BIT) != 0;
                println!("[Console Device] Control Reg write: 0x{:02X}. RX_INT_EN: {}, TX_INT_EN: {}",
                         val, self.rx_interrupt_enabled, self.tx_interrupt_enabled);
                Ok(())
            }
            STATUS_REGISTER_OFFSET => {
                // Status register is typically read-only, or some bits might be clear-on-write.
                // For this example, we'll make it read-only.
                Err(MemoryError::WriteNotPermitted(self.base_address + addr_offset))
            }
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }

    fn check_interrupt(&mut self) -> bool {
        // Simulate an external event occurring based on ticks
        if self.ticks_until_input_event > 0 {
            self.ticks_until_input_event -= 1;
            if self.ticks_until_input_event == 0 {
                self.simulate_external_input_event();
                // Reset tick for next simulated event (optional, or make it a one-shot)
                self.ticks_until_input_event = 20; // trigger again later
            }
        }

        let status = self.get_status_register_value();
        let mut interrupt_signalled = false;

        // Check for RX Interrupt condition
        if self.rx_interrupt_enabled && (status & RX_READY_BIT) != 0 {
            // Data is available and RX interrupts are enabled.
            // For a level-triggered interrupt, as long as this condition holds,
            // the interrupt line would be active. The CPU's interrupt controller
            // and ISR are responsible for managing this.
            // Our `request_interrupt()` in CupanaMachine sets a flag that's usually
            // cleared once the ISR is entered. The ISR should read the data, which
            // should then make RX_READY_BIT go low (if buffer becomes empty).
            println!("[Console Device] RX Interrupt condition met. Signalling.");
            interrupt_signalled = true;
        }

        // Check for TX Interrupt condition (Optional for simple console)
        // This console is always ready to transmit (TX_READY_BIT is always 1).
        // A TX interrupt might be useful if you want to know when the CPU *can* send data.
        if self.tx_interrupt_enabled && (status & TX_READY_BIT) != 0 {
            // If we were to implement a scenario where TX_READY could be 0 (e.g. buffer full),
            // and then it becomes 1, that would be a clearer trigger for TX interrupt.
            // For now, if TX interrupts are enabled, it would signal constantly as TX_READY is always 1.
            // This is usually not desired. A TX interrupt often means "transmit buffer empty".
            // Let's comment this out for a more practical RX-focused example unless specifically needed.
            // println!("[Console Device] TX Interrupt condition met. Signalling.");
            // interrupt_signalled = true;
        }
        
        interrupt_signalled
    }
}