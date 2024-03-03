//! Custom small crate for panicking behavior of all esp targets.
//!
//! The default behavior of all targets would be a simple reset in release mode or
//! a halt in debug mode. Some specific targets may want to execute custom code for
//! their panicking function, so it allows to do just that.

#![feature(custom_test_frameworks, used_with_arg)]
#![no_std]

#[cfg(not(feature = "custom_panic_code"))]
mod nofun {
    use core::panic::PanicInfo;
    use core::sync::atomic::{self, Ordering}; 

    /// Custom panic function
    ///
    /// If the custom function is not required, this panic handler will be used.
    /// In debug mode, function will only halt, while in release mode, microchip
    /// will be reset.
    #[inline(never)]
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        #[cfg(not(debug_assertions))]
        // Causes reset for all esp microchips.
        let _ = 0x0 as *const ();

        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }
}

#[cfg(feature = "custom_panic_code")]
mod fun {
    use core::panic::PanicInfo;
    use core::sync::atomic::{self, Ordering}; 

    #[link(name = "__pre_panic")]
    extern "C" {
        fn custom_panic_function();
    }

    /// Extern function that should be used before resetting.
    ///
    /// This functin is defined as "C" function only because this way it can be any type of
    /// regular function.
    #[used(linker)]
    static CUSTOM_PANIC_FUNCTION: unsafe extern "C" fn() = custom_panic_function;

    /// Custom panic function
    ///
    /// This panic function will execute custom user code first, before really panicking. It
    /// allows to do something with the hardware or software, before full reset. After the
    /// execution 
    #[inline(never)]
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        unsafe { (CUSTOM_PANIC_FUNCTION)(); }

        #[cfg(not(debug_assertions))]
        // Causes reset for all esp microchips.
        let _ = 0x0 as *const ();

        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }
}

