use std::io::{self, Write};

use serde::Serialize;

use crate::{Result, V2Error};

pub fn write_json_stdout<T: Serialize>(value: &T, pretty: bool) -> Result<()> {
    let mut bytes = if pretty {
        serde_json::to_vec_pretty(value)?
    } else {
        serde_json::to_vec(value)?
    };
    bytes.push(b'\n');
    write_machine_output(io::stdout().lock(), &bytes).map_err(V2Error::from)
}

fn write_machine_output(mut writer: impl Write, bytes: &[u8]) -> io::Result<()> {
    match writer.write_all(bytes) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};

    use super::write_machine_output;

    struct FailingWriter(io::ErrorKind);

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(self.0))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn broken_pipe_is_normal_downstream_termination() {
        write_machine_output(FailingWriter(io::ErrorKind::BrokenPipe), b"json")
            .expect("broken pipe is accepted");
    }

    #[test]
    fn unrelated_output_failures_remain_errors() {
        let error = write_machine_output(FailingWriter(io::ErrorKind::PermissionDenied), b"json")
            .expect_err("permission failure must propagate");
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    }
}
