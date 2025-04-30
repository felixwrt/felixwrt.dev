/// std::io::Write
pub trait Write {
    /// Writes a formatted string into this writer, returning any error encountered.
    ///
    /// This method is primarily used to interface with the [`format_args!()`] macro, and it is rare that this should explicitly be called. The [`write!()`] macro should be favored to invoke this method instead.
    ///
    /// This function internally uses the [`write_all`] method on this trait and hence will continuously write data so long as no errors are received. This also means that partial writes are not indicated in this signature.
    ///
    /// [`write_all`]: Write::write_all
    ///
    /// # Errors
    ///
    /// This function will return any I/O error reported while formatting.
    ///
    /// ...
    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<()> {
        // Create a shim which translates a Write to a fmt::Write and saves off I/O errors. instead of discarding them
        struct Adapter<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Result<()>,
        }

        impl<T: Write + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter { inner: self, error: Ok(()) };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    // This shouldn't happen: the underlying stream did not error, but somehow the formatter still errored?
                    panic!(
                        "a formatting trait implementation returned an error when the underlying stream did not"
                    );
                }
            }
        }
    }
}