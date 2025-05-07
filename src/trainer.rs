use libmem::*;
use windows_sys::{core::*, Win32::UI::WindowsAndMessaging::*};

pub struct TrainerStates {
    no_cool_down_state: bool,
    auto_collect_sun_state: bool,
    game_process: Process,
    game_module: Module,
}

impl TrainerStates {
    pub fn default() -> Self {
        let game_process = match find_process(crate::offset::GAME_PROCESS_NAME) {
            Some(game_process) => game_process,
            None => unsafe {
                MessageBoxW(
                    0 as _,
                    w!("Game process not found"),
                    w!("Trainer Error"),
                    MB_ICONERROR,
                );
                panic!("{}", "Game process not found".to_string());
            },
        };

        let game_module = match find_module_ex(&game_process, crate::offset::GAME_MODULE_NAME) {
            Some(game_module) => game_module,
            None => unsafe {
                MessageBoxW(
                    0 as _,
                    w!("Game module not found"),
                    w!("Trainer Error"),
                    MB_ICONERROR,
                );
                panic!("{}", "Game module not found".to_string());
            },
        };

        Self {
            no_cool_down_state: false,
            auto_collect_sun_state: false,
            game_process,
            game_module,
        }
    }

    pub fn no_cool_down(&mut self, toggle: bool) -> Result<bool, String> {
        if self.no_cool_down_state == toggle {
            return Ok(toggle);
        }
        let no_cool_down_address = self.game_module.base + crate::offset::NO_COOLDOWN_OFFSET;
        if toggle {
            // NOP out the instruction at the address
            if let Some(shellcode) = assemble_ex("nop; nop", Arch::X86, 0) {
                match write_memory_ex(
                    &self.game_process,
                    no_cool_down_address,
                    shellcode.as_slice(),
                ) {
                    Some(_) => {}
                    None => {
                        return Err("Failed to write memory".to_string());
                    }
                };
            } else {
                return Err("Failed to assemble shellcode".to_string());
            }
        } else {
            // Restore the original instruction at the address
            let original_shellcode: Vec<u8> = vec![0x74, 0x1F];
            match write_memory_ex(
                &self.game_process,
                no_cool_down_address,
                original_shellcode.as_slice(),
            ) {
                Some(_) => {}
                None => {
                    return Err("Failed to write memory".to_string());
                }
            };
        }

        self.no_cool_down_state = toggle;
        Ok(toggle)
    }

    pub fn auto_collect_sun(&mut self, toggle: bool) -> Result<bool, String> {
        if self.auto_collect_sun_state == toggle {
            return Ok(toggle);
        }

        let auto_collect_sun_address =
            self.game_module.base + crate::offset::AUTO_COLLECT_SUN_OFFSET;
        if toggle {
            let shellcode: Vec<u8> = vec![0x74, 0x09];
            match write_memory_ex(
                &self.game_process,
                auto_collect_sun_address,
                shellcode.as_slice(),
            ) {
                Some(_) => {}
                None => {
                    return Err("Failed to write memory".to_string());
                }
            };
        } else {
            // Restore the original instruction at the address
            let original_shellcode: Vec<u8> = vec![0x75, 0x09];
            match write_memory_ex(
                &self.game_process,
                auto_collect_sun_address,
                original_shellcode.as_slice(),
            ) {
                Some(_) => {}
                None => {
                    return Err("Failed to write memory".to_string());
                }
            };
        }

        self.auto_collect_sun_state = toggle;
        Ok(toggle)
    }

    pub fn edit_sun_value(&mut self, value: u32) -> Result<bool, String> {
        if let Some(sun_address) = deep_pointer_ex(
            &self.game_process,
            self.game_module.base + crate::offset::SUN_ADDR_BASE,
            &[
                crate::offset::SUN_ADDR_OFFSET_1,
                crate::offset::SUN_ADDR_OFFSET_2,
            ],
        ) {
            match write_memory_ex(&self.game_process, sun_address, &value) {
                Some(_) => {}
                None => {
                    return Err("Failed to write memory".to_string());
                }
            };
        } else {
            return Err("Failed to find sun address".to_string());
        }

        Ok(true)
    }

    pub fn edit_coin_value(&mut self, value: u32) -> Result<bool, String> {
        if let Some(coin_address) = deep_pointer_ex(
            &self.game_process,
            self.game_module.base + crate::offset::COIN_ADDR_BASE,
            &[
                crate::offset::COIN_ADDR_OFFSET_1,
                crate::offset::COIN_ADDR_OFFSET_2,
            ],
        ) {
            match write_memory_ex(&self.game_process, coin_address, &value) {
                Some(_) => {}
                None => {
                    return Err("Failed to write memory".to_string());
                }
            };
        } else {
            return Err("Failed to find coin address".to_string());
        }

        Ok(true)
    }
}
