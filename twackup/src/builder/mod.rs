/*
 * Copyright 2020 DanP
 *
 * This file is part of Twackup
 *
 * Twackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Twackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Twackup. If not, see <http://www.gnu.org/licenses/>.
 */

pub mod deb;

use crate::error::Generic;
use crate::{error::Result, package::Package, progress::Progress};
use deb::{Deb, DebianTarArchive};
use std::{
    fs, io,
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Preferences {
    pub remove_deb: bool,
    pub admin_dir: PathBuf,
    pub destination: PathBuf,
    pub compression_level: u32,
}

/// Creates DEB from filesystem contents
pub struct Worker<T: Progress> {
    pub package: Package,
    pub progress: T,
    pub archive: Option<Arc<Mutex<DebianTarArchive>>>,
    pub preferences: Preferences,
    pub working_dir: PathBuf,
}

impl Preferences {
    pub fn new<A: AsRef<Path>, D: AsRef<Path>>(admin_dir: A, destination: D) -> Self {
        Self {
            remove_deb: true,
            admin_dir: admin_dir.as_ref().to_path_buf(),
            destination: destination.as_ref().to_path_buf(),
            compression_level: 0,
        }
    }
}

impl<T: Progress> Worker<T> {
    pub fn new(
        package: Package,
        progress: T,
        archive: Option<Arc<Mutex<DebianTarArchive>>>,
        preferences: Preferences,
    ) -> Self {
        let name = package.canonical_name();
        let working_dir = preferences.destination.join(name);

        Self {
            package,
            progress,
            archive,
            preferences,
            working_dir,
        }
    }

    /// Runs worker
    ///
    /// # Errors
    /// Returns error if temp dir creation or any of underlying package operation failed
    pub fn run(&self) -> io::Result<PathBuf> {
        let w_dir = &self.working_dir;

        // Removing all dir contents
        fs::remove_dir_all(w_dir).ok();
        fs::create_dir(w_dir)?;

        let deb_name = format!("{}.deb", self.package.canonical_name());
        let deb_path = self.preferences.destination.join(deb_name);

        let mut deb = Deb::new(w_dir, &deb_path, self.preferences.compression_level)?;
        self.archive_files(deb.data_mut_ref())?;
        self.archive_metadata(deb.control_mut_ref())?;
        deb.package()?;

        fs::remove_dir_all(w_dir).ok();

        Ok(deb_path)
    }

    /// Archives package files and compresses in a single archive
    ///
    /// # Errros
    /// Returns error if dpkg directory couldn't be read or any of underlying operation failed
    fn archive_files(&self, archiver: &mut DebianTarArchive) -> io::Result<()> {
        let files = self
            .package
            .get_installed_files(&self.preferences.admin_dir)?;

        for file in files {
            // Remove root slash because tars don't contain absolute paths
            let name = file.trim_start_matches('/');
            let res = archiver.get_mut().append_path_with_name(&file, name);
            if let Err(error) = res {
                log::warn!("[{}] {}", self.package.id, error);
            }
        }

        Ok(())
    }

    /// Collects package metadata such as install scripts,
    /// creates control and packages all this together
    ///
    /// # Errors
    /// Returns error if dpkg directory couldn't be read or any of underlying operation failed
    fn archive_metadata(&self, archiver: &mut DebianTarArchive) -> io::Result<()> {
        // Order in this archive doesn't matter. So we'll add control at first
        archiver.append_new_file("control", self.package.to_control().as_bytes())?;

        // Then add every matching metadata file in dpkg database dir
        let pid_len = self.package.id.len();
        let package_id = self.package.id.as_bytes();
        for entry in fs::read_dir(self.preferences.admin_dir.join("info"))?.flatten() {
            let file_name = entry.file_name().into_vec();

            // Reject every file not starting with package id + dot
            if file_name.len() <= pid_len + 1 {
                continue;
            }
            if &file_name[..pid_len] != package_id || file_name[pid_len] != b'.' {
                continue;
            }
            let ext = unsafe { std::str::from_utf8_unchecked(&file_name[pid_len + 1..]) };
            // And skip .list file as it contains package files list
            if ext == "list" {
                continue;
            }

            let res = archiver.get_mut().append_path_with_name(entry.path(), ext);
            if let Err(error) = res {
                log::warn!("[{}] {}", self.package.id, error);
            }
        }

        Ok(())
    }

    /// Creates package debian archive and optionally add it to shared TAR archive
    ///
    /// # Errors
    /// Returns error if any of underlying operations failed
    pub fn work(&self) -> Result<()> {
        let progress = format!("Processing {}", self.package.human_name());
        self.progress.set_message(progress);

        let file = self.run()?;
        self.progress.increment(1);

        let progress = format!("Done {}", self.package.human_name());
        self.progress.set_message(progress);

        self.add_to_archive(file)?;

        Ok(())
    }

    /// Adds already assembled package to common TAR archive
    ///
    /// # Errors
    /// Returns error if any of underlying operations failed
    fn add_to_archive(&self, file: PathBuf) -> Result<()> {
        if let Some(archive) = &self.archive {
            let mut archive = archive.try_lock().map_err(|_| Generic::LockError)?;

            let abs_file = Path::new(".").join(file.file_name().unwrap());
            archive.get_mut().append_path_with_name(&file, &abs_file)?;

            if self.preferences.remove_deb {
                fs::remove_file(file).ok();
            }
        }

        Ok(())
    }
}
