use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use swc_common::{input::StringInput, sync::Lrc, FileName, SourceMap};
use swc_ecma_ast as ast;
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, Syntax};

use crate::compiler::{
    CompiledBinding, CompiledBindingSource, CompiledOrientation, CompiledProps, CompiledRsxNode,
    CompiledStyleValue,
};
use crate::error::{GuiError, GuiResult};

pub fn parse_rsx(source: &str) -> GuiResult<CompiledRsxNode> {
    parse_rsx_source("a3s.rsx", source)
}

pub fn parse_rsx_source(source_name: impl AsRef<str>, source: &str) -> GuiResult<CompiledRsxNode> {
    let source_name = normalize_rsx_source_name(source_name);
    let root_source = match extract_rsx_function_component(&source_name, source)? {
        Some(component) => component.root_expression,
        None => source.trim().to_string(),
    };
    let module = parse_rsx_module(&source_name, &root_source)?;
    let root = find_root_expr(&module).map_err(|error| with_source_context(&source_name, error))?;
    let node = lower_root_expr(root).map_err(|error| with_source_context(&source_name, error))?;
    node.validate()
        .map_err(|error| with_source_context(&source_name, error))?;
    Ok(node)
}

pub fn parse_rsx_file(path: impl AsRef<Path>) -> GuiResult<CompiledRsxNode> {
    let path = path.as_ref();
    let source = std::fs::read_to_string(path).map_err(|error| {
        GuiError::invalid_tree(format!(
            "failed to read RSX source {:?}: {error}",
            path.display()
        ))
    })?;
    parse_rsx_source(path.display().to_string(), &source)
}

fn parse_rsx_module(source_name: &str, source: &str) -> GuiResult<ast::Module> {
    let source_map: Lrc<SourceMap> = Default::default();
    let source_file = source_map.new_source_file(
        Lrc::new(FileName::Custom(source_name.to_string())),
        source.to_string(),
    );
    let lexer = Lexer::new(
        Syntax::Es(EsSyntax {
            jsx: true,
            decorators: true,
            ..Default::default()
        }),
        EsVersion::Es2022,
        StringInput::from(&*source_file),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().map_err(|error| {
        GuiError::invalid_tree(format!(
            "invalid RSX syntax in {source_name:?}: {}",
            error.kind().msg()
        ))
    })?;

    if let Some(error) = parser.take_errors().into_iter().next() {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX syntax in {source_name:?}: {}",
            error.kind().msg()
        )));
    }

    Ok(module)
}

fn normalize_rsx_source_name(source_name: impl AsRef<str>) -> String {
    let source_name = source_name.as_ref().trim();
    if source_name.is_empty() {
        "inline.rsx".to_string()
    } else {
        source_name.to_string()
    }
}

struct RsxFunctionComponent {
    root_expression: String,
}

fn extract_rsx_function_component(
    source_name: &str,
    source: &str,
) -> GuiResult<Option<RsxFunctionComponent>> {
    if contains_javascript_component_syntax(source) {
        return Err(GuiError::invalid_tree(
            "RSX files use Rust-style view templates; write `fn View(props: ViewProps) -> RSX { (...) }`, not JavaScript modules, exports, or functions",
        ));
    }

    let Some(fn_start) = find_rust_fn_keyword(source) else {
        return Ok(None);
    };

    validate_only_leading_ws_before(source, fn_start, source_name)?;
    let mut cursor = fn_start + "fn".len();
    cursor = skip_whitespace(source, cursor);
    let Some((_name, after_name)) = read_identifier(source, cursor) else {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: expected a component name after `fn`"
        )));
    };
    cursor = skip_whitespace(source, after_name);
    if !source[cursor..].starts_with('(') {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: expected props parameter list"
        )));
    }

    let Some(params_end) = find_matching_delimiter(source, cursor, '(', ')') else {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: unclosed props parameter list"
        )));
    };
    validate_rsx_component_params(source_name, &source[cursor + 1..params_end])?;

    cursor = skip_whitespace(source, params_end + 1);
    if !source[cursor..].starts_with("->") {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: expected `-> RSX` return type"
        )));
    }
    cursor = skip_whitespace(source, cursor + "->".len());

    let return_type_start = cursor;
    while cursor < source.len() {
        let Some(ch) = source[cursor..].chars().next() else {
            break;
        };
        if ch == '{' {
            break;
        }
        cursor += ch.len_utf8();
    }
    let return_type = source[return_type_start..cursor].trim();
    if return_type != "RSX" {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: expected return type `RSX`, found `{return_type}`"
        )));
    }

    cursor = skip_whitespace(source, cursor);
    if !source[cursor..].starts_with('{') {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: expected component body"
        )));
    }
    let Some(body_end) = find_matching_delimiter(source, cursor, '{', '}') else {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: unclosed component body"
        )));
    };
    validate_only_trailing_ws_after(source, body_end + 1, source_name)?;

    let body = &source[cursor + 1..body_end];
    let root_expression = extract_rsx_return_expression(source_name, body)?;

    Ok(Some(RsxFunctionComponent { root_expression }))
}

fn keyword_at(source: &str, index: usize, keyword: &str) -> bool {
    source[index..].starts_with(keyword)
        && source[..index]
            .chars()
            .next_back()
            .is_none_or(|ch| !is_ident_char(ch))
        && source[index + keyword.len()..]
            .chars()
            .next()
            .is_none_or(|ch| !is_ident_char(ch))
}

fn is_ident_char(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn contains_javascript_component_syntax(source: &str) -> bool {
    first_code_token(source).is_some_and(|token| matches!(token, "export" | "import" | "function"))
}

fn first_code_token(source: &str) -> Option<&str> {
    let source = source.trim_start();
    if source.is_empty() {
        return None;
    }
    let end = source
        .char_indices()
        .find_map(|(index, ch)| (!is_ident_char(ch)).then_some(index))
        .unwrap_or(source.len());
    Some(&source[..end])
}

fn find_rust_fn_keyword(source: &str) -> Option<usize> {
    let mut index = 0;
    while index < source.len() {
        if keyword_at(source, index, "fn") {
            return Some(index);
        }
        if keyword_at(source, index, "pub") {
            let after_pub = skip_whitespace(source, index + "pub".len());
            if keyword_at(source, after_pub, "fn") {
                return Some(after_pub);
            }
        }
        index += source[index..].chars().next()?.len_utf8();
    }
    None
}

fn validate_only_leading_ws_before(
    source: &str,
    fn_start: usize,
    source_name: &str,
) -> GuiResult<()> {
    let leading = source[..fn_start].trim();
    if leading.is_empty() || leading == "pub" {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "invalid RSX view template in {source_name:?}: only one top-level `fn View(props: Props) -> RSX` wrapper is supported"
        )))
    }
}

