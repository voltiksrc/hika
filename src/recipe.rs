use mlua::{Lua, Table};
use std::{cell::RefCell, error::Error, fs, path::Path, rc::Rc};

pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub artifacts: Vec<Artifact>,
    pub runtime_deps: Vec<String>,
    // pub build_deps: Vec<String>, Once we have source implemented
}

pub struct Artifact {
    pub urls: Vec<String>,
    pub sha256: String,
}

pub fn load_recipe(path: &Path) -> Result<Package, Box<dyn Error>> {
    let recipe = fs::read_to_string(path)?;

    // Create a new lua interpreter
    let lua = Lua::new();

    // We'll let pkg() write the parsed package here,
    // then read it back after the lua file finishes running.
    let parsed_pkg = Rc::new(RefCell::new(None::<Package>));
    let parsed_pkg_ref = Rc::clone(&parsed_pkg);

    // Define our main pkg() function as a rust function that lua can call
    let pkg_fn = lua.create_function(move |_, table: Table| {
        let name: String = table.get("name")?;
        let version: String = table.get("version")?;
        let description: String = table.get("description")?;

        let artifacts_table: Table = table.get("artifacts")?;
        let mut artifacts = Vec::new();

        for artifact_result in artifacts_table.sequence_values::<Table>() {
            let artifact_table = artifact_result?;

            // Get our urls table in the recipe so we can loop through them
            let urls_table: Table = artifact_table.get("urls")?;
            let mut urls = Vec::new();

            for url_result in urls_table.sequence_values::<String>() {
                urls.push(url_result?);
            }

            let sha256: String = artifact_table.get("sha256")?;

            artifacts.push(Artifact { urls, sha256 });
        }

        // Eventually pass this into my DFS resolver (when I make one)
        let runtime_deps = match table.get::<Table>("runtime_deps") {
            Ok(deps_table) => {
                let mut deps = Vec::new();
                for dep_result in deps_table.sequence_values::<String>() {
                    deps.push(dep_result?);
                }
                deps
            }
            Err(_) => Vec::new(),
        };

        let package = Package {
            name,
            version,
            description,
            artifacts,
            runtime_deps,
        };

        *parsed_pkg_ref.borrow_mut() = Some(package);

        Ok(())
    })?;

    // Register pkg as a lua global and execute the recipe
    lua.globals().set("pkg", pkg_fn)?;

    lua.load(&recipe).exec()?;

    parsed_pkg
        .borrow_mut()
        .take()
        .ok_or_else(|| "recipe did not call pkg { ... }".into())
}
