#[macro_export]
macro_rules! on_state_change {
    ($expr:expr, $on_fail:expr, $on_recover:expr) => {{
        thread_local! {
            static LAST_CALL_SUCCEEDED: std::cell::Cell<bool> = std::cell::Cell::new(false);
        }
        let result = $expr;

        #[allow(unused_must_use, unused_results)]
        LAST_CALL_SUCCEEDED.with(|state| match result {
            Ok(val) => {
                if !state.get() {
                    $on_recover;
                }
                state.set(true);
                Ok(val)
            }
            Err(err) => {
                if state.get() {
                    $on_fail;
                }
                state.set(false);
                Err(err)
            }
        })
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {
        thread_local! {
            static LAST_CALL_SUCCEEDED: std::cell::Cell<bool> = std::cell::Cell::new(false);
        }

        let mut result_to_return = Ok(());
        for i in 0..5 {
            if i == 3 {
                result_to_return = Err(());
            }

            let _ = on_state_change!(
                result_to_return,
                eprintln!("It started failing"),
                eprintln!("It started working again")
            );
        }
    }
}
