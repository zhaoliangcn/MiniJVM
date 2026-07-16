use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use crate::classfile::ClassFileParser;
use crate::error::{ClassFileError, JvmError, Result};
use crate::runtime::method_area::{Class, Method};

/// The type of class loader
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaderType {
    Bootstrap,
    Application,
}

/// A class loader in the parent-delegation model.
///
/// Hierarchy:
///   Application ClassLoader → Bootstrap ClassLoader
///
/// Bootstrap ClassLoader handles native/standard library classes.
/// Application ClassLoader loads `.class` files from the filesystem.
pub struct ClassLoader {
    loader_type: LoaderType,
    parent: Option<Box<ClassLoader>>,
    /// Class search paths for the Application ClassLoader
    class_paths: Vec<PathBuf>,
    /// Cache of loaded classes (class name -> Arc<Class>)
    loaded_classes: HashMap<String, Arc<Class>>,
}

impl ClassLoader {
    /// Create a new Bootstrap ClassLoader (no parent)
    pub fn new_bootstrap() -> Self {
        ClassLoader {
            loader_type: LoaderType::Bootstrap,
            parent: None,
            class_paths: vec![],
            loaded_classes: HashMap::new(),
        }
    }

    /// Create a new Application ClassLoader with Bootstrap as parent
    pub fn new_application(class_paths: Vec<PathBuf>) -> Self {
        ClassLoader {
            loader_type: LoaderType::Application,
            parent: Some(Box::new(ClassLoader::new_bootstrap())),
            class_paths,
            loaded_classes: HashMap::new(),
        }
    }

    /// Load a class by name using parent-delegation.
    ///
    /// 1. Check if already loaded in this loader or parent
    /// 2. Ask parent to load
    /// 3. If parent can't load, try to load it ourselves
    pub fn load_class(&mut self, class_name: &str) -> Result<Arc<Class>> {
        // 1. Check local cache
        if let Some(class) = self.loaded_classes.get(class_name) {
            return Ok(Arc::clone(class));
        }

        // 2. Delegate to parent
        if let Some(parent) = &mut self.parent {
            match parent.load_class(class_name) {
                Ok(class) => {
                    let class = Arc::clone(&class);
                    self.loaded_classes.insert(class_name.to_string(), Arc::clone(&class));
                    return Ok(class);
                }
                Err(_) => {
                    // Parent couldn't load, fall through to try ourselves
                }
            }
        }

        // 3. Try to load ourselves
        let class = self.load_class_internal(class_name)?;
        let class = Arc::new(class);
        self.loaded_classes.insert(class_name.to_string(), Arc::clone(&class));
        Ok(class)
    }

    /// Check if a class is already loaded (in any loader in the chain)
    pub fn is_loaded(&self, class_name: &str) -> bool {
        if self.loaded_classes.contains_key(class_name) {
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.is_loaded(class_name);
        }
        false
    }

    /// Get a loaded class by name (without trying to load)
    pub fn get_loaded(&self, class_name: &str) -> Option<&Arc<Class>> {
        if let Some(class) = self.loaded_classes.get(class_name) {
            return Some(class);
        }
        if let Some(parent) = &self.parent {
            return parent.get_loaded(class_name);
        }
        None
    }

    /// Get a mutable reference to a loaded class
    pub fn get_loaded_mut(&mut self, class_name: &str) -> Option<&mut Arc<Class>> {
        if self.loaded_classes.contains_key(class_name) {
            return self.loaded_classes.get_mut(class_name);
        }
        if let Some(parent) = &mut self.parent {
            return parent.get_loaded_mut(class_name);
        }
        None
    }

    /// Register a pre-built class (used for native/standard library classes)
    pub fn register_class(&mut self, class: Class) {
        let class_name = class.class_file.get_class_name().unwrap_or_default();
        self.loaded_classes.insert(class_name, Arc::new(class));
    }

    /// Internal: load a class from the filesystem
    fn load_class_internal(&mut self, class_name: &str) -> Result<Class> {
        match self.loader_type {
            LoaderType::Bootstrap => {
                // Bootstrap loader can't find filesystem classes
                Err(JvmError::ClassFileError(ClassFileError::ClassNotFound(class_name.to_string())))
            }
            LoaderType::Application => {
                let class_path = class_name.replace('.', "/");
                for path in &self.class_paths {
                    let file_path = path.join(format!("{}.class", class_path));
                    if file_path.exists() {
                        let data = fs::read(&file_path)?;
                        let mut parser = ClassFileParser::new(&data);
                        let class_file = parser.parse()?;
                        return Class::new(class_file);
                    }
                }
                // Also try the class name directly (for cases like "HelloWorld" -> "HelloWorld.class")
                for path in &self.class_paths {
                    let file_path = path.join(format!("{}.class", class_name));
                    if file_path.exists() {
                        let data = fs::read(&file_path)?;
                        let mut parser = ClassFileParser::new(&data);
                        let class_file = parser.parse()?;
                        return Class::new(class_file);
                    }
                }
                Err(JvmError::ClassFileError(ClassFileError::ClassNotFound(class_name.to_string())))
            }
        }
    }
}

impl std::fmt::Debug for ClassLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassLoader")
            .field("loader_type", &self.loader_type)
            .field("class_paths", &self.class_paths)
            .field("loaded_count", &self.loaded_classes.len())
            .finish()
    }
}