fn validate_only_trailing_ws_after(source: &str, index: usize, source_name: &str) -> GuiResult<()> {
    if source[index..].trim().is_empty() {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "invalid RSX component in {source_name:?}: unexpected content after component body"
        )))
    }
}

fn read_identifier(source: &str, index: usize) -> Option<(&str, usize)> {
    let first = source[index..].chars().next()?;
    if first != '_' && !first.is_ascii_alphabetic() {
        return None;
    }
    let mut end = index + first.len_utf8();
    while end < source.len() {
        let ch = source[end..].chars().next()?;
        if !is_ident_char(ch) {
            break;
        }
        end += ch.len_utf8();
    }
    Some((&source[index..end], end))
}

fn validate_rsx_component_params(source_name: &str, params: &str) -> GuiResult<()> {
    for param in params.split(',') {
        let param = param.trim();
        if param.is_empty() {
            continue;
        }
        let param = param.strip_prefix("mut ").unwrap_or(param).trim();
        let name = param.split_once(':').map(|(name, _)| name).unwrap_or(param);
        let name = name.trim();
        if read_identifier(name, 0).is_none_or(|(_, end)| end != name.len()) {
            return Err(GuiError::invalid_tree(format!(
                "invalid RSX component in {source_name:?}: parameter `{param}` must be an identifier or typed identifier"
            )));
        }
    }
    Ok(())
}

fn extract_rsx_return_expression(source_name: &str, body: &str) -> GuiResult<String> {
    let body = body.trim();
    if keyword_at(body, 0, "return") {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX view template in {source_name:?}: view-template body must be an RSX expression; remove `return`"
        )));
    }
    let expression = body;
    let expression = expression.strip_suffix(';').unwrap_or(expression).trim();
    if expression.is_empty() {
        return Err(GuiError::invalid_tree(format!(
            "invalid RSX view template in {source_name:?}: view-template body must return an RSX element or fragment"
        )));
    }
    Ok(expression.to_string())
}

fn find_matching_delimiter(
    source: &str,
    mut index: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if let Some(quote_ch) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_ch {
                quote = None;
            }
            index += ch.len_utf8();
            continue;
        }

        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            ch if ch == open => depth += 1,
            ch if ch == close => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += ch.len_utf8();
    }
    None
}

fn skip_whitespace(source: &str, mut index: usize) -> usize {
    while index < source.len() {
        let ch = source[index..].chars().next().unwrap();
        if !ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}

fn with_source_context(source_name: &str, error: GuiError) -> GuiError {
    match error {
        GuiError::InvalidTree { message } if message.contains(source_name) => {
            GuiError::invalid_tree(message)
        }
        GuiError::InvalidTree { message } => {
            GuiError::invalid_tree(format!("RSX source {source_name:?}: {message}"))
        }
        other => other,
    }
}

fn find_root_expr(module: &ast::Module) -> GuiResult<&ast::Expr> {
    let mut expression_roots = Vec::new();
    let mut variable_roots = Vec::new();

    for item in &module.body {
        match item {
            ast::ModuleItem::ModuleDecl(_) => {
                return Err(GuiError::invalid_tree(
                    "RSX files do not use JavaScript module exports; write `fn View(props: ViewProps) -> RSX { (...) }`",
                ));
            }
            ast::ModuleItem::Stmt(ast::Stmt::Expr(expr)) => {
                if is_rsx_root_expr(&expr.expr) {
                    expression_roots.push(expr.expr.as_ref());
                }
            }
            ast::ModuleItem::Stmt(ast::Stmt::Decl(decl)) => {
                if matches!(decl, ast::Decl::Fn(_)) {
                    return Err(GuiError::invalid_tree(
                        "RSX files use Rust-style view templates; write `fn View(props: ViewProps) -> RSX { (...) }`, not JavaScript `function` declarations",
                    ));
                }
                collect_variable_roots_from_decl(decl, &mut variable_roots);
            }
            _ => {}
        }
    }

    let mut roots = expression_roots;
    roots.extend(variable_roots);
    match roots.len() {
        0 => Err(GuiError::invalid_tree(
            "expected an RSX root expression, variable initializer, or `fn View(props: Props) -> RSX` view-template expression body",
        )),
        1 => Ok(roots[0]),
        _ => Err(GuiError::invalid_tree(
            "RSX source has more than one possible root expression",
        )),
    }
}

fn collect_variable_roots_from_decl<'a>(decl: &'a ast::Decl, roots: &mut Vec<&'a ast::Expr>) {
    if let ast::Decl::Var(var_decl) = decl {
        for declarator in &var_decl.decls {
            let Some(init) = declarator.init.as_deref() else {
                continue;
            };
            if is_rsx_root_expr(init) {
                roots.push(init);
            }
        }
    }
}

fn is_rsx_root_expr(expr: &ast::Expr) -> bool {
    matches!(
        unwrap_static_expr(expr),
        ast::Expr::JSXElement(_) | ast::Expr::JSXFragment(_)
    )
}

fn lower_root_expr(expr: &ast::Expr) -> GuiResult<CompiledRsxNode> {
    let scope = LoweringScope::default();
    match unwrap_static_expr(expr) {
        ast::Expr::JSXElement(element) => lower_rsx_element(element, "root", &scope),
        ast::Expr::JSXFragment(fragment) => lower_rsx_fragment(fragment, "root", &scope),
        _ => Err(unsupported_dynamic("root expression")),
    }
}

#[derive(Clone, Default)]
struct LoweringScope {
    locals: BTreeSet<String>,
    allow_local_dynamic_keys: bool,
}

impl LoweringScope {
    fn with_local(&self, name: impl Into<String>) -> Self {
        let mut next = self.clone();
        next.locals.insert(name.into());
        next
    }

    fn with_local_dynamic_keys(&self) -> Self {
        let mut next = self.clone();
        next.allow_local_dynamic_keys = true;
        next
    }

    fn has_local(&self, name: &str) -> bool {
        self.locals.contains(name)
    }

    fn allows_dynamic_key(&self, binding: &CompiledBinding) -> bool {
        self.allow_local_dynamic_keys && binding.source == CompiledBindingSource::Local
    }
}

