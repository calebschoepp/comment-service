use suborbital::req::*;
use suborbital::runnable::*;

struct ValidatePlatform {}

impl Runnable for ValidatePlatform {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        // Read url parameters
        let platform = url_param("platform");

        // Validate platform
        if let false = vec!["reddit", "hackernews"].iter().any(|p| platform == *p) {
            return Err(RunErr::new(400, "Provided platform is invalid"));
        }

        Ok(String::from(format!("{}", platform)).as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &ValidatePlatform = &ValidatePlatform {};

#[no_mangle]
pub extern "C" fn init() {
    use_runnable(RUNNABLE);
}
