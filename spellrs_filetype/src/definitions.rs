use crate::types::{FileTypeDefinition as F, FileTypeFormat};
use once_cell::sync::Lazy;

pub static DEFINITIONS: Lazy<Vec<F>> = Lazy::new(|| {
    vec![
        F::new(
            "ada".to_string(),
            vec![".adb".to_string(), ".ads".to_string()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "apiblueprint".into(),
            vec![".apib".into(), ".apiblueprint".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "argdown".into(),
            vec![
                ".ad".into(),
                ".adown".into(),
                ".argdn".into(),
                ".argdown".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "asciidoc".into(),
            vec![".adoc".into(), ".asc".into(), ".asciidoc".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "bat".into(),
            vec![".bat".into(), ".cmd".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "bazel".into(),
            vec![".bazel".into(), ".bzl".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("bibtex".into(), vec![".bib".into()], None, None, None, None),
        F::new(
            "bicep".into(),
            vec![".bicep".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "c".into(),
            vec![".c".into(), ".i".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "cache_files".into(),
            vec![],
            Some(vec![
                ".DS_Store".into(),
                ".cspellcache".into(),
                ".eslintcache".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "clojure".into(),
            vec![
                ".clj".into(),
                ".cljc".into(),
                ".cljs".into(),
                ".cljx".into(),
                ".clojure".into(),
                ".edn".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "cmake".into(),
            vec![".cmake".into()],
            Some(vec!["CMakeLists.txt".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "coffeescript".into(),
            vec![".coffee".into(), ".cson".into(), ".iced".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "cpp".into(),
            vec![
                ".c++".into(),
                ".c++m".into(),
                ".cc".into(),
                ".ccm".into(),
                ".cpp".into(),
                ".cppm".into(),
                ".cxx".into(),
                ".cxxm".into(),
                ".h".into(),
                ".h++".into(),
                ".h.in".into(),
                ".hh".into(),
                ".hpp".into(),
                ".hpp.in".into(),
                ".hxx".into(),
                ".ii".into(),
                ".inl".into(),
                ".ino".into(),
                ".ipp".into(),
                ".ixx".into(),
                ".mm".into(),
                ".tpp".into(),
                ".txx".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new("cpp_embedded_latex".into(), vec![], None, None, None, None),
        F::new(
            "csharp".into(),
            vec![".cake".into(), ".cs".into(), ".csx".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("css".into(), vec![".css".into()], None, None, None, None),
        F::new(
            "cuda-cpp".into(),
            vec![".cu".into(), ".cuh".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("dart".into(), vec![".dart".into()], None, None, None, None),
        F::new(
            "dhall".into(),
            vec![".dhall".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "diff".into(),
            vec![".diff".into(), ".patch".into(), ".rej".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "dockercompose".into(),
            vec![],
            Some(vec![
                "*docker*compose*.yaml".into(),
                "*docker*compose*.yml".into(),
                "compose.*.yaml".into(),
                "compose.*.yml".into(),
                "compose.yaml".into(),
                "compose.yml".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "dockerfile".into(),
            vec![".containerfile".into(), ".dockerfile".into()],
            Some(vec![
                "*.Dockerfile.*".into(),
                "Containerfile".into(),
                "Containerfile.*".into(),
                "Dockerfile".into(),
                "Dockerfile.*".into(),
                "Dockerfile.dev".into(),
                "dockerfile".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new("elisp".into(), vec![".el".into()], None, None, None, None),
        F::new(
            "elixir".into(),
            vec![".ex".into(), ".exs".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("elm".into(), vec![".elm".into()], None, None, None, None),
        F::new(
            "erb".into(),
            vec![".erb".into(), ".html.erb".into(), ".rhtml".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "fsharp".into(),
            vec![
                ".fs".into(),
                ".fsi".into(),
                ".fsscript".into(),
                ".fsx".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "git-commit".into(),
            vec![],
            Some(vec!["COMMIT_EDITMSG".into(), "MERGE_MSG".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "git-rebase".into(),
            vec![],
            Some(vec!["git-rebase-todo".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "github-issues".into(),
            vec![".github-issues".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("go".into(), vec![".go".into()], None, None, None, None),
        F::new(
            "godot".into(),
            vec![
                ".gd".into(),
                ".godot".into(),
                ".tres".into(),
                ".tscn".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "gradle".into(),
            vec![".gradle".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "groovy".into(),
            vec![
                ".gradle".into(),
                ".groovy".into(),
                ".gvy".into(),
                ".jenkinsfile".into(),
                ".nf".into(),
            ],
            Some(vec!["Jenkinsfile".into(), "Jenkinsfile*".into()]),
            None,
            None,
            None,
        ),
        F::new("haml".into(), vec![".haml".into()], None, None, None, None),
        F::new(
            "handlebars".into(),
            vec![".handlebars".into(), ".hbs".into(), ".hjs".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "haskell".into(),
            vec![".hs".into(), ".lhs".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("haxe".into(), vec![".hx".into()], None, None, None, None),
        F::new(
            "hlsl".into(),
            vec![
                ".cginc".into(),
                ".compute".into(),
                ".fx".into(),
                ".fxh".into(),
                ".hlsl".into(),
                ".hlsli".into(),
                ".psh".into(),
                ".vsh".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "html".into(),
            vec![
                ".asp".into(),
                ".aspx".into(),
                ".ejs".into(),
                ".htm".into(),
                ".html".into(),
                ".jshtm".into(),
                ".jsp".into(),
                ".mdoc".into(),
                ".rhtml".into(),
                ".shtml".into(),
                ".volt".into(),
                ".vue".into(),
                ".xht".into(),
                ".xhtml".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "ignore".into(),
            vec![
                ".git-blame-ignore-revs".into(),
                ".gitignore".into(),
                ".gitignore_global".into(),
                ".npmignore".into(),
            ],
            Some(vec![".vscodeignore".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "ini".into(),
            vec![".conf".into(), ".ini".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "jade".into(),
            vec![".jade".into(), ".pug".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "java".into(),
            vec![".jav".into(), ".java".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "javascript".into(),
            vec![
                ".cjs".into(),
                ".es6".into(),
                ".js".into(),
                ".mjs".into(),
                ".pac".into(),
            ],
            Some(vec!["jakefile".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "javascriptreact".into(),
            vec![".jsx".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "jinja".into(),
            vec![".jinja".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "json".into(),
            vec![
                ".babelrc".into(),
                ".bowerrc".into(),
                ".code-profile".into(),
                ".css.map".into(),
                ".eslintrc".into(),
                ".geojson".into(),
                ".har".into(),
                ".ipynb".into(),
                ".js.map".into(),
                ".jscsrc".into(),
                ".jshintrc".into(),
                ".jslintrc".into(),
                ".json".into(),
                ".jsonc".into(),
                ".jsonld".into(),
                ".ts.map".into(),
                ".tsbuildinfo".into(),
                ".vuerc".into(),
                ".webmanifest".into(),
            ],
            Some(vec![".watchmanconfig".into(), "composer.lock".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "jsonc".into(),
            vec![
                ".babelrc".into(),
                ".code-workspace".into(),
                ".color-theme.json".into(),
                ".eslintrc".into(),
                ".eslintrc.json".into(),
                ".hintrc".into(),
                ".icon-theme.json".into(),
                ".jsfmtrc".into(),
                ".jshintrc".into(),
                ".jsonc".into(),
                ".language-configuration.json".into(),
                ".swcrc".into(),
            ],
            Some(vec![
                ".babelrc.json".into(),
                ".code-workspace".into(),
                ".devcontainer.json".into(),
                ".ember-cli".into(),
                "argv.json".into(),
                "babel.config.json".into(),
                "devcontainer.json".into(),
                "extensions.json".into(),
                "jsconfig-*.json".into(),
                "jsconfig.*.json".into(),
                "jsconfig.json".into(),
                "keybindings.json".into(),
                "launch.json".into(),
                "profiles.json".into(),
                "settings.json".into(),
                "tasks.json".into(),
                "tsconfig-*.json".into(),
                "tsconfig.*.json".into(),
                "tsconfig.json".into(),
                "typedoc.json".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "jsonl".into(),
            vec![".jsonl".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("jsx-tags".into(), vec![], None, None, None, None),
        F::new("julia".into(), vec![".jl".into()], None, None, None, None),
        F::new(
            "juliamarkdown".into(),
            vec![".jmd".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "jungle".into(),
            vec![".jungle".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("kotlin".into(), vec![".kt".into()], None, None, None, None),
        F::new(
            "latex".into(),
            vec![".ctx".into(), ".ltx".into(), ".tex".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("less".into(), vec![".less".into()], None, None, None, None),
        F::new(
            "lisp".into(),
            vec![".fasl".into(), ".l".into(), ".lisp".into(), ".lsp".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "literate haskell".into(),
            vec![".lhs".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "lock".into(),
            vec![".lock".into()],
            Some(vec![
                "Cargo.lock".into(),
                "berksfile.lock".into(),
                "composer.lock".into(),
                "package-lock.json".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "log".into(),
            vec![".log".into()],
            Some(vec!["*.log.?".into()]),
            None,
            None,
            None,
        ),
        F::new("lua".into(), vec![".lua".into()], None, None, None, None),
        F::new(
            "makefile".into(),
            vec![".mak".into(), ".mk".into()],
            Some(vec![
                "GNUmakefile".into(),
                "Makefile".into(),
                "OCamlMakefile".into(),
                "makefile".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "map".into(),
            vec![
                ".map".into(),
                ".css.map".into(),
                ".ts.map".into(),
                ".js.map".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "markdown".into(),
            vec![
                ".markdn".into(),
                ".markdown".into(),
                ".md".into(),
                ".mdown".into(),
                ".mdtext".into(),
                ".mdtxt".into(),
                ".mdwn".into(),
                ".mkd".into(),
                ".workbook".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "markdown_latex_combined".into(),
            vec![],
            None,
            None,
            None,
            None,
        ),
        F::new("markdown-math".into(), vec![], None, None, None, None),
        F::new("mdx".into(), vec![".mdx".into()], None, None, None, None),
        F::new(
            "monkeyc".into(),
            vec![".mb".into(), ".mc".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "mustache".into(),
            vec![
                ".mst".into(),
                ".mu".into(),
                ".mustache".into(),
                ".stache".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new("nix".into(), vec![".nix".into()], None, None, None, None),
        F::new(
            "nunjucks".into(),
            vec![
                ".nj".into(),
                ".njk".into(),
                ".nunj".into(),
                ".nunjs".into(),
                ".nunjucks".into(),
                ".tmpl".into(),
                ".tpl".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "objective-c".into(),
            vec![".m".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "objective-cpp".into(),
            vec![".mm".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "ocaml".into(),
            vec![
                ".eliom".into(),
                ".eliomi".into(),
                ".ml".into(),
                ".mli".into(),
                ".mll".into(),
                ".mly".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new("pdf".into(), vec![".pdf".into()], None, None, None, None),
        F::new(
            "pem".into(),
            vec![".pem".into(), ".private-key.pem".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "pem-private-key".into(),
            vec![".private-key.pem".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "perl".into(),
            vec![
                ".PL".into(),
                ".pl".into(),
                ".pm".into(),
                ".pod".into(),
                ".psgi".into(),
                ".t".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "perl6".into(),
            vec![".nqp".into(), ".p6".into(), ".pl6".into(), ".pm6".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "php".into(),
            vec![
                ".ctp".into(),
                ".php".into(),
                ".php4".into(),
                ".php5".into(),
                ".phtml".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "plaintext".into(),
            vec![".txt".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "powershell".into(),
            vec![
                ".ps1".into(),
                ".psd1".into(),
                ".psm1".into(),
                ".psrc".into(),
                ".pssc".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "properties".into(),
            vec![
                ".cfg".into(),
                ".conf".into(),
                ".directory".into(),
                ".editorconfig".into(),
                ".gitattributes".into(),
                ".gitconfig".into(),
                ".gitmodules".into(),
                ".npmrc".into(),
                ".properties".into(),
                ".repo".into(),
            ],
            Some(vec![".env".into(), "gitconfig".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "puppet".into(),
            vec![".puppet".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "purescript".into(),
            vec![".purs".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "python".into(),
            vec![
                ".cpy".into(),
                ".gyp".into(),
                ".gypi".into(),
                ".ipy".into(),
                ".py".into(),
                ".pyi".into(),
                ".pyt".into(),
                ".pyw".into(),
                ".rpy".into(),
            ],
            Some(vec!["SConscript".into(), "SConstruct".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "r".into(),
            vec![
                ".R".into(),
                ".r".into(),
                ".rhistory".into(),
                ".rprofile".into(),
                ".rt".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "raku".into(),
            vec![
                ".nqp".into(),
                ".p6".into(),
                ".pl6".into(),
                ".pm6".into(),
                ".raku".into(),
                ".rakudoc".into(),
                ".rakumod".into(),
                ".rakutest".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "razor".into(),
            vec![".cshtml".into(), ".razor".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "rescript".into(),
            vec![".res".into(), ".resi".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "restructuredtext".into(),
            vec![".rst".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "rsa".into(),
            vec![".pub".into()],
            Some(vec!["id_rsa".into(), "id_rsa.pub".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "ruby".into(),
            vec![
                ".erb".into(),
                ".gemspec".into(),
                ".podspec".into(),
                ".rake".into(),
                ".rb".into(),
                ".rbi".into(),
                ".rbx".into(),
                ".rjs".into(),
                ".ru".into(),
            ],
            Some(vec![
                "Gemfile".into(),
                "appfile".into(),
                "appraisals".into(),
                "berksfile".into(),
                "berksfile.lock".into(),
                "brewfile".into(),
                "capfile".into(),
                "cheffile".into(),
                "dangerfile".into(),
                "deliverfile".into(),
                "fastfile".into(),
                "gemfile".into(),
                "guardfile".into(),
                "gymfile".into(),
                "hobofile".into(),
                "matchfile".into(),
                "podfile".into(),
                "puppetfile".into(),
                "rakefile".into(),
                "rantfile".into(),
                "scanfile".into(),
                "snapfile".into(),
                "thorfile".into(),
                "vagrantfile".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new("rust".into(), vec![".rs".into()], None, None, None, None),
        F::new("sass".into(), vec![".sass".into()], None, None, None, None),
        F::new(
            "scala".into(),
            vec![".sbt".into(), ".sc".into(), ".scala".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("scss".into(), vec![".scss".into()], None, None, None, None),
        F::new(
            "search-result".into(),
            vec![".code-search".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "shaderlab".into(),
            vec![".cginc".into(), ".shader".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "shellscript".into(),
            vec![
                ".Xsession".into(),
                ".bash".into(),
                ".bash_aliases".into(),
                ".bash_login".into(),
                ".bash_logout".into(),
                ".bash_profile".into(),
                ".bashrc".into(),
                ".csh".into(),
                ".cshrc".into(),
                ".ebuild".into(),
                ".eclass".into(),
                ".fish".into(),
                ".install".into(),
                ".ksh".into(),
                ".profile".into(),
                ".sh".into(),
                ".tcshrc".into(),
                ".xprofile".into(),
                ".xsession".into(),
                ".xsessionrc".into(),
                ".yash_profile".into(),
                ".yashrc".into(),
                ".zlogin".into(),
                ".zlogout".into(),
                ".zprofile".into(),
                ".zsh".into(),
                ".zsh-theme".into(),
                ".zshenv".into(),
                ".zshrc".into(),
            ],
            Some(vec![
                ".env.*".into(),
                ".envrc".into(),
                ".hushlogin".into(),
                "APKBUILD".into(),
                "PKGBUILD".into(),
                "bashrc_Apple_Terminal".into(),
                "zlogin".into(),
                "zlogout".into(),
                "zprofile".into(),
                "zshenv".into(),
                "zshrc".into(),
                "zshrc_Apple_Terminal".into(),
            ]),
            None,
            None,
            None,
        ),
        F::new(
            "snippets".into(),
            vec![".code-snippets".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "sql".into(),
            vec![".dsql".into(), ".sql".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "stylus".into(),
            vec![".styl".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "svelte".into(),
            vec![".svelte".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "swift".into(),
            vec![".swift".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "terraform".into(),
            vec![
                ".hcl".into(),
                ".tf".into(),
                ".tf.json".into(),
                ".tfvars".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "tex".into(),
            vec![".bbx".into(), ".cbx".into(), ".cls".into(), ".sty".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "tfvars".into(),
            vec![".tfvars".into()],
            None,
            None,
            Some("Terraform Variables".into()),
            None,
        ),
        F::new(
            "todo".into(),
            vec![],
            Some(vec!["todo".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "toml".into(),
            vec![".toml".into()],
            Some(vec!["Cargo.lock".into(), "Cargo.toml".into()]),
            None,
            None,
            None,
        ),
        F::new(
            "typescript".into(),
            vec![".cts".into(), ".mts".into(), ".ts".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "typescriptreact".into(),
            vec![".tsx".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "typst".into(),
            vec![".typst".into()],
            None,
            None,
            None,
            None,
        ),
        F::new("vala".into(), vec![".vala".into()], None, None, None, None),
        F::new(
            "vb".into(),
            vec![
                ".bas".into(),
                ".brs".into(),
                ".vb".into(),
                ".vba".into(),
                ".vbs".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new("vue".into(), vec![".vue".into()], None, None, None, None),
        F::new(
            "xml".into(),
            vec![
                ".ascx".into(),
                ".atom".into(),
                ".axaml".into(),
                ".axml".into(),
                ".bpmn".into(),
                ".config".into(),
                ".cpt".into(),
                ".csl".into(),
                ".csproj".into(),
                ".csproj.user".into(),
                ".dita".into(),
                ".ditamap".into(),
                ".dtd".into(),
                ".dtml".into(),
                ".ent".into(),
                ".fsproj".into(),
                ".fxml".into(),
                ".iml".into(),
                ".isml".into(),
                ".jmx".into(),
                ".launch".into(),
                ".menu".into(),
                ".mod".into(),
                ".mxml".into(),
                ".nuspec".into(),
                ".opml".into(),
                ".owl".into(),
                ".proj".into(),
                ".props".into(),
                ".pt".into(),
                ".publishsettings".into(),
                ".pubxml".into(),
                ".pubxml.user".into(),
                ".rbxlx".into(),
                ".rbxmx".into(),
                ".rdf".into(),
                ".rng".into(),
                ".rss".into(),
                ".shproj".into(),
                ".storyboard".into(),
                ".svg".into(),
                ".targets".into(),
                ".tld".into(),
                ".tmx".into(),
                ".vbproj".into(),
                ".vbproj.user".into(),
                ".vcxproj".into(),
                ".vcxproj.filters".into(),
                ".wsdl".into(),
                ".wxi".into(),
                ".wxl".into(),
                ".wxs".into(),
                ".xaml".into(),
                ".xbl".into(),
                ".xib".into(),
                ".xlf".into(),
                ".xliff".into(),
                ".xml".into(),
                ".xoml".into(),
                ".xpdl".into(),
                ".xsd".into(),
                ".xul".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "xsl".into(),
            vec![".xsl".into(), ".xslt".into()],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "yaml".into(),
            vec![
                ".cff".into(),
                ".eyaml".into(),
                ".eyml".into(),
                ".yaml".into(),
                ".yaml-tmlanguage".into(),
                ".yaml-tmpreferences".into(),
                ".yaml-tmtheme".into(),
                ".yml".into(),
            ],
            None,
            None,
            None,
            None,
        ),
        F::new(
            "binary".into(),
            vec![
                ".bin".into(),
                ".cur".into(),
                ".dll".into(),
                ".eot".into(),
                ".exe".into(),
                ".gz".into(),
                ".lib".into(),
                ".o".into(),
                ".obj".into(),
                ".phar".into(),
                ".zip".into(),
            ],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "dll".into(),
            vec![".dll".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "exe".into(),
            vec![".exe".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "fonts".into(),
            vec![".ttf".into(), ".woff".into(), ".woff2".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "gzip".into(),
            vec![".gz".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "image".into(),
            vec![
                ".bmp".into(),
                ".exr".into(),
                ".gif".into(),
                ".heic".into(),
                ".ico".into(),
                ".jpeg".into(),
                ".jpg".into(),
                ".pbm".into(),
                ".pgm".into(),
                ".png".into(),
                ".ppm".into(),
                ".ras".into(),
                ".sgi".into(),
                ".tiff".into(),
                ".webp".into(),
                ".xbm".into(),
            ],
            None,
            Some(FileTypeFormat::Binary),
            Some("Some image extensions".into()),
            None,
        ),
        F::new(
            "jar".into(),
            vec![".jar".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "mdb".into(),
            vec![".mdb".into()],
            None,
            Some(FileTypeFormat::Binary),
            Some("Microsoft Access DB".into()),
            None,
        ),
        F::new(
            "object-file".into(),
            vec![".o".into(), ".obj".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "spv".into(),
            vec![".spv".into()],
            None,
            Some(FileTypeFormat::Binary),
            Some("SPSS Output Document".into()),
            None,
        ),
        F::new(
            "trie".into(),
            vec![".trie".into()],
            None,
            Some(FileTypeFormat::Binary),
            Some("CSpell dictionary file.".into()),
            None,
        ),
        F::new(
            "video".into(),
            vec![
                ".avi".into(),
                ".flv".into(),
                ".mkv".into(),
                ".mov".into(),
                ".mp4".into(),
                ".mpeg".into(),
                ".mpg".into(),
                ".wmv".into(),
            ],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
        F::new(
            "webm".into(),
            vec![".webm".into()],
            None,
            Some(FileTypeFormat::Binary),
            Some("WebM is an audiovisual media file format.".into()),
            None,
        ),
        F::new(
            "wheel".into(),
            vec![".whl".into()],
            None,
            Some(FileTypeFormat::Binary),
            None,
            None,
        ),
    ]
});
