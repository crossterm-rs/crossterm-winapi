# Version 0.5.1
- Make `Semaphore` implement `Clone`.

# Version 0.5.0 
- Add `Semaphore` object handling
- Make `ButtonState` more flexible.

# Version 0.4.0
- The `Handle` API has been reworked to make it `Send` + `Sync` and close the underlying `HANDLE` when dropped.

# Version 0.3.0

- Make read sync block for windows systems ([PR #2](https://github.com/crossterm-rs/crossterm-winapi/pull/2))

# Version 0.2.1

- Maintenance release only
- Moved to a [separate repository](https://github.com/crossterm-rs/crossterm-winapi)

# Version 0.2.0

- `Console::get_handle` to `Console::handle`
