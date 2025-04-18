use std::{path::PathBuf, sync::RwLock};

use crate::Concatenate;

use super::{ASSETS_PATH, fs::FileSystem};
use assets_manager::{
    AnyCache, AssetCache, BoxedError, Compound, Storable,
    asset::DirLoadable,
    hot_reloading::EventSender,
    source::{FileContent, Source, Tar},
};

struct PluginEntry {
    path: PathBuf,
    cache: AssetCache<Tar>,
}

/// The location of this asset
enum AssetSource {
    FileSystem,
    Plugin { index: usize },
}

struct SourceAndContents<'a>(AssetSource, FileContent<'a>);

/// This source combines assets loaded from the filesystem and from plugins.
/// It is typically used via the CombinedCache type.
///
/// A load will search through all sources and warn about unhandled duplicates.
pub struct CombinedSource {
    fs: AssetCache<FileSystem>,
    plugin_list: RwLock<Vec<PluginEntry>>,
}

impl CombinedSource {
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            fs: AssetCache::with_source(FileSystem::new()?),
            plugin_list: RwLock::new(Vec::new()),
        })
    }
}

impl CombinedSource {
    /// Look for an asset in all known sources
    fn read_multiple(&self, id: &str, ext: &str) -> Vec<SourceAndContents<'_>> {
        let mut result = Vec::new();
        if let Ok(file_entry) = self.fs.raw_source().read(id, ext) {
            result.push(SourceAndContents(AssetSource::FileSystem, file_entry));
        }
        for (n, p) in self.plugin_list.read().unwrap().iter().enumerate() {
            if let Ok(entry) = p.cache.raw_source().read(id, ext) {
                // the data is behind an RwLockReadGuard, so own it for returning
                result.push(SourceAndContents(
                    AssetSource::Plugin { index: n },
                    match entry {
                        FileContent::Slice(s) => FileContent::Buffer(Vec::from(s)),
                        FileContent::Buffer(b) => FileContent::Buffer(b),
                        FileContent::Owned(s) => {
                            FileContent::Buffer(Vec::from(s.as_ref().as_ref()))
                        },
                    },
                ));
            }
        }
        result
    }

    /// Return the path of a source
    fn plugin_path(&self, index: &AssetSource) -> Option<PathBuf> {
        match index {
            AssetSource::FileSystem => Some(ASSETS_PATH.clone()),
            AssetSource::Plugin { index } => self.plugin_list
                .read()
                .unwrap()
                .get(*index)
                // We don't want to keep the lock, so we clone
                .map(|plugin| plugin.path.clone()),
        }
    }
}

impl Source for CombinedSource {
    fn read(&self, id: &str, ext: &str) -> std::io::Result<FileContent<'_>> {
        // We could shortcut on fs if we dont check for conflicts
        let mut entries = self.read_multiple(id, ext);
        if entries.is_empty() {
            Err(std::io::ErrorKind::NotFound.into())
        } else {
            if entries.len() > 1 {
                let patha = self.plugin_path(&entries[0].0);
                let pathb = self.plugin_path(&entries[1].0);
                tracing::error!("Duplicate asset {id} in {patha:?} and {pathb:?}");
            }
            // unconditionally return the first asset found
            Ok(entries.swap_remove(0).1)
        }
    }

    fn read_dir(
        &self,
        id: &str,
        f: &mut dyn FnMut(assets_manager::source::DirEntry),
    ) -> std::io::Result<()> {
        // TODO: We should combine the sources, but this isn't used in coping chronicles
        self.fs.raw_source().read_dir(id, f)
    }

    fn exists(&self, entry: assets_manager::source::DirEntry) -> bool {
        self.fs.raw_source().exists(entry)
            || self
                .plugin_list
                .read()
                .unwrap()
                .iter()
                .any(|plugin| plugin.cache.raw_source().exists(entry))
    }

    // TODO: Enable hot reloading for plugins
    fn make_source(&self) -> Option<Box<dyn Source + Send>> { self.fs.raw_source().make_source() }

    fn configure_hot_reloading(&self, events: EventSender) -> Result<(), BoxedError> {
        self.fs.raw_source().configure_hot_reloading(events)
    }
}

