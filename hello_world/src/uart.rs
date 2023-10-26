pub struct UART {
	base: *mut u8
}


impl UART {
	pub unsafe fn new(base: usize) -> Self {
	    let base = base as *mut u8;

	    use core::ptr::write_volatile;
	    unsafe {
	        // Set data size to 8 bits.
	        base.offset(3).write_volatile(0b11);
	        // Enable FIFO.
	        base.offset(2).write_volatile(0b1);
	        // Enable receiver buffer interrupts.
	        base.offset(1).write_volatile(0b1);
	     }

	     UART { base }
	}

	pub fn put_ch(&mut self, ch: u8) {
		unsafe { self.base.write_volatile(ch); }
	}
}
