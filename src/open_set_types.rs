use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use crate::{SemanticTokenType, SemanticTokenModifier};


/// Predefined Semantic token types
/// @since 3.16.0
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SemanticTokenTypes(pub String);

impl SemanticTokenTypes {
    pub const NAMESPACE: &'static str = "namespace";
    pub const TYPE: &'static str = "type";
    pub const CLASS: &'static str = "class";
    pub const ENUM: &'static str = "enum";
    pub const INTERFACE: &'static str = "interface";
    pub const STRUCT: &'static str = "struct";
    pub const TYPE_PARAMETER: &'static str = "typeParameter";
    pub const PARAMETER: &'static str = "parameter";
    pub const VARIABLE: &'static str = "variable";
    pub const PROPERTY: &'static str = "property";
    pub const ENUM_MEMBER: &'static str = "enumMember";
    pub const EVENT: &'static str = "event";
    pub const FUNCTION: &'static str = "function";
    pub const METHOD: &'static str = "method";
    pub const MACRO: &'static str = "macro";
    pub const KEYWORD: &'static str = "keyword";
    pub const MODIFIER: &'static str = "modifier";
    pub const COMMENT: &'static str = "comment";
    pub const STRING: &'static str = "string";
    pub const NUMBER: &'static str = "number";
    pub const REGEXP: &'static str = "regexp";
    pub const OPERATOR: &'static str = "operator";

    /// @since 3.17.0
    pub const DECORATOR: &'static str = "decorator";

    /// @since 3.18.0
    pub const LABEL: &'static str = "label";

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SemanticTokenTypes {
    fn from(s: String) -> Self {
        SemanticTokenTypes(s)
    }
}

impl From<&'static str> for SemanticTokenTypes {
    fn from(s: &'static str) -> Self {
        SemanticTokenTypes(s.to_string())
    }
}

impl AsRef<str> for SemanticTokenTypes {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SemanticTokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Predefined Semantic token modifiers
/// @since 3.16.0
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SemanticTokenModifiers(pub String);

impl SemanticTokenModifiers {
    pub const DECLARATION: &'static str = "declaration";
    pub const DEFINITION: &'static str = "definition";
    pub const READONLY: &'static str = "readonly";
    pub const STATIC: &'static str = "static";
    pub const DEPRECATED: &'static str = "deprecated";
    pub const ABSTRACT: &'static str = "abstract";
    pub const ASYNC: &'static str = "async";
    pub const MODIFICATION: &'static str = "modification";
    pub const DOCUMENTATION: &'static str = "documentation";
    pub const DEFAULT_LIBRARY: &'static str = "defaultLibrary";

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for SemanticTokenModifiers {
    fn from(s: String) -> Self {
        SemanticTokenModifiers(s)
    }
}

impl From<&'static str> for SemanticTokenModifiers {
    fn from(s: &'static str) -> Self {
        SemanticTokenModifiers(s.to_string())
    }
}

impl AsRef<str> for SemanticTokenModifiers {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SemanticTokenModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Predefined Language kinds
/// @since 3.18.0
#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LanguageKind {
    Known(LanguageKindEnum),
    Custom(Cow<'static, str>),
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Clone, Deserialize, Serialize)]
pub enum LanguageKindEnum {
    #[serde(rename = "abap")]
    Abap,
    #[serde(rename = "bat")]
    WindowsBat,
    #[serde(rename = "bibtex")]
    BibTeX,
    #[serde(rename = "clojure")]
    Clojure,
    #[serde(rename = "coffeescript")]
    Coffeescript,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "cpp")]
    Cpp,
    #[serde(rename = "csharp")]
    CSharp,
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "d")]
    D,
    #[serde(rename = "diff")]
    Diff,
    #[serde(rename = "dart")]
    Dart,
    #[serde(rename = "dockerfile")]
    Dockerfile,
    #[serde(rename = "elixir")]
    Elixir,
    #[serde(rename = "erlang")]
    Erlang,
    #[serde(rename = "fsharp")]
    FSharp,
    #[serde(rename = "git-commit")]
    GitCommit,
    #[serde(rename = "git-rebase")]
    GitRebase,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "groovy")]
    Groovy,
    #[serde(rename = "handlebars")]
    Handlebars,
    #[serde(rename = "haskell")]
    Haskell,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "ini")]
    Ini,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "javascript")]
    JavaScript,
    #[serde(rename = "javascriptreact")]
    JavaScriptReact,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "latex")]
    LaTeX,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "makefile")]
    Makefile,
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    #[serde(rename = "objective-cpp")]
    ObjectiveCPP,
    #[serde(rename = "pascal")]
    Pascal,
    #[serde(rename = "perl")]
    Perl,
    #[serde(rename = "perl6")]
    Perl6,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "powershell")]
    Powershell,
    #[serde(rename = "jade")]
    Pug,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "razor")]
    Razor,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "scss")]
    Scss,
    #[serde(rename = "sass")]
    Sass,
    #[serde(rename = "scala")]
    Scala,
    #[serde(rename = "shaderlab")]
    ShaderLab,
    #[serde(rename = "shellscript")]
    ShellScript,
    #[serde(rename = "sql")]
    Sql,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "typescript")]
    TypeScript,
    #[serde(rename = "typescriptreact")]
    TypeScriptReact,
    #[serde(rename = "tex")]
    TeX,
    #[serde(rename = "vb")]
    VisualBasic,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "xsl")]
    Xsl,
    #[serde(rename = "yaml")]
    Yaml,
}