fn lower_rsx_element(
    element: &ast::JSXElement,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<CompiledRsxNode> {
    let tag = rsx_element_name(&element.opening.name);
    let mut key = None;
    let mut props = CompiledProps::default();

    for attr in &element.opening.attrs {
        match attr {
            ast::JSXAttrOrSpread::JSXAttr(attr) => {
                let name = rsx_attr_name(&attr.name);
                let value = lower_attr_value(&name, attr.value.as_ref(), scope)?;
                if name == "key" {
                    key = Some(match value {
                        AttributeValue::Binding(binding) if scope.allows_dynamic_key(&binding) => {
                            fallback_key.to_string()
                        }
                        value => value.into_key()?,
                    });
                } else {
                    apply_attribute(&mut props, &name, value);
                }
            }
            ast::JSXAttrOrSpread::SpreadElement(spread) => {
                let Some(binding) = binding_ref(&spread.expr, scope) else {
                    return Err(GuiError::invalid_tree(
                        "RSX spread attributes must be state/props/derived/context/resource/local object bindings such as {...props.button}",
                    ));
                };
                props.spreads.push(binding);
            }
        }
    }

    let child_scope = scope_for_element_children(&tag, &props, scope)?;
    let children = lower_rsx_children(&element.children, &child_scope)?;

    Ok(CompiledRsxNode::Element {
        key: key.unwrap_or_else(|| fallback_key.to_string()),
        tag,
        import_source: None,
        props,
        children,
    })
}

fn lower_rsx_fragment(
    fragment: &ast::JSXFragment,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<CompiledRsxNode> {
    Ok(CompiledRsxNode::Element {
        key: fallback_key.to_string(),
        tag: "Fragment".to_string(),
        import_source: None,
        props: CompiledProps::default(),
        children: lower_rsx_children(&fragment.children, scope)?,
    })
}

fn scope_for_element_children(
    tag: &str,
    props: &CompiledProps,
    scope: &LoweringScope,
) -> GuiResult<LoweringScope> {
    if !matches!(tag, "For" | "Each") {
        return Ok(scope.clone());
    }

    let item_name = static_identifier_prop(props, "as")?.unwrap_or("item");
    let mut scope = scope.with_local(item_name);
    if let Some(index_name) = static_identifier_prop(props, "indexAs")? {
        if index_name == item_name {
            return Err(GuiError::invalid_tree(
                "RSX <For> indexAs cannot reuse the item variable name",
            ));
        }
        scope = scope.with_local(index_name);
    }
    Ok(scope)
}

fn static_identifier_prop<'a>(props: &'a CompiledProps, name: &str) -> GuiResult<Option<&'a str>> {
    if props.bindings.contains_key(name) {
        return Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a static identifier"
        )));
    }
    let Some(value) = props.attributes.get(name) else {
        return Ok(None);
    };
    if is_valid_local_identifier(value) {
        Ok(Some(value.as_str()))
    } else {
        Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a valid identifier"
        )))
    }
}

fn lower_rsx_children(
    children: &[ast::JSXElementChild],
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let mut lowered = Vec::new();
    let mut text_index = 0usize;
    let mut element_index = 0usize;

    for child in children {
        let fallback_key = match child {
            ast::JSXElementChild::JSXText(_) | ast::JSXElementChild::JSXExprContainer(_) => {
                let key = format!("text-{text_index}");
                text_index += 1;
                key
            }
            _ => {
                let key = format!("child-{element_index}");
                element_index += 1;
                key
            }
        };

        lowered.extend(lower_rsx_child(child, &fallback_key, scope)?);
    }

    Ok(lowered)
}

fn lower_rsx_child(
    child: &ast::JSXElementChild,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    match child {
        ast::JSXElementChild::JSXText(text) => {
            let normalized = normalize_rsx_text(text.value.as_ref());
            if normalized.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(vec![CompiledRsxNode::Text {
                    key: fallback_key.to_string(),
                    value: normalized,
                }])
            }
        }
        ast::JSXElementChild::JSXExprContainer(container) => match &container.expr {
            ast::JSXExpr::JSXEmptyExpr(_) => Ok(Vec::new()),
            ast::JSXExpr::Expr(expr) => lower_child_expr(expr, fallback_key, scope),
        },
        ast::JSXElementChild::JSXSpreadChild(_) => Err(GuiError::invalid_tree(
            "RSX spread children are dynamic and are not supported by the static native UI compiler",
        )),
        ast::JSXElementChild::JSXElement(element) => {
            Ok(vec![lower_rsx_element(element, fallback_key, scope)?])
        }
        ast::JSXElementChild::JSXFragment(fragment) => Ok(vec![lower_rsx_fragment(
            fragment,
            fallback_key,
            scope,
        )?]),
    }
}

fn lower_child_expr(
    expr: &ast::Expr,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    match unwrap_static_expr(expr) {
        ast::Expr::JSXElement(element) => {
            Ok(vec![lower_rsx_element(element, fallback_key, scope)?])
        }
        ast::Expr::JSXFragment(fragment) => {
            Ok(vec![lower_rsx_fragment(fragment, fallback_key, scope)?])
        }
        ast::Expr::Lit(ast::Lit::Str(value)) => Ok(vec![CompiledRsxNode::Text {
            key: fallback_key.to_string(),
            value: swc_string(&value.value),
        }]),
        ast::Expr::Lit(ast::Lit::Num(value)) => Ok(vec![CompiledRsxNode::Text {
            key: fallback_key.to_string(),
            value: number_to_string(value.value),
        }]),
        ast::Expr::Lit(ast::Lit::Bool(_)) | ast::Expr::Lit(ast::Lit::Null(_)) => Ok(Vec::new()),
        ast::Expr::Tpl(template) if template.exprs.is_empty() => Ok(vec![CompiledRsxNode::Text {
            key: fallback_key.to_string(),
            value: template
                .quasis
                .first()
                .and_then(|quasi| quasi.cooked.as_ref())
                .map(swc_string)
                .unwrap_or_default(),
        }]),
        ast::Expr::Bin(binary) if binary.op == ast::BinaryOp::LogicalAnd => {
            lower_logical_and_child(binary, fallback_key, scope)
        }
        ast::Expr::Cond(conditional) => lower_conditional_child(conditional, fallback_key, scope),
        ast::Expr::Call(call) => lower_map_child(call, fallback_key, scope),
        _ if let Some(binding) = binding_ref(expr, scope) => {
            let mut props = CompiledProps::default();
            props.bindings.insert("textValue".to_string(), binding);
            Ok(vec![CompiledRsxNode::Element {
                key: fallback_key.to_string(),
                tag: "Text".to_string(),
                import_source: None,
                props,
                children: Vec::new(),
            }])
        }
        _ => Err(unsupported_dynamic("RSX child expression")),
    }
}

fn lower_logical_and_child(
    binary: &ast::BinExpr,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let condition = control_condition_from_expr(&binary.left, scope)?;
    let children = lower_child_expr(&binary.right, &format!("{fallback_key}-then"), scope)?;
    if children.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(vec![show_control_node(fallback_key, condition, children)])
    }
}

fn lower_conditional_child(
    conditional: &ast::CondExpr,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let condition = control_condition_from_expr(&conditional.test, scope)?;
    let when_children =
        lower_child_expr(&conditional.cons, &format!("{fallback_key}-when"), scope)?;
    let unless_children =
        lower_child_expr(&conditional.alt, &format!("{fallback_key}-unless"), scope)?;
    let mut nodes = Vec::new();

    if !when_children.is_empty() {
        nodes.push(show_control_node(
            &format!("{fallback_key}-when"),
            condition.clone(),
            when_children,
        ));
    }
    if !unless_children.is_empty() {
        nodes.push(show_control_node(
            &format!("{fallback_key}-unless"),
            condition.inverted(),
            unless_children,
        ));
    }

    Ok(nodes)
}

