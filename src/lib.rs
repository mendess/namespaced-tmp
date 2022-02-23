#![doc = include_str!("../README.md")]

use std::{io, path::PathBuf};

fn tmp_with_namespace(namespace: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(namespace);
    path
}

fn io_result_to_opt_error<T>(r: io::Result<T>) -> Option<io::Error> {
    match r {
        Ok(_) => None,
        Err(e) => Some(e),
    }
}

#[cfg(feature = "async")]
pub mod async_impl {
    use super::*;
    use std::{io, path::PathBuf};
    use tokio::fs::create_dir_all;

    /// Create a path to `filename` namespaced under `namespace`.
    ///
    /// Also creates the namespace directory.
    ///
    /// # Errors
    /// If creating the dir fails an error is returned alongside the path. Otherwise [None] is
    /// returned.
    pub async fn in_tmp(namespace: &str, filename: &str) -> (PathBuf, Option<io::Error>) {
        let mut path = tmp_with_namespace(namespace);
        let e = io_result_to_opt_error(create_dir_all(&path).await);
        path.push(filename);
        (path, e)
    }

    #[cfg(feature = "user")]
    /// Create a path to `filename` namespaced under the current user's name.
    ///
    /// Also creates the namespace directory.
    ///
    /// # Errors
    /// If creating the dir fails an error is returned alongside the path. Otherwise [None] is
    /// returned.
    pub async fn in_user_tmp(filename: &str) -> (PathBuf, Option<io::Error>) {
        in_tmp(&whoami::username(), filename).await
    }
}

pub mod blocking {
    use super::*;
    use std::{fs::create_dir_all, io, path::PathBuf};

    /// Create a path to `filename` namespaced under `namespace`.
    ///
    /// Also creates the namespace directory.
    ///
    /// # Errors
    /// If creating the dir fails an error is returned alongside the path. Otherwise [None] is
    /// returned.
    pub fn in_tmp(namespace: &str, filename: &str) -> (PathBuf, Option<io::Error>) {
        let mut path = tmp_with_namespace(namespace);
        let e = io_result_to_opt_error(create_dir_all(&path));
        path.push(filename);
        (path, e)
    }

    #[cfg(feature = "user")]
    /// Create a path to `filename` namespaced under the current user's name.
    ///
    /// Also creates the namespace directory.
    ///
    /// # Errors
    /// If creating the dir fails an error is returned alongside the path. Otherwise [None] is
    /// returned.
    pub fn in_user_tmp(filename: &str) -> (PathBuf, Option<io::Error>) {
        in_tmp(&whoami::username(), filename)
    }
}
