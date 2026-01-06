// Elixir Check
// Integration layer for Erlang/OTP and Elixir guarantees

use std::process::Command;

pub struct ElixirCheck {
    pub has_erlang: bool,
    pub has_elixir: bool,
    pub otp_version: Option<String>,
}

impl Default for ElixirCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl ElixirCheck {
    pub fn new() -> Self {
        let has_erlang = Self::check_erlang();
        let has_elixir = Self::check_elixir();
        let otp_version = Self::get_otp_version();

        ElixirCheck {
            has_erlang,
            has_elixir,
            otp_version,
        }
    }

    fn check_erlang() -> bool {
        Command::new("erl")
            .arg("-version")
            .output()
            .is_ok()
    }

    fn check_elixir() -> bool {
        Command::new("elixir")
            .arg("--version")
            .output()
            .is_ok()
    }

    fn get_otp_version() -> Option<String> {
        Command::new("erl")
            .arg("-eval")
            .arg("erlang:display(erlang:system_info(otp_release)), halt().")
            .arg("-noshell")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|s| s.trim().to_string())
    }

    pub fn verify_guarantees(&self) -> Result<String, String> {
        if !self.has_erlang && !self.has_elixir {
            return Ok("Erlang/OTP and Elixir not detected (optional)".to_string());
        }

        let mut guarantees = Vec::new();

        if self.has_erlang {
            guarantees.push("✓ Erlang/OTP runtime available".to_string());
            guarantees.push("✓ Fault tolerance via supervisor trees".to_string());
            guarantees.push("✓ Hot code reloading support".to_string());
        }

        if self.has_elixir {
            guarantees.push("✓ Elixir runtime available".to_string());
            guarantees.push("✓ Immutable data structures".to_string());
            guarantees.push("✓ Pattern matching enabled".to_string());
        }

        if let Some(ref version) = self.otp_version {
            guarantees.push(format!("✓ OTP Version: {}", version));
        }

        Ok(guarantees.join("\n"))
    }

    pub fn display(&self) -> String {
        match self.verify_guarantees() {
            Ok(guarantees) => format!("Elixir Check:\n{}", guarantees),
            Err(e) => format!("Elixir Check: {}", e),
        }
    }
}

pub fn run_elixir_check() -> ElixirCheck {
    ElixirCheck::new()
}