fn lower_map_child(
    call: &ast::CallExpr,
    fallback_key: &str,
    scope: &LoweringScope,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let Some(map) = map_call_parts(call, scope)? else {
        return Err(unsupported_dynamic("RSX map expression"));
    };

    let mut props = CompiledProps::default();
    props.bindings.insert("each".to_string(), map.each);
    props
        .attributes
        .insert("as".to_string(), map.item_name.clone());
    if let Some(index_name) = &map.index_name {
        props
            .attributes
            .insert("indexAs".to_string(), index_name.clone());
    }
    if let Some(key_by) = &map.key_by {
        props.attributes.insert("keyBy".to_string(), key_by.clone());
    }

    let mut item_scope = scope.with_local(&map.item_name);
    if let Some(index_name) = &map.index_name {
        item_scope = item_scope.with_local(index_name);
    }
    if map.key_by.is_some() {
        item_scope = item_scope.with_local_dynamic_keys();
    }
    let children = lower_child_expr(&map.body, &format!("{fallback_key}-item"), &item_scope)?;

    Ok(vec![CompiledRsxNode::Element {
        key: fallback_key.to_string(),
        tag: "For".to_string(),
        import_source: None,
        props,
        children,
    }])
}

struct MapCallParts<'a> {
    each: CompiledBinding,
    item_name: String,
    index_name: Option<String>,
    key_by: Option<String>,
    body: &'a ast::Expr,
}

fn map_call_parts<'a>(
    call: &'a ast::CallExpr,
    scope: &LoweringScope,
) -> GuiResult<Option<MapCallParts<'a>>> {
    if call.args.len() != 1 || call.args[0].spread.is_some() {
        return Ok(None);
    }

    let ast::Callee::Expr(callee) = &call.callee else {
        return Ok(None);
    };
    let ast::Expr::Member(member) = unwrap_static_expr(callee) else {
        return Ok(None);
    };
    let ast::MemberProp::Ident(prop) = &member.prop else {
        return Ok(None);
    };
    if prop.sym.as_ref() != "map" {
        return Ok(None);
    }
    let Some(each) = binding_ref(&member.obj, scope) else {
        return Ok(None);
    };

    let ast::Expr::Arrow(arrow) = unwrap_static_expr(&call.args[0].expr) else {
        return Ok(None);
    };
    if arrow.is_async || arrow.is_generator || arrow.params.is_empty() || arrow.params.len() > 2 {
        return Ok(None);
    }
    let Some(item_name) = pat_identifier(&arrow.params[0]) else {
        return Ok(None);
    };
    validate_local_identifier(&item_name)?;
    let index_name = arrow.params.get(1).and_then(pat_identifier);
    if let Some(index_name) = &index_name {
        validate_local_identifier(index_name)?;
        if index_name == &item_name {
            return Err(GuiError::invalid_tree(
                "RSX map index parameter cannot reuse the item variable name",
            ));
        }
    }
    let ast::BlockStmtOrExpr::Expr(body) = arrow.body.as_ref() else {
        return Ok(None);
    };

    let scoped = scope.with_local(&item_name);
    let key_by = local_key_by_from_expr(body, &item_name, &scoped)?;

    Ok(Some(MapCallParts {
        each,
        item_name,
        index_name,
        key_by,
        body,
    }))
}

fn pat_identifier(pat: &ast::Pat) -> Option<String> {
    match pat {
        ast::Pat::Ident(ident) => Some(ident.id.sym.to_string()),
        _ => None,
    }
}

fn local_key_by_from_expr(
    expr: &ast::Expr,
    item_name: &str,
    scope: &LoweringScope,
) -> GuiResult<Option<String>> {
    let ast::Expr::JSXElement(element) = unwrap_static_expr(expr) else {
        return Ok(None);
    };

    for attr in &element.opening.attrs {
        let ast::JSXAttrOrSpread::JSXAttr(attr) = attr else {
            continue;
        };
        if rsx_attr_name(&attr.name) != "key" {
            continue;
        }
        let value = lower_attr_value("key", attr.value.as_ref(), scope)?;
        let AttributeValue::Binding(binding) = value else {
            return Ok(None);
        };
        if binding.source != CompiledBindingSource::Local
            || binding.path.first().map(String::as_str) != Some(item_name)
        {
            return Err(GuiError::invalid_tree(
                "RSX map key must use the map item, such as key={item.id}",
            ));
        }
        return if binding.path.len() == 1 {
            Ok(Some(".".to_string()))
        } else {
            Ok(Some(binding.path[1..].join(".")))
        };
    }

    Ok(None)
}

fn show_control_node(
    key: &str,
    condition: ControlCondition,
    children: Vec<CompiledRsxNode>,
) -> CompiledRsxNode {
    let mut props = CompiledProps::default();
    let (attribute, value) = condition.into_attribute();
    apply_attribute(&mut props, attribute, value);
    CompiledRsxNode::Element {
        key: key.to_string(),
        tag: "Show".to_string(),
        import_source: None,
        props,
        children,
    }
}

fn lower_attr_value(
    name: &str,
    value: Option<&ast::JSXAttrValue>,
    scope: &LoweringScope,
) -> GuiResult<AttributeValue> {
    match value {
        None => Ok(AttributeValue::Bool(true)),
        Some(ast::JSXAttrValue::Str(value)) => Ok(AttributeValue::String(swc_string(&value.value))),
        Some(ast::JSXAttrValue::JSXExprContainer(container)) => match &container.expr {
            ast::JSXExpr::JSXEmptyExpr(_) => Err(GuiError::invalid_tree(format!(
                "RSX attribute {name:?} has an empty expression"
            ))),
            ast::JSXExpr::Expr(expr) => lower_attr_expr(name, expr, scope),
        },
        Some(ast::JSXAttrValue::JSXElement(_)) | Some(ast::JSXAttrValue::JSXFragment(_)) => Err(
            GuiError::invalid_tree(format!(
                "RSX attribute {name:?} uses a nested element value; only static string, boolean, number, event action references, or state/props/derived/context/resource bindings are supported"
            )),
        ),
    }
}

