#[doc = r" The RTIC application module"] pub mod app
{
    #[doc =
    r" Always include the device crate which contains the vector table"] use
    rp2040_hal :: pac as
    you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml; use core ::
    fmt :: Write; use fugit :: RateExtU32; use hal :: Clock; use rp2040_hal as
    hal; #[doc = r" User code from within the module"]
    #[doc =
    " External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust"]
    #[doc = " if your board has a different frequency"] const XTAL_FREQ_HZ :
    u32 = 12_000_000u32; const SAMPLE_COUNT : usize = 1000; type Uart = hal ::
    uart :: UartPeripheral < hal :: uart :: Enabled, hal :: pac :: UART0,
    (hal :: gpio :: Pin < hal :: gpio :: bank0 :: Gpio0, hal :: gpio ::
    FunctionUart, hal :: gpio :: PullDown > , hal :: gpio :: Pin < hal :: gpio
    :: bank0 :: Gpio1, hal :: gpio :: FunctionUart, hal :: gpio :: PullDown >
    ,), > ; #[doc = r" User code end"] #[doc = " User provided init function"]
    #[inline(always)] #[allow(non_snake_case)] fn init(c : init :: Context) ->
    (Shared, Local, init :: Monotonics)
    {
        unsafe { hal :: sio :: spinlock_reset(); } let mut resets =
        c.device.RESETS; let mut watchdog = hal :: Watchdog ::
        new(c.device.WATCHDOG); let clocks = hal :: clocks ::
        init_clocks_and_plls(XTAL_FREQ_HZ, c.device.XOSC, c.device.CLOCKS,
        c.device.PLL_SYS, c.device.PLL_USB, & mut resets, & mut
        watchdog,).unwrap(); let sio = hal :: Sio :: new(c.device.SIO); let
        pins = hal :: gpio :: Pins ::
        new(c.device.IO_BANK0, c.device.PADS_BANK0, sio.gpio_bank0, & mut
        resets,); let uart_pins =
        (pins.gpio0.into_function :: < hal :: gpio :: FunctionUart > (),
        pins.gpio1.into_function :: < hal :: gpio :: FunctionUart > (),); let
        uart = hal :: uart :: UartPeripheral ::
        new(c.device.UART0, uart_pins, & mut
        resets).enable(hal :: uart :: UartConfig ::
        new(115200.Hz(), hal :: uart :: DataBits :: Eight, None, hal :: uart
        :: StopBits :: One,), clocks.peripheral_clock.freq(),).unwrap(); *
        c.local.adc = Some(hal :: Adc :: new(c.device.ADC, & mut resets)); let
        adc = c.local.adc.as_mut().unwrap(); let mut adc_pin_0 = hal :: adc ::
        AdcPin :: new(pins.gpio26.into_floating_input()).unwrap();
        uart.write_full_blocking(b"ADC FIFO interrupt example\r\n"); let
        adc_fifo =
        adc.build_fifo().clock_divider(47999,
        0).set_channel(& mut adc_pin_0).enable_interrupt(1).start();
        (Shared { done : false, buf : [0; SAMPLE_COUNT], uart, }, Local
        { adc_fifo : Some(adc_fifo), }, init :: Monotonics(),)
    } #[doc = " User provided idle function"] #[allow(non_snake_case)] fn
    idle(mut c : idle :: Context) -> !
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; loop
        {
            let finished =
            (& mut c.shared.done, & mut c.shared.buf, & mut
            c.shared.uart).lock(| done, buf, uart |
            {
                if * done
                {
                    for sample in buf
                    { writeln! (uart, "Sample: {sample}\r").unwrap(); } writeln!
                    (uart, "All done, going to sleep 😴\r").unwrap(); true
                } else { false }
            },); if finished { break; }
        } #[allow(clippy::empty_loop)] loop {}
    } #[doc = " User HW task: adc_irq_fifo"] #[allow(non_snake_case)] fn
    adc_irq_fifo(mut c : adc_irq_fifo :: Context)
    {
        use rtic :: Mutex as _; use rtic :: mutex :: prelude :: * ; if let
        Ok(sample) = c.local.adc_fifo.as_mut().unwrap().read()
        {
            let i = * c.local.counter;
            c.shared.buf.lock(| buf | buf [i] = sample); * c.local.counter +=
            1; if * c.local.counter == SAMPLE_COUNT
            {
                c.local.adc_fifo.take().unwrap().stop();
                c.shared.done.lock(| done | * done = true);
            }
        }
    } #[doc = " RTIC shared resource struct"] struct Shared
    { done : bool, buf : [u16; SAMPLE_COUNT], uart : Uart, }
    #[doc = " RTIC local resource struct"] struct Local
    { adc_fifo : Option < hal :: adc :: AdcFifo < 'static, u16 > > , }
    #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = " Local resources `init` has access to"] pub struct
    __rtic_internal_initLocalResources < >
    {
        #[doc = " Local resource `adc`"] pub adc : & 'static mut Option < hal
        :: Adc > ,
    } #[doc = r" Monotonics used by the system"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_Monotonics();
    #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_init_Context <
    'a >
    {
        #[doc = r" Core (Cortex-M) peripherals"] pub core : rtic :: export ::
        Peripherals, #[doc = r" Device peripherals"] pub device : rp2040_hal
        :: pac :: Peripherals, #[doc = r" Critical section token for init"]
        pub cs : rtic :: export :: CriticalSection < 'a > ,
        #[doc = r" Local Resources this task has access to"] pub local : init
        :: LocalResources < > ,
    } impl < 'a > __rtic_internal_init_Context < 'a >
    {
        #[doc(hidden)] #[inline(always)] pub unsafe fn
        new(core : rtic :: export :: Peripherals,) -> Self
        {
            __rtic_internal_init_Context
            {
                device : rp2040_hal :: pac :: Peripherals :: steal(), cs :
                rtic :: export :: CriticalSection :: new(), core, local : init
                :: LocalResources :: new(),
            }
        }
    } #[allow(non_snake_case)] #[doc = " Initialization function"] pub mod
    init
    {
        #[doc(inline)] pub use super :: __rtic_internal_initLocalResources as
        LocalResources; #[doc(inline)] pub use super ::
        __rtic_internal_Monotonics as Monotonics; #[doc(inline)] pub use super
        :: __rtic_internal_init_Context as Context;
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = " Shared resources `idle` has access to"] pub struct
    __rtic_internal_idleSharedResources < 'a >
    {
        #[doc =
        " Resource proxy resource `done`. Use method `.lock()` to gain access"]
        pub done : shared_resources :: done_that_needs_to_be_locked < 'a > ,
        #[doc =
        " Resource proxy resource `buf`. Use method `.lock()` to gain access"]
        pub buf : shared_resources :: buf_that_needs_to_be_locked < 'a > ,
        #[doc =
        " Resource proxy resource `uart`. Use method `.lock()` to gain access"]
        pub uart : shared_resources :: uart_that_needs_to_be_locked < 'a > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct __rtic_internal_idle_Context <
    'a >
    {
        #[doc = r" Shared Resources this task has access to"] pub shared :
        idle :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_idle_Context < 'a >
    {
        #[doc(hidden)] #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idle_Context
            { shared : idle :: SharedResources :: new(priority), }
        }
    } #[allow(non_snake_case)] #[doc = " Idle loop"] pub mod idle
    {
        #[doc(inline)] pub use super :: __rtic_internal_idleSharedResources as
        SharedResources; #[doc(inline)] pub use super ::
        __rtic_internal_idle_Context as Context;
    } mod shared_resources
    {
        use rtic :: export :: Priority; #[doc(hidden)]
        #[allow(non_camel_case_types)] pub struct done_that_needs_to_be_locked
        < 'a > { priority : & 'a Priority, } impl < 'a >
        done_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { done_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        buf_that_needs_to_be_locked < 'a > { priority : & 'a Priority, } impl
        < 'a > buf_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { buf_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        } #[doc(hidden)] #[allow(non_camel_case_types)] pub struct
        uart_that_needs_to_be_locked < 'a > { priority : & 'a Priority, } impl
        < 'a > uart_that_needs_to_be_locked < 'a >
        {
            #[inline(always)] pub unsafe fn new(priority : & 'a Priority) ->
            Self { uart_that_needs_to_be_locked { priority } }
            #[inline(always)] pub unsafe fn priority(& self) -> & Priority
            { self.priority }
        }
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = " Local resources `adc_irq_fifo` has access to"] pub struct
    __rtic_internal_adc_irq_fifoLocalResources < 'a >
    {
        #[doc = " Local resource `adc_fifo`"] pub adc_fifo : & 'a mut Option <
        hal :: adc :: AdcFifo < 'static, u16 > > ,
        #[doc = " Local resource `counter`"] pub counter : & 'a mut usize,
    } #[allow(non_snake_case)] #[allow(non_camel_case_types)]
    #[doc = " Shared resources `adc_irq_fifo` has access to"] pub struct
    __rtic_internal_adc_irq_fifoSharedResources < 'a >
    {
        #[doc =
        " Resource proxy resource `done`. Use method `.lock()` to gain access"]
        pub done : shared_resources :: done_that_needs_to_be_locked < 'a > ,
        #[doc =
        " Resource proxy resource `buf`. Use method `.lock()` to gain access"]
        pub buf : shared_resources :: buf_that_needs_to_be_locked < 'a > ,
    } #[doc = r" Execution context"] #[allow(non_snake_case)]
    #[allow(non_camel_case_types)] pub struct
    __rtic_internal_adc_irq_fifo_Context < 'a >
    {
        #[doc = r" Local Resources this task has access to"] pub local :
        adc_irq_fifo :: LocalResources < 'a > ,
        #[doc = r" Shared Resources this task has access to"] pub shared :
        adc_irq_fifo :: SharedResources < 'a > ,
    } impl < 'a > __rtic_internal_adc_irq_fifo_Context < 'a >
    {
        #[doc(hidden)] #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_adc_irq_fifo_Context
            {
                local : adc_irq_fifo :: LocalResources :: new(), shared :
                adc_irq_fifo :: SharedResources :: new(priority),
            }
        }
    } #[allow(non_snake_case)] #[doc = " Hardware task"] pub mod adc_irq_fifo
    {
        #[doc(inline)] pub use super ::
        __rtic_internal_adc_irq_fifoLocalResources as LocalResources;
        #[doc(inline)] pub use super ::
        __rtic_internal_adc_irq_fifoSharedResources as SharedResources;
        #[doc(inline)] pub use super :: __rtic_internal_adc_irq_fifo_Context
        as Context;
    } #[doc = r" App module"] impl < > __rtic_internal_initLocalResources < >
    {
        #[inline(always)] #[doc(hidden)] pub unsafe fn new() -> Self
        {
            __rtic_internal_initLocalResources
            { adc : & mut * __rtic_internal_local_init_adc.get_mut(), }
        }
    } impl < 'a > __rtic_internal_idleSharedResources < 'a >
    {
        #[doc(hidden)] #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_idleSharedResources
            {
                #[doc(hidden)] done : shared_resources ::
                done_that_needs_to_be_locked :: new(priority), #[doc(hidden)]
                buf : shared_resources :: buf_that_needs_to_be_locked ::
                new(priority), #[doc(hidden)] uart : shared_resources ::
                uart_that_needs_to_be_locked :: new(priority),
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic0"] static
    __rtic_internal_shared_resource_done : rtic :: RacyCell < core :: mem ::
    MaybeUninit < bool >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: done_that_needs_to_be_locked < 'a >
    {
        type T = bool; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut bool) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_done.get_mut() as * mut
                _, self.priority(), CEILING, rp2040_hal :: pac ::
                NVIC_PRIO_BITS, & __rtic_internal_MASKS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic1"] static
    __rtic_internal_shared_resource_buf : rtic :: RacyCell < core :: mem ::
    MaybeUninit < [u16; SAMPLE_COUNT] >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: buf_that_needs_to_be_locked < 'a >
    {
        type T = [u16; SAMPLE_COUNT]; #[inline(always)] fn lock <
        RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut [u16; SAMPLE_COUNT]) ->
        RTIC_INTERNAL_R) -> RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 1u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_buf.get_mut() as * mut _,
                self.priority(), CEILING, rp2040_hal :: pac :: NVIC_PRIO_BITS,
                & __rtic_internal_MASKS, f,)
            }
        }
    } #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic2"] static
    __rtic_internal_shared_resource_uart : rtic :: RacyCell < core :: mem ::
    MaybeUninit < Uart >> = rtic :: RacyCell ::
    new(core :: mem :: MaybeUninit :: uninit()); impl < 'a > rtic :: Mutex for
    shared_resources :: uart_that_needs_to_be_locked < 'a >
    {
        type T = Uart; #[inline(always)] fn lock < RTIC_INTERNAL_R >
        (& mut self, f : impl FnOnce(& mut Uart) -> RTIC_INTERNAL_R) ->
        RTIC_INTERNAL_R
        {
            #[doc = r" Priority ceiling"] const CEILING : u8 = 0u8; unsafe
            {
                rtic :: export ::
                lock(__rtic_internal_shared_resource_uart.get_mut() as * mut
                _, self.priority(), CEILING, rp2040_hal :: pac ::
                NVIC_PRIO_BITS, & __rtic_internal_MASKS, f,)
            }
        }
    } #[doc(hidden)] #[allow(non_upper_case_globals)] const
    __rtic_internal_MASK_CHUNKS : usize = rtic :: export ::
    compute_mask_chunks([rp2040_hal :: pac :: Interrupt :: ADC_IRQ_FIFO as
    u32]); #[doc(hidden)] #[allow(non_upper_case_globals)] const
    __rtic_internal_MASKS :
    [rtic :: export :: Mask < __rtic_internal_MASK_CHUNKS > ; 3] =
    [rtic :: export ::
    create_mask([rp2040_hal :: pac :: Interrupt :: ADC_IRQ_FIFO as u32]), rtic
    :: export :: create_mask([]), rtic :: export :: create_mask([])];
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] #[link_section = ".uninit.rtic3"] static
    __rtic_internal_local_resource_adc_fifo : rtic :: RacyCell < core :: mem
    :: MaybeUninit < Option < hal :: adc :: AdcFifo < 'static, u16 > > >> =
    rtic :: RacyCell :: new(core :: mem :: MaybeUninit :: uninit());
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_init_adc : rtic :: RacyCell <
    Option < hal :: Adc > > = rtic :: RacyCell :: new(None);
    #[allow(non_camel_case_types)] #[allow(non_upper_case_globals)]
    #[doc(hidden)] static __rtic_internal_local_adc_irq_fifo_counter : rtic ::
    RacyCell < usize > = rtic :: RacyCell :: new(0); #[allow(non_snake_case)]
    #[no_mangle] #[doc = " User HW task ISR trampoline for adc_irq_fifo"]
    unsafe fn ADC_IRQ_FIFO()
    {
        const PRIORITY : u8 = 1u8; rtic :: export ::
        run(PRIORITY, ||
        {
            adc_irq_fifo(adc_irq_fifo :: Context ::
            new(& rtic :: export :: Priority :: new(PRIORITY)))
        });
    } impl < 'a > __rtic_internal_adc_irq_fifoLocalResources < 'a >
    {
        #[inline(always)] #[doc(hidden)] pub unsafe fn new() -> Self
        {
            __rtic_internal_adc_irq_fifoLocalResources
            {
                adc_fifo : & mut *
                (& mut *
                __rtic_internal_local_resource_adc_fifo.get_mut()).as_mut_ptr(),
                counter : & mut *
                __rtic_internal_local_adc_irq_fifo_counter.get_mut(),
            }
        }
    } impl < 'a > __rtic_internal_adc_irq_fifoSharedResources < 'a >
    {
        #[doc(hidden)] #[inline(always)] pub unsafe fn
        new(priority : & 'a rtic :: export :: Priority) -> Self
        {
            __rtic_internal_adc_irq_fifoSharedResources
            {
                #[doc(hidden)] done : shared_resources ::
                done_that_needs_to_be_locked :: new(priority), #[doc(hidden)]
                buf : shared_resources :: buf_that_needs_to_be_locked ::
                new(priority),
            }
        }
    } #[doc(hidden)] mod rtic_ext
    {
        use super :: * ; #[no_mangle] unsafe extern "C" fn main() -> !
        {
            rtic :: export :: assert_send :: < bool > (); rtic :: export ::
            assert_send :: < [u16; SAMPLE_COUNT] > (); rtic :: export ::
            assert_send :: < Option < hal :: adc :: AdcFifo < 'static, u16 > >
            > (); const _CONST_CHECK : () =
            {
                if ! rtic :: export :: have_basepri()
                {
                    if (rp2040_hal :: pac :: Interrupt :: ADC_IRQ_FIFO as usize)
                    >= (__rtic_internal_MASK_CHUNKS * 32)
                    {
                        :: core :: panic!
                        ("An interrupt out of range is used while in armv6 or armv8m.base");
                    }
                } else {}
            }; let _ = _CONST_CHECK; rtic :: export :: interrupt :: disable();
            let mut core : rtic :: export :: Peripherals = rtic :: export ::
            Peripherals :: steal().into(); const _ : () = if
            (1 << rp2040_hal :: pac :: NVIC_PRIO_BITS) < 1u8 as usize
            {
                :: core :: panic!
                ("Maximum priority used by interrupt vector 'ADC_IRQ_FIFO' is more than supported by hardware");
            };
            core.NVIC.set_priority(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
            :: interrupt :: ADC_IRQ_FIFO, rtic :: export ::
            logical2hw(1u8, rp2040_hal :: pac :: NVIC_PRIO_BITS),); rtic ::
            export :: NVIC ::
            unmask(you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml
            :: interrupt :: ADC_IRQ_FIFO); #[inline(never)] fn
            __rtic_init_resources < F > (f : F) where F : FnOnce() { f(); }
            __rtic_init_resources(||
            {
                let (shared_resources, local_resources, mut monotonics) =
                init(init :: Context :: new(core.into()));
                __rtic_internal_shared_resource_done.get_mut().write(core ::
                mem :: MaybeUninit :: new(shared_resources.done));
                __rtic_internal_shared_resource_buf.get_mut().write(core ::
                mem :: MaybeUninit :: new(shared_resources.buf));
                __rtic_internal_shared_resource_uart.get_mut().write(core ::
                mem :: MaybeUninit :: new(shared_resources.uart));
                __rtic_internal_local_resource_adc_fifo.get_mut().write(core
                :: mem :: MaybeUninit :: new(local_resources.adc_fifo)); rtic
                :: export :: interrupt :: enable();
            });
            idle(idle :: Context ::
            new(& rtic :: export :: Priority :: new(0)))
        }
    }
}