/// A cache combining filesystem and plugin assets
pub struct CombinedCache(AssetCache<CombinedSource>);

impl CombinedCache {
    pub fn new() -> std::io::Result<Self> {
        CombinedSource::new().map(|combined_source| Self(AssetCache::with_source(combined_source)))
    }

    #[doc(hidden)]
    // Provide a cache to the "combine_static" functions as they omit
    // wrapping in a Compound (which enables hot-reload)
    pub(crate) fn non_reloading_cache(&self) -> AnyCache<'_> {
        self.0.raw_source().fs.as_any_cache()
    }

    /// Combine objects from filesystem and plugins
    pub fn combine<T: Concatenate>(
        &self,
        // this cache registers with hot reloading
        reloading_cache: AnyCache,
        mut load_from: impl FnMut(AnyCache) -> Result<T, assets_manager::Error>,
    ) -> Result<T, assets_manager::Error> {
        let mut result = load_from(reloading_cache);
        // Report a severe error from the filesystem asset even if later overwritten by
        // an Ok value from a plugin
        if let Err(ref fs_error) = result {
            match fs_error
                .reason()
                .downcast_ref::<std::io::Error>()
                .map(|io_error| io_error.kind())
            {
                Some(std::io::ErrorKind::NotFound) => (),
                _ => tracing::error!("Filesystem asset load {fs_error:?}"),
            }
        }
        for plugin in self.0.raw_source().plugin_list.read().unwrap().iter() {
            match load_from(plugin.cache.as_any_cache()) {
                Ok(b) => {
                    result = if let Ok(a) = result {
                        Ok(a.concatenate(b))
                    } else {
                        Ok(b)
                    };
                },
                // Report any error other than NotFound
                Err(plugin_error) => {
                    match plugin_error
                        .reason()
                        .downcast_ref::<std::io::Error>()
                        .map(|io_error| io_error.kind())
                    {
                        Some(std::io::ErrorKind::NotFound) => (),
                        _ => tracing::error!(
                            "Loading from {:?} failed {plugin_error:?}",
                            plugin.path
                        ),
                    }
                },
            }
        }
        result
    }

    /// Add a tar archive (a plugin) to the system.
    /// All files in that tar file become potential assets.
    pub fn register_tar(&self, path: PathBuf) -> std::io::Result<()> {
        let tar_source = Tar::open(&path)?;
        let cache = AssetCache::with_source(tar_source);
        self.0
            .raw_source()
            .plugin_list
            .write()
            .unwrap()
            .push(PluginEntry { path, cache });
        Ok(())
    }

    // Just forward these methods to the cache
    #[inline]
    #[cfg(feature = "hot-reloading")]
    pub fn enhance_hot_reloading(&'static self) { self.0.enhance_hot_reloading(); }

    #[inline]
    pub fn load_rec_dir<A: DirLoadable>(
        &self,
        id: &str,
    ) -> Result<&assets_manager::Handle<assets_manager::RecursiveDirectory<A>>, assets_manager::Error>
    {
        self.0.load_rec_dir(id)
    }

    #[inline]
    pub fn load<A: Compound>(
        &self,
        id: &str,
    ) -> Result<&assets_manager::Handle<A>, assets_manager::Error> {
        self.0.load(id)
    }

    #[inline]
    pub fn get_or_insert<A: Storable>(&self, id: &str, a: A) -> &assets_manager::Handle<A> {
        self.0.get_or_insert(id, a)
    }

    #[inline]
    pub fn load_owned<A: Compound>(&self, id: &str) -> Result<A, assets_manager::Error> {
        self.0.load_owned(id)
    }
}