fn lower_attr_expr(
    name: &str,
    expr: &ast::Expr,
    scope: &LoweringScope,
) -> GuiResult<AttributeValue> {
    let expr = unwrap_static_expr(expr);
    match expr {
        ast::Expr::Lit(ast::Lit::Str(value)) => {
            Ok(AttributeValue::String(swc_string(&value.value)))
        }
        ast::Expr::Lit(ast::Lit::Bool(value)) => Ok(AttributeValue::Bool(value.value)),
        ast::Expr::Lit(ast::Lit::Num(value)) => Ok(AttributeValue::Number(value.value)),
        ast::Expr::Tpl(template) if template.exprs.is_empty() => Ok(AttributeValue::String(
            template
                .quasis
                .first()
                .and_then(|quasi| quasi.cooked.as_ref())
                .map(swc_string)
                .unwrap_or_default(),
        )),
        ast::Expr::Unary(unary) => lower_unary_attr_expr(name, unary),
        _ if name.starts_with("on") => {
            if let Some(binding) = binding_ref(expr, scope) {
                Ok(AttributeValue::Binding(binding))
            } else {
                event_action_ref(expr)
                    .map(AttributeValue::String)
                    .ok_or_else(|| {
                        GuiError::invalid_tree(format!(
                            "RSX event handler {name:?} must be an action identifier such as {name}={{saveDocument}}"
                        ))
                    })
            }
        }
        _ if let Some(binding) = binding_ref(expr, scope) => Ok(AttributeValue::Binding(binding)),
        _ => Err(unsupported_dynamic(format!("attribute {name:?}"))),
    }
}

fn lower_unary_attr_expr(name: &str, unary: &ast::UnaryExpr) -> GuiResult<AttributeValue> {
    let ast::Expr::Lit(ast::Lit::Num(number)) = unwrap_static_expr(&unary.arg) else {
        return Err(unsupported_dynamic(format!("attribute {name:?}")));
    };

    match unary.op {
        ast::UnaryOp::Minus => Ok(AttributeValue::Number(-number.value)),
        ast::UnaryOp::Plus => Ok(AttributeValue::Number(number.value)),
        _ => Err(unsupported_dynamic(format!("attribute {name:?}"))),
    }
}

fn event_action_ref(expr: &ast::Expr) -> Option<String> {
    match unwrap_static_expr(expr) {
        ast::Expr::Ident(ident) => Some(ident.sym.to_string()),
        ast::Expr::Member(member) => {
            let object = event_action_ref(&member.obj)?;
            match &member.prop {
                ast::MemberProp::Ident(prop) => Some(format!("{object}.{}", prop.sym)),
                _ => None,
            }
        }
        _ => None,
    }
}

fn binding_ref(expr: &ast::Expr, scope: &LoweringScope) -> Option<CompiledBinding> {
    let mut segments = member_path(expr)?;
    if segments.is_empty() {
        return None;
    }
    let root = segments[0].clone();
    let source = match root.as_str() {
        "state" | "props" | "derived" | "context" | "resource" if segments.len() < 2 => {
            return None;
        }
        "state" => {
            segments.remove(0);
            CompiledBindingSource::State
        }
        "props" => {
            segments.remove(0);
            CompiledBindingSource::Props
        }
        "derived" => {
            segments.remove(0);
            CompiledBindingSource::Derived
        }
        "context" => {
            segments.remove(0);
            CompiledBindingSource::Context
        }
        "resource" => {
            segments.remove(0);
            CompiledBindingSource::Resource
        }
        local if scope.has_local(local) => CompiledBindingSource::Local,
        _ => return None,
    };
    Some(CompiledBinding {
        source,
        path: segments,
    })
}

fn member_path(expr: &ast::Expr) -> Option<Vec<String>> {
    match unwrap_static_expr(expr) {
        ast::Expr::Ident(ident) => Some(vec![ident.sym.to_string()]),
        ast::Expr::Member(member) => {
            let mut path = member_path(&member.obj)?;
            match &member.prop {
                ast::MemberProp::Ident(prop) => path.push(prop.sym.to_string()),
                ast::MemberProp::Computed(prop) => {
                    path.push(static_computed_member_segment(&prop.expr)?)
                }
                _ => return None,
            }
            Some(path)
        }
        _ => None,
    }
}

fn static_computed_member_segment(expr: &ast::Expr) -> Option<String> {
    let segment = match unwrap_static_expr(expr) {
        ast::Expr::Lit(ast::Lit::Str(value)) => swc_string(&value.value),
        ast::Expr::Lit(ast::Lit::Num(value))
            if value.value.is_finite() && value.value >= 0.0 && value.value.fract() == 0.0 =>
        {
            number_to_string(value.value)
        }
        ast::Expr::Tpl(template) if template.exprs.is_empty() => template
            .quasis
            .first()
            .and_then(|quasi| quasi.cooked.as_ref())
            .map(swc_string)
            .unwrap_or_default(),
        _ => return None,
    };
    if segment.is_empty() {
        None
    } else {
        Some(segment)
    }
}

fn unwrap_static_expr(expr: &ast::Expr) -> &ast::Expr {
    match expr {
        ast::Expr::Paren(paren) => unwrap_static_expr(&paren.expr),
        ast::Expr::TsAs(ts) => unwrap_static_expr(&ts.expr),
        ast::Expr::TsSatisfies(ts) => unwrap_static_expr(&ts.expr),
        ast::Expr::TsNonNull(ts) => unwrap_static_expr(&ts.expr),
        ast::Expr::TsConstAssertion(ts) => unwrap_static_expr(&ts.expr),
        ast::Expr::TsTypeAssertion(ts) => unwrap_static_expr(&ts.expr),
        ast::Expr::TsInstantiation(ts) => unwrap_static_expr(&ts.expr),
        _ => expr,
    }
}

fn rsx_element_name(name: &ast::JSXElementName) -> String {
    match name {
        ast::JSXElementName::Ident(ident) => ident.sym.to_string(),
        ast::JSXElementName::JSXMemberExpr(member) => rsx_member_expr_name(member),
        ast::JSXElementName::JSXNamespacedName(name) => {
            format!("{}:{}", name.ns.sym, name.name.sym)
        }
    }
}

fn rsx_member_expr_name(member: &ast::JSXMemberExpr) -> String {
    let object = match &member.obj {
        ast::JSXObject::Ident(ident) => ident.sym.to_string(),
        ast::JSXObject::JSXMemberExpr(member) => rsx_member_expr_name(member),
    };
    format!("{object}.{}", member.prop.sym)
}

fn rsx_attr_name(name: &ast::JSXAttrName) -> String {
    match name {
        ast::JSXAttrName::Ident(ident) => ident.sym.to_string(),
        ast::JSXAttrName::JSXNamespacedName(name) => {
            format!("{}:{}", name.ns.sym, name.name.sym)
        }
    }
}

fn swc_string(value: &swc_atoms::Wtf8Atom) -> String {
    value.as_str().unwrap_or("").to_string()
}