impl LanguageKind {
    pub const ABAP: LanguageKind = LanguageKind::Known(LanguageKindEnum::Abap);
    pub const WINDOWS_BAT: LanguageKind = LanguageKind::Known(LanguageKindEnum::WindowsBat);
    pub const BIB_TE_X: LanguageKind = LanguageKind::Known(LanguageKindEnum::BibTeX);
    pub const CLOJURE: LanguageKind = LanguageKind::Known(LanguageKindEnum::Clojure);
    pub const COFFEESCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::Coffeescript);
    pub const C: LanguageKind = LanguageKind::Known(LanguageKindEnum::C);
    pub const CPP: LanguageKind = LanguageKind::Known(LanguageKindEnum::Cpp);
    pub const C_SHARP: LanguageKind = LanguageKind::Known(LanguageKindEnum::CSharp);
    pub const CSS: LanguageKind = LanguageKind::Known(LanguageKindEnum::Css);
    pub const D: LanguageKind = LanguageKind::Known(LanguageKindEnum::D);
    pub const DELPHI: LanguageKind = LanguageKind::Known(LanguageKindEnum::Pascal);
    pub const DIFF: LanguageKind = LanguageKind::Known(LanguageKindEnum::Diff);
    pub const DART: LanguageKind = LanguageKind::Known(LanguageKindEnum::Dart);
    pub const DOCKERFILE: LanguageKind = LanguageKind::Known(LanguageKindEnum::Dockerfile);
    pub const ELIXIR: LanguageKind = LanguageKind::Known(LanguageKindEnum::Elixir);
    pub const ERLANG: LanguageKind = LanguageKind::Known(LanguageKindEnum::Erlang);
    pub const F_SHARP: LanguageKind = LanguageKind::Known(LanguageKindEnum::FSharp);
    pub const GIT_COMMIT: LanguageKind = LanguageKind::Known(LanguageKindEnum::GitCommit);
    pub const GIT_REBASE: LanguageKind = LanguageKind::Known(LanguageKindEnum::GitRebase);
    pub const GO: LanguageKind = LanguageKind::Known(LanguageKindEnum::Go);
    pub const GROOVY: LanguageKind = LanguageKind::Known(LanguageKindEnum::Groovy);
    pub const HANDLEBARS: LanguageKind = LanguageKind::Known(LanguageKindEnum::Handlebars);
    pub const HASKELL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Haskell);
    pub const HTML: LanguageKind = LanguageKind::Known(LanguageKindEnum::Html);
    pub const INI: LanguageKind = LanguageKind::Known(LanguageKindEnum::Ini);
    pub const JAVA: LanguageKind = LanguageKind::Known(LanguageKindEnum::Java);
    pub const JAVA_SCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::JavaScript);
    pub const JAVA_SCRIPT_REACT: LanguageKind = LanguageKind::Known(LanguageKindEnum::JavaScriptReact);
    pub const JSON: LanguageKind = LanguageKind::Known(LanguageKindEnum::Json);
    pub const LA_TE_X: LanguageKind = LanguageKind::Known(LanguageKindEnum::LaTeX);
    pub const LESS: LanguageKind = LanguageKind::Known(LanguageKindEnum::Less);
    pub const LUA: LanguageKind = LanguageKind::Known(LanguageKindEnum::Lua);
    pub const MAKEFILE: LanguageKind = LanguageKind::Known(LanguageKindEnum::Makefile);
    pub const MARKDOWN: LanguageKind = LanguageKind::Known(LanguageKindEnum::Markdown);
    pub const OBJECTIVE_C: LanguageKind = LanguageKind::Known(LanguageKindEnum::ObjectiveC);
    pub const OBJECTIVE_CPP: LanguageKind = LanguageKind::Known(LanguageKindEnum::ObjectiveCPP);
    pub const PASCAL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Pascal);
    pub const PERL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Perl);
    pub const PERL6: LanguageKind = LanguageKind::Known(LanguageKindEnum::Perl6);
    pub const PHP: LanguageKind = LanguageKind::Known(LanguageKindEnum::Php);
    pub const PLAINTEXT: LanguageKind = LanguageKind::Known(LanguageKindEnum::Plaintext);
    pub const POWERSHELL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Powershell);
    pub const PUG: LanguageKind = LanguageKind::Known(LanguageKindEnum::Pug);
    pub const PYTHON: LanguageKind = LanguageKind::Known(LanguageKindEnum::Python);
    pub const R: LanguageKind = LanguageKind::Known(LanguageKindEnum::R);
    pub const RAZOR: LanguageKind = LanguageKind::Known(LanguageKindEnum::Razor);
    pub const RUBY: LanguageKind = LanguageKind::Known(LanguageKindEnum::Ruby);
    pub const RUST: LanguageKind = LanguageKind::Known(LanguageKindEnum::Rust);
    pub const SCSS: LanguageKind = LanguageKind::Known(LanguageKindEnum::Scss);
    pub const SASS: LanguageKind = LanguageKind::Known(LanguageKindEnum::Sass);
    pub const SCALA: LanguageKind = LanguageKind::Known(LanguageKindEnum::Scala);
    pub const SHADER_LAB: LanguageKind = LanguageKind::Known(LanguageKindEnum::ShaderLab);
    pub const SHELL_SCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::ShellScript);
    pub const SQL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Sql);
    pub const SWIFT: LanguageKind = LanguageKind::Known(LanguageKindEnum::Swift);
    pub const TYPE_SCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::TypeScript);
    pub const TYPE_SCRIPT_REACT: LanguageKind = LanguageKind::Known(LanguageKindEnum::TypeScriptReact);
    pub const TE_X: LanguageKind = LanguageKind::Known(LanguageKindEnum::TeX);
    pub const VISUAL_BASIC: LanguageKind = LanguageKind::Known(LanguageKindEnum::VisualBasic);
    pub const XML: LanguageKind = LanguageKind::Known(LanguageKindEnum::Xml);
    pub const XSL: LanguageKind = LanguageKind::Known(LanguageKindEnum::Xsl);
    pub const YAML: LanguageKind = LanguageKind::Known(LanguageKindEnum::Yaml);

    // Backward compatibility aliases
    pub const TYPESCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::TypeScript);
    pub const JAVASCRIPT: LanguageKind = LanguageKind::Known(LanguageKindEnum::JavaScript);

    pub const fn new(kind: &'static str) -> Self {
        LanguageKind::Custom(Cow::Borrowed(kind))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Known(k) => match k {
                LanguageKindEnum::Abap => "abap",
                LanguageKindEnum::WindowsBat => "bat",
                LanguageKindEnum::BibTeX => "bibtex",
                LanguageKindEnum::Clojure => "clojure",
                LanguageKindEnum::Coffeescript => "coffeescript",
                LanguageKindEnum::C => "c",
                LanguageKindEnum::Cpp => "cpp",
                LanguageKindEnum::CSharp => "csharp",
                LanguageKindEnum::Css => "css",
                LanguageKindEnum::D => "d",
                LanguageKindEnum::Diff => "diff",
                LanguageKindEnum::Dart => "dart",
                LanguageKindEnum::Dockerfile => "dockerfile",
                LanguageKindEnum::Elixir => "elixir",
                LanguageKindEnum::Erlang => "erlang",
                LanguageKindEnum::FSharp => "fsharp",
                LanguageKindEnum::GitCommit => "git-commit",
                LanguageKindEnum::GitRebase => "git-rebase",
                LanguageKindEnum::Go => "go",
                LanguageKindEnum::Groovy => "groovy",
                LanguageKindEnum::Handlebars => "handlebars",
                LanguageKindEnum::Haskell => "haskell",
                LanguageKindEnum::Html => "html",
                LanguageKindEnum::Ini => "ini",
                LanguageKindEnum::Java => "java",
                LanguageKindEnum::JavaScript => "javascript",
                LanguageKindEnum::JavaScriptReact => "javascriptreact",
                LanguageKindEnum::Json => "json",
                LanguageKindEnum::LaTeX => "latex",
                LanguageKindEnum::Less => "less",
                LanguageKindEnum::Lua => "lua",
                LanguageKindEnum::Makefile => "makefile",
                LanguageKindEnum::Markdown => "markdown",
                LanguageKindEnum::ObjectiveC => "objective-c",
                LanguageKindEnum::ObjectiveCPP => "objective-cpp",
                LanguageKindEnum::Pascal => "pascal",
                LanguageKindEnum::Perl => "perl",
                LanguageKindEnum::Perl6 => "perl6",
                LanguageKindEnum::Php => "php",
                LanguageKindEnum::Plaintext => "plaintext",
                LanguageKindEnum::Powershell => "powershell",
                LanguageKindEnum::Pug => "jade",
                LanguageKindEnum::Python => "python",
                LanguageKindEnum::R => "r",
                LanguageKindEnum::Razor => "razor",
                LanguageKindEnum::Ruby => "ruby",
                LanguageKindEnum::Rust => "rust",
                LanguageKindEnum::Scss => "scss",
                LanguageKindEnum::Sass => "sass",
                LanguageKindEnum::Scala => "scala",
                LanguageKindEnum::ShaderLab => "shaderlab",
                LanguageKindEnum::ShellScript => "shellscript",
                LanguageKindEnum::Sql => "sql",
                LanguageKindEnum::Swift => "swift",
                LanguageKindEnum::TypeScript => "typescript",
                LanguageKindEnum::TypeScriptReact => "typescriptreact",
                LanguageKindEnum::TeX => "tex",
                LanguageKindEnum::VisualBasic => "vb",
                LanguageKindEnum::Xml => "xml",
                LanguageKindEnum::Xsl => "xsl",
                LanguageKindEnum::Yaml => "yaml",
            },
            Self::Custom(c) => c.as_ref(),
        }
    }
}

