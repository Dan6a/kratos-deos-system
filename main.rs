// Kratos OS — Bare-Metal Entry Point
// RPi Zero 2W (BCM2710, Cortex-A53, AArch64)
//
// #![no_std]  — без стандартной библиотеки
// #![no_main] — без стандартного main, точка входа в start.s
//
// Аксиома 4: простота — минимум кода на пути к запуску

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod hal;

// Подключаем ассемблерную точку входа
core::arch::global_asm!(include_str!("asm/start.s"));

/// Обработчик паники — выводим сообщение и останавливаемся
/// Аксиома 8: система не молчит при смерти
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hal::uart::puts("\n[PANIC] ");
    if let Some(location) = info.location() {
        hal::uart::puts(location.file());
        hal::uart::puts(":");
        hal::uart::put_dec(location.line() as u64);
    }
    hal::uart::puts("\n");
    loop {
        unsafe { core::arch::asm!("wfe") }
    }
}

/// Главная функция — вызывается из start.s
/// После инициализации стека и BSS
#[no_mangle]
pub extern "C" fn kratos_main() -> ! {
    // ── Инициализация UART ────────────────────────────────────────────────
    hal::uart::init();

    // ── Приветствие ───────────────────────────────────────────────────────
    hal::uart::puts("\n");
    hal::uart::puts("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    hal::uart::puts("  Kratos OS — bare-metal\n");
    hal::uart::puts("  One node. All truth.\n");
    hal::uart::puts("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    hal::uart::puts("\n");

    // ── Проверка таймера ──────────────────────────────────────────────────
    hal::uart::puts("[hal] timer: ");
    let t0 = hal::timer::now_us();
    hal::timer::delay_ms(100);
    let t1 = hal::timer::now_us();
    hal::uart::put_dec(t1 - t0);
    hal::uart::puts(" us elapsed (expect ~100000)\n");

    // ── Основной цикл ─────────────────────────────────────────────────────
    hal::uart::puts("\n[kratos] kernel loop starting...\n");

    let mut tick: u64 = 0;
    loop {
        hal::timer::delay_ms(1000);
        tick += 1;

        hal::uart::puts("[tick] ");
        hal::uart::put_dec(tick);
        hal::uart::puts("\n");
    }
}