fn number_to_string(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn unsupported_dynamic(context: impl Into<String>) -> GuiError {
    GuiError::invalid_tree(format!(
        "dynamic RSX {} is not supported by the static native UI compiler; use static literals, event action identifiers, or state/props/derived/context/resource member bindings such as {{state.title}}",
        context.into()
    ))
}

#[derive(Clone)]
enum ControlCondition {
    When(AttributeValue),
    Unless(AttributeValue),
}

impl ControlCondition {
    fn inverted(&self) -> Self {
        match self {
            Self::When(value) => Self::Unless(value.clone()),
            Self::Unless(value) => Self::When(value.clone()),
        }
    }

    fn into_attribute(self) -> (&'static str, AttributeValue) {
        match self {
            Self::When(value) => ("when", value),
            Self::Unless(value) => ("unless", value),
        }
    }
}

fn control_condition_from_expr(
    expr: &ast::Expr,
    scope: &LoweringScope,
) -> GuiResult<ControlCondition> {
    match unwrap_static_expr(expr) {
        ast::Expr::Lit(ast::Lit::Bool(value)) => {
            Ok(ControlCondition::When(AttributeValue::Bool(value.value)))
        }
        ast::Expr::Unary(unary) if unary.op == ast::UnaryOp::Bang => {
            control_condition_from_negated_expr(&unary.arg, scope)
        }
        _ if let Some(binding) = binding_ref(expr, scope) => {
            Ok(ControlCondition::When(AttributeValue::Binding(binding)))
        }
        _ => Err(unsupported_dynamic("RSX control condition")),
    }
}

fn control_condition_from_negated_expr(
    expr: &ast::Expr,
    scope: &LoweringScope,
) -> GuiResult<ControlCondition> {
    match unwrap_static_expr(expr) {
        ast::Expr::Lit(ast::Lit::Bool(value)) => {
            Ok(ControlCondition::When(AttributeValue::Bool(!value.value)))
        }
        _ if let Some(binding) = binding_ref(expr, scope) => {
            Ok(ControlCondition::Unless(AttributeValue::Binding(binding)))
        }
        _ => Err(unsupported_dynamic("RSX control condition")),
    }
}

fn validate_local_identifier(value: &str) -> GuiResult<()> {
    if is_valid_local_identifier(value) {
        Ok(())
    } else {
        Err(GuiError::invalid_tree(format!(
            "RSX local name {value:?} must be a valid identifier"
        )))
    }
}

fn is_valid_local_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if matches!(
        value,
        "state" | "props" | "derived" | "context" | "resource"
    ) {
        return false;
    }
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

#[derive(Clone)]
enum AttributeValue {
    String(String),
    Bool(bool),
    Number(f64),
    Binding(CompiledBinding),
}

impl AttributeValue {
    fn into_string(self) -> String {
        match self {
            Self::String(value) => value,
            Self::Bool(value) => value.to_string(),
            Self::Number(value) => number_to_string(value),
            Self::Binding(binding) => binding.display_path(),
        }
    }

    fn into_key(self) -> GuiResult<String> {
        match self {
            Self::Binding(binding) => Err(GuiError::invalid_tree(format!(
                "RSX key cannot be dynamic; binding {} is not supported for keyed identity",
                binding.display_path()
            ))),
            value => Ok(value.into_string()),
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            Self::String(value) if value == "true" => Some(true),
            Self::String(value) if value == "false" => Some(false),
            _ => None,
        }
    }

    fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            Self::String(value) => value.parse().ok(),
            Self::Bool(_) | Self::Binding(_) => None,
        }
    }
}

