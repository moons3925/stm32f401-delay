#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m_rt::entry;
use cortex_m::delay;    // (1)Delayを使う
use stm32f4::stm32f401;

#[entry]
fn main() -> ! {

    let dp = stm32f401::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    clock_init(&dp);    // (2)クロック関連の初期化
    gpioa5_init(&dp);   // (3)GPIOA5の初期化
    let mut delay = delay::Delay::new(cp.SYST, 84000000_u32);   // (4)Delayの初期化

    loop {
        dp.GPIOA.odr.modify(|r, w| w.odr5().bit(r.odr5().is_low()));    // (5)GPIOAの出力を反転する
        delay.delay_ms(1000_u32);   // (6)1000msec遅延
    }
}

fn clock_init(dp: &stm32f401::Peripherals) {

    // PLLSRC = HSI: 16MHz (default)
    dp.RCC.pllcfgr.modify(|_, w| w.pllp().div4());      // P=4
    dp.RCC.pllcfgr.modify(|_, w| unsafe { w.plln().bits(336) });    // N=336
    // PLLM = 16 (default)

    dp.RCC.cfgr.modify(|_, w| w.ppre1().div2());        // APB1 PSC = 1/2
    dp.RCC.cr.modify(|_, w| w.pllon().on());            // PLL On
    while dp.RCC.cr.read().pllrdy().is_not_ready() {    // 安定するまで待つ
        // PLLがロックするまで待つ (PLLRDY)
    }

    // データシートのテーブル15より
    dp.FLASH.acr.modify(|_,w| w.latency().bits(2));    // レイテンシの設定: 2ウェイト

    dp.RCC.cfgr.modify(|_,w| w.sw().pll());     // sysclk = PLL
    while !dp.RCC.cfgr.read().sws().is_pll() {  // SWS システムクロックソースがPLLになるまで待つ
    }
}

fn gpioa5_init(dp: &stm32f401::Peripherals) {
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().enabled());    // GPIOAのクロックを有効にする
    dp.GPIOA.moder.modify(|_, w| w.moder5().output());   // GPIOA5を汎用出力に設定    
}
