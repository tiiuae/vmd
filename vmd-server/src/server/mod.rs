// === Server =================================================================
//
// This module encapsulates server logic for the virtual machine daemon. The
// `api` module contains the API definitions which connect the generated
// api-bindings to the server logic. The rest of the modules inside this
// module implement the server itself. The `api` module should call functions
// from the `vmd` module to perform the actual work.
//
// === Module declarations ====================================================

pub(crate) mod run;
pub(crate) mod cli;
mod tls;
mod util;
mod api;

// === EOF ====================================================================