fn apply_attribute(props: &mut CompiledProps, name: &str, value: AttributeValue) {
    props.explicit_props.insert(canonical_prop_name(name));
    if let AttributeValue::Binding(binding) = value {
        props
            .bindings
            .insert(canonical_binding_property(name), binding);
        return;
    }

    match name {
        "class" | "className" => props.class_name = Some(value.into_string()),
        "label" => props.label = Some(value.into_string()),
        "textValue" => props.text_value = Some(value.into_string()),
        "value" => props.value = Some(value.into_string()),
        "placeholder" => props.placeholder = Some(value.into_string()),
        "action" => props.action = Some(value.into_string()),
        "aria-label" | "ariaLabel" => props.aria_label = Some(value.into_string()),
        "id" => props.id = Some(value.into_string()),
        "name" => props.name = Some(value.into_string()),
        "form" => props.form = Some(value.into_string()),
        "type" | "inputType" => props.input_type = Some(value.into_string()),
        "orientation" => {
            props.orientation = match value.into_string().as_str() {
                "horizontal" => Some(CompiledOrientation::Horizontal),
                "vertical" => Some(CompiledOrientation::Vertical),
                _ => None,
            };
        }
        "isDisabled" | "disabled" => props.is_disabled = value.as_bool().unwrap_or(false),
        "aria-disabled" => {
            props.is_disabled = value.as_bool().unwrap_or(false);
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isRequired" | "required" => props.is_required = value.as_bool().unwrap_or(false),
        "aria-required" => {
            props.is_required = value.as_bool().unwrap_or(false);
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isInvalid" | "invalid" => props.is_invalid = value.as_bool().unwrap_or(false),
        "aria-invalid" => {
            props.is_invalid = value.as_bool().unwrap_or(false);
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isReadOnly" | "readOnly" | "readonly" => {
            props.is_read_only = value.as_bool().unwrap_or(false)
        }
        "aria-readonly" => {
            props.is_read_only = value.as_bool().unwrap_or(false);
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isSelected" | "selected" => props.is_selected = value.as_bool().unwrap_or(false),
        "aria-selected" => {
            props.is_selected = value.as_bool().unwrap_or(false);
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isChecked" | "checked" => props.is_checked = value.as_bool(),
        "aria-checked" => {
            props.is_checked = value.as_bool();
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "isExpanded" | "expanded" => props.is_expanded = value.as_bool(),
        "aria-expanded" => {
            props.is_expanded = value.as_bool();
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "min" | "minValue" => props.min_value = value.as_number(),
        "max" | "maxValue" => props.max_value = value.as_number(),
        "step" | "stepValue" => props.step_value = value.as_number(),
        "valueNumber" => props.value_number = value.as_number(),
        name if name.starts_with("on") => {
            props
                .events
                .insert(normalize_event_name(name), value.into_string());
        }
        name if name.starts_with("aria-") || name.starts_with("data-") => {
            props
                .attributes
                .insert(name.to_string(), value.into_string());
        }
        "style" => {
            props
                .style
                .extend(parse_style_text(&value.into_string()).unwrap_or_default());
        }
        other => {
            props
                .attributes
                .insert(other.to_string(), value.into_string());
        }
    }
}

fn canonical_binding_property(name: &str) -> String {
    canonical_prop_name(name)
}

fn canonical_prop_name(name: &str) -> String {
    match name {
        "class" | "className" => "className".to_string(),
        "aria-label" | "ariaLabel" => "aria-label".to_string(),
        "disabled" | "aria-disabled" | "isDisabled" => "isDisabled".to_string(),
        "required" | "aria-required" | "isRequired" => "isRequired".to_string(),
        "invalid" | "aria-invalid" | "isInvalid" => "isInvalid".to_string(),
        "readOnly" | "readonly" | "aria-readonly" | "isReadOnly" => "isReadOnly".to_string(),
        "selected" | "aria-selected" | "isSelected" => "isSelected".to_string(),
        "checked" | "aria-checked" | "isChecked" => "isChecked".to_string(),
        "expanded" | "aria-expanded" | "isExpanded" => "isExpanded".to_string(),
        "min" | "minValue" => "minValue".to_string(),
        "max" | "maxValue" => "maxValue".to_string(),
        "step" | "stepValue" => "stepValue".to_string(),
        "type" | "inputType" => "inputType".to_string(),
        other if other.starts_with("on") => normalize_event_name(other),
        other => other.to_string(),
    }
}

fn normalize_event_name(name: &str) -> String {
    match name {
        "onclick" => "onClick",
        "onpress" => "onPress",
        "onchange" => "onChange",
        "oninput" => "onInput",
        "onselectionchange" => "onSelectionChange",
        "onfocus" => "onFocus",
        "onblur" => "onBlur",
        "onfocuschange" => "onFocusChange",
        "ontoggle" => "onToggle",
        "onexpandedchange" => "onExpandedChange",
        "onhoverstart" => "onHoverStart",
        "onhoverend" => "onHoverEnd",
        "onhoverchange" => "onHoverChange",
        "onkeydown" => "onKeyDown",
        "onkeyup" => "onKeyUp",
        "oncopy" => "onCopy",
        "oncut" => "onCut",
        "onpaste" => "onPaste",
        _ => name,
    }
    .to_string()
}

fn parse_style_text(style: &str) -> Option<BTreeMap<String, CompiledStyleValue>> {
    let declarations = style
        .split(';')
        .filter_map(|declaration| {
            let (property, value) = declaration.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if property.is_empty() || value.is_empty() {
                return None;
            }
            Some((
                property.to_string(),
                value
                    .parse::<f64>()
                    .map(CompiledStyleValue::Number)
                    .unwrap_or_else(|_| CompiledStyleValue::String(value.to_string())),
            ))
        })
        .collect::<BTreeMap<_, _>>();
    Some(declarations)
}

fn normalize_rsx_text(text: &str) -> String {
    let lines: Vec<&str> = text.split(['\r', '\n']).collect();
    let last_non_empty = lines
        .iter()
        .rposition(|line| line.contains(|ch: char| ch != ' ' && ch != '\t'))
        .unwrap_or(0);
    let line_count = lines.len();
    let mut out = String::new();

    for (index, line) in lines.iter().enumerate() {
        let mut segment = line.replace('\t', " ");
        if index != 0 {
            segment = segment.trim_start_matches(' ').to_string();
        }
        if index != line_count - 1 {
            segment = segment.trim_end_matches(' ').to_string();
        }
        if !segment.is_empty() {
            if index != last_non_empty {
                segment.push(' ');
            }
            out.push_str(&segment);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_rsx_elements_with_tailwind_class_names() {
        let root = parse_rsx(
            r##"
            <Toolbar key="root" orientation="vertical" className="flex gap-2 bg-[#efefef]">
              <Button key="save" onPress={saveDocument} className="rounded-md border border-[#ebebeb]">
                Save
              </Button>
            </Toolbar>
            "##,
        )
        .unwrap();

        let CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } = root
        else {
            panic!("root element");
        };

        assert_eq!(tag, "Toolbar");
        assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
        assert_eq!(props.class_name.as_deref(), Some("flex gap-2 bg-[#efefef]"));
        let CompiledRsxNode::Element {
            props, children, ..
        } = &children[0]
        else {
            panic!("button element");
        };
        assert_eq!(
            props.events.get("onPress").map(String::as_str),
            Some("saveDocument")
        );
        assert_eq!(
            props.class_name.as_deref(),
            Some("rounded-md border border-[#ebebeb]")
        );
        assert_eq!(
            children,
            &[CompiledRsxNode::Text {
                key: "text-0".to_string(),
                value: "Save".to_string()
            }]
        );
    }

    #[test]
    fn parses_rust_style_view_template_expression_body() {
        let root = parse_rsx(
            r##"
            fn CounterView(props: CounterViewProps) -> RSX {
              (
                <Button key="counter" onPress={props.onIncrement}>
                  Count {state.count}
                </Button>
              )
            }
            "##,
        )
        .unwrap();

        let CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } = root
        else {
            panic!("button element");
        };

        assert_eq!(tag, "Button");
        assert_eq!(
            props.bindings.get("onPress"),
            Some(&CompiledBinding::props(["onIncrement"]))
        );
        let CompiledRsxNode::Element { props, .. } = &children[1] else {
            panic!("bound count text element");
        };
        assert_eq!(
            props.bindings.get("textValue"),
            Some(&CompiledBinding::state(["count"]))
        );
    }

    #[test]
    fn rejects_return_statement_in_view_templates() {
        let error = parse_rsx(
            r##"
            fn CounterView(props: CounterViewProps) -> RSX {
              return (
                <Button key="counter" onPress={props.onIncrement}>
                  Count {state.count}
                </Button>
              );
            }
            "##,
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("view-template body must be an RSX expression"));
    }

    #[test]
    fn parses_typed_rust_style_view_template() {
        let root = parse_rsx(
            r##"
            pub fn BadgeView(props: BadgeViewProps) -> RSX {
              <Text key="badge" className={props.className} label={props.label} />
            }
            "##,
        )
        .unwrap();

        let CompiledRsxNode::Element { tag, props, .. } = root else {
            panic!("text element");
        };

        assert_eq!(tag, "Text");
        assert_eq!(
            props.bindings.get("className"),
            Some(&CompiledBinding::props(["className"]))
        );
        assert_eq!(
            props.bindings.get("label"),
            Some(&CompiledBinding::props(["label"]))
        );
    }

    #[test]
    fn rejects_javascript_module_exports() {
        let error = parse_rsx(
            r##"
            export function Counter(props) {
              return <Text key="counter" label={props.label} />;
            }
            "##,
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("RSX files use Rust-style view templates"));
    }

    #[test]
    fn parses_web_class_alias_for_static_rsx() {
        let root = parse_rsx(
            r##"
            <div key="root" class="min-w-[920px] bg-[#efefef]">
              <button key="save" onclick={saveDocument}>Save</button>
            </div>
            "##,
        )
        .unwrap();

        let CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } = root
        else {
            panic!("root element");
        };

        assert_eq!(tag, "div");
        assert_eq!(
            props.class_name.as_deref(),
            Some("min-w-[920px] bg-[#efefef]")
        );

        let CompiledRsxNode::Element { props, .. } = &children[0] else {
            panic!("button element");
        };
        assert_eq!(
            props.events.get("onClick").map(String::as_str),
            Some("saveDocument")
        );
    }

    #[test]
    fn parses_rsx_boolean_number_and_attribute_props() {
        let root = parse_rsx(
            r#"<Slider key="effort" isDisabled={false} minValue={0} maxValue={10} valueNumber={5} data-testid="effort" />"#,
        )
        .unwrap();

        let CompiledRsxNode::Element { props, .. } = root else {
            panic!("slider element");
        };
        assert!(!props.is_disabled);
        assert_eq!(props.min_value, Some(0.0));
        assert_eq!(props.max_value, Some(10.0));
        assert_eq!(props.value_number, Some(5.0));
        assert_eq!(
            props.attributes.get("data-testid").map(String::as_str),
            Some("effort")
        );
    }

    #[test]
    fn parses_function_component_expression_fragments() {
        let root = parse_rsx(
            r#"
            fn Actions() -> RSX {
              (
                <>
                  <Button key="save" label={`Save`} />
                  <Button key="cancel" label="Cancel" />
                </>
              )
            }
            "#,
        )
        .unwrap();

        let CompiledRsxNode::Element { tag, children, .. } = root else {
            panic!("fragment element");
        };
        assert_eq!(tag, "Fragment");
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn parses_state_props_derived_context_and_resource_attribute_bindings() {
        let root = parse_rsx(
            r#"<Text key="title" label={state.title} class={props.titleClass} data-status={derived.status} data-theme={context.theme.name} data-resource={resource.profile.status} />"#,
        )
        .unwrap();

        let CompiledRsxNode::Element { props, .. } = root else {
            panic!("text element");
        };
        assert_eq!(
            props.bindings.get("label"),
            Some(&CompiledBinding::state(["title"]))
        );
        assert_eq!(
            props.bindings.get("className"),
            Some(&CompiledBinding::props(["titleClass"]))
        );
        assert_eq!(
            props.bindings.get("data-status"),
            Some(&CompiledBinding::derived(["status"]))
        );
        assert_eq!(
            props.bindings.get("data-theme"),
            Some(&CompiledBinding::context(["theme", "name"]))
        );
        assert_eq!(
            props.bindings.get("data-resource"),
            Some(&CompiledBinding::resource(["profile", "status"]))
        );
    }

    #[test]
    fn parses_static_computed_member_bindings() {
        let root = parse_rsx(
            r#"
            <Text
              key="title"
              label={state.items[0].title}
              class={props.classes["primary"]}
              data-theme={context["theme"].name}
              data-token={derived.tokens[`accent`]}
            />
            "#,
        )
        .unwrap();

        let CompiledRsxNode::Element { props, .. } = root else {
            panic!("text element");
        };
        assert_eq!(
            props.bindings.get("label"),
            Some(&CompiledBinding::state(["items", "0", "title"]))
        );
        assert_eq!(
            props.bindings.get("className"),
            Some(&CompiledBinding::props(["classes", "primary"]))
        );
        assert_eq!(
            props.bindings.get("data-theme"),
            Some(&CompiledBinding::context(["theme", "name"]))
        );
        assert_eq!(
            props.bindings.get("data-token"),
            Some(&CompiledBinding::derived(["tokens", "accent"]))
        );
    }

    #[test]
    fn parses_rsx_spread_props_as_object_bindings() {
        let root = parse_rsx(
            r#"<Button key="save" {...props.primaryButton} label="Save" disabled={false} />"#,
        )
        .unwrap();

        let CompiledRsxNode::Element { props, .. } = root else {
            panic!("button element");
        };
        assert_eq!(
            props.spreads,
            vec![CompiledBinding::props(["primaryButton"])]
        );
        assert!(props.explicit_props.contains("label"));
        assert!(props.explicit_props.contains("isDisabled"));
    }

    #[test]
    fn parses_context_child_bindings_as_native_text_elements() {
        let root = parse_rsx(
            r#"
            <Toolbar key="root" orientation="vertical">
              {context.session.userName}
            </Toolbar>
            "#,
        )
        .unwrap();

        let CompiledRsxNode::Element { children, .. } = root else {
            panic!("root element");
        };
        let CompiledRsxNode::Element { tag, props, .. } = &children[0] else {
            panic!("bound text element");
        };
        assert_eq!(tag, "Text");
        assert_eq!(
            props.bindings.get("textValue"),
            Some(&CompiledBinding::context(["session", "userName"]))
        );
    }

    #[test]
    fn parses_state_child_bindings_as_native_text_elements() {
        let root = parse_rsx(
            r#"
            <Button key="counter" onPress={increment}>
              Count {state.count} {derived.status}
            </Button>
            "#,
        )
        .unwrap();

        let CompiledRsxNode::Element { children, .. } = root else {
            panic!("button element");
        };
        assert_eq!(
            children[0],
            CompiledRsxNode::Text {
                key: "text-0".to_string(),
                value: "Count ".to_string()
            }
        );
        let CompiledRsxNode::Element { tag, props, .. } = &children[1] else {
            panic!("bound text element");
        };
        assert_eq!(tag, "Text");
        assert_eq!(
            props.bindings.get("textValue"),
            Some(&CompiledBinding::state(["count"]))
        );
        let CompiledRsxNode::Element { tag, props, .. } = &children[3] else {
            panic!("derived text element");
        };
        assert_eq!(tag, "Text");
        assert_eq!(
            props.bindings.get("textValue"),
            Some(&CompiledBinding::derived(["status"]))
        );
    }

    #[test]
    fn rejects_dynamic_key_bindings() {
        let error = parse_rsx(r#"<Text key={state.id} />"#).unwrap_err();
        assert!(error.to_string().contains("RSX key cannot be dynamic"));
    }

    #[test]
    fn named_rsx_sources_report_source_name_in_errors() {
        let error =
            parse_rsx_source("ui/profile-card.rsx", r#"<Text key={state.id} />"#).unwrap_err();
        let message = error.to_string();

        assert!(message.contains("ui/profile-card.rsx"));
        assert!(message.contains("RSX key cannot be dynamic"));
    }

    #[test]
    fn rejects_arbitrary_js_child_expressions() {
        let error =
            parse_rsx(r#"<div key="root">{items.map((item) => <Text key="item" />)}</div>"#)
                .unwrap_err();
        assert!(error
            .to_string()
            .contains("dynamic RSX RSX map expression is not supported"));
    }

    #[test]
    fn rejects_arbitrary_js_spread_props() {
        let error = parse_rsx(r#"<Button key="save" {...buttonProps()} />"#).unwrap_err();
        assert!(error.to_string().contains(
            "RSX spread attributes must be state/props/derived/context/resource/local object bindings"
        ));
    }

    #[test]
    fn parses_member_event_action_references() {
        let root = parse_rsx(r#"<Button onPress={actions.saveDocument} />"#).unwrap();
        let CompiledRsxNode::Element { props, .. } = root else {
            panic!("button element");
        };
        assert_eq!(
            props.events.get("onPress").map(String::as_str),
            Some("actions.saveDocument")
        );
    }
}