impl From<String> for LanguageKind {
    fn from(from: String) -> Self {
        LanguageKind::Custom(Cow::from(from))
    }
}

impl From<&'static str> for LanguageKind {
    fn from(from: &'static str) -> Self {
        LanguageKind::new(from)
    }
}

impl AsRef<str> for LanguageKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<LanguageKind> for String {
    fn from(kind: LanguageKind) -> Self {
        match kind {
            LanguageKind::Known(_) => kind.as_str().to_string(),
            LanguageKind::Custom(c) => c.into_owned(),
        }
    }
}

impl From<LanguageKind> for Cow<'static, str> {
    fn from(kind: LanguageKind) -> Self {
        match kind {
            LanguageKind::Known(_) => Cow::Owned(kind.as_str().to_string()),
            LanguageKind::Custom(c) => c,
        }
    }
}

impl From<SemanticTokenTypes> for String {
    fn from(val: SemanticTokenTypes) -> Self {
        val.0
    }
}

impl From<SemanticTokenModifiers> for String {
    fn from(val: SemanticTokenModifiers) -> Self {
        val.0
    }
}

impl From<SemanticTokenTypes> for SemanticTokenType {
    fn from(types: SemanticTokenTypes) -> Self {
        SemanticTokenType::Custom(Cow::Owned(types.0))
    }
}

impl From<SemanticTokenType> for SemanticTokenTypes {
    fn from(token_type: SemanticTokenType) -> Self {
        SemanticTokenTypes(token_type.as_str().to_string())
    }
}

impl From<SemanticTokenModifiers> for SemanticTokenModifier {
    fn from(mods: SemanticTokenModifiers) -> Self {
        SemanticTokenModifier::Custom(Cow::Owned(mods.0))
    }
}

impl From<SemanticTokenModifier> for SemanticTokenModifiers {
    fn from(token_mod: SemanticTokenModifier) -> Self {
        SemanticTokenModifiers(token_mod.as_str().to_string())
    }
}

