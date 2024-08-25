use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{LitStr, Token};
use crate::html;

pub struct Html(Vec<html::Item>);

impl Html {
    pub(crate) fn generate(self) -> TokenStream {
        let items = self.0.iter()
            .map(Self::compile_item);
        quote! {{
            let mut __html_bundle__ = crate::html::HtmlBundle::default();
            #(#items)*
            __html_bundle__
        }}
    }

    pub(crate) fn compile_item(item: &html::Item) -> TokenStream {
        match item {
            html::Item::Element(element) => Self::compile_element(element),
            html::Item::Expr(expr) => Self::compile_expr(expr),
        }
    }

    fn compile_expr(expr: &html::HtmlItemExpr) -> TokenStream {
        let value = &expr.value;
        quote! {
            __html_bundle__.add_text({ #value });
        }
    }

    fn compile_name(name: &html::Name) -> TokenStream {
        match name {
            html::Name::Ident(ident) => {
                let value = ident.to_string();
                LitStr::new(&value, ident.span()).into_token_stream()
            }
            html::Name::Expr(value) => {
                let value = &value.expr;
                quote! {
                    ::std::string::String::from({ #value })
                }
            }
        }
    }

    fn compile_element(element: &Element) -> TokenStream {
        let element_name = Self::compile_name(element.name());
        let element_style = match element {
            Element::Normal(_) => quote! { crate::html::ElementStyle::Normal },
            Element::Unclosed(_) => quote! { crate::html::ElementStyle::Unclosed },
            Element::Inline(_) => quote! { crate::Html::ElementStyle::Inline },
        };
        let attrs = element.attrs()
            .map(|attr| {
                let name = Self::compile_name(&attr.name);
                let value = attr.value.as_ref()
                    .map(|(_, value)| {
                        let value = Self::compile_attribute_value(value);
                        quote! {
                            ::core::option::Option::Some(#value)
                        }
                    })
                    .unwrap_or_else(|| quote! {
                        ::core::option::Option::None
                    });
                quote! {
                    .attr_opt(#name, #value)
                }
            });
        let children = element.children()
            .map(|child| {
                let value = Self::compile_item(child);
                quote! {
                    .with_children(|__html_bundle__| {
                        #value
                    })
                }
            });
        quote! {
            __html_bundle__.add_element(
                crate::html::Element::new(#element_name, #element_style)
                    #(#attrs)*
                    #(#children)*
            );
        }
    }

    fn compile_attribute_value(attribute_value: &html::AttributeValue) -> TokenStream {
        match attribute_value {
            html::AttributeValue::Path(path) => quote!{ ::std::string::String::from(#path) },
            html::AttributeValue::Lit(lit) => quote! { ::std::string::ToString::to_string(&#lit) },
            html::AttributeValue::Block(block) => quote! { ::std::string::String::from(#block) },
            html::AttributeValue::Expr(expr) => quote! { ::std::string::String::from(#expr) },
        }
    }
}

pub(crate) enum Element {
    Normal(ElementNormal),
    Unclosed(ElementUnclosed),
    Inline(ElementInline),
}

impl Element {
    pub(crate) fn name(&self) -> &html::Name {
        match self {
            Element::Normal(item) => &item.name,
            Element::Unclosed(item) => &item.name,
            Element::Inline(item) => &item.name,
        }
    }

    pub(crate) fn children(&self) -> std::slice::Iter<html::Item> {
        match self {
            Element::Normal(item) => item.children.iter(),
            Element::Unclosed(_) => Default::default(),
            Element::Inline(_) => Default::default(),
        }
    }

    pub(crate) fn attrs(&self) -> std::slice::Iter<html::Attribute> {
        match self {
            Element::Normal(item) => item.attrs.iter(),
            Element::Unclosed(item) => item.attrs.iter(),
            Element::Inline(item) => item.attrs.iter(),
        }
    }
}

pub(crate) struct ElementNormal {
    pub token: Token![<],
    pub name: html::Name,
    pub attrs: Vec<html::Attribute>,
    pub end: Token![>],
    pub children: Vec<html::Item>,
    pub closing: html::ClosingTag,
}

impl ElementNormal {
    pub(crate) fn span(&self) -> Span {
        self.token.span().join(self.closing.span()).unwrap()
    }
}

pub(crate) struct ElementUnclosed {
    pub token: Token![<],
    pub name: html::Name,
    pub attrs: Vec<html::Attribute>,
    pub end: Token![>],
}

impl ElementUnclosed {
    pub(crate) fn span(&self) -> Span {
        self.token.span().join(self.end.span()).unwrap()
    }
}

pub(crate) struct ElementInline {
    pub token: Token![<],
    pub name: html::Name,
    pub attrs: Vec<html::Attribute>,
    pub end: html::InlineEnding,
}

impl ElementInline {
    pub(crate) fn span(&self) -> Span {
        self.token.span().join(self.end.span()).unwrap()
    }
}

mod parsing {
    use proc_macro2::Span;
    use syn::{Error, Token};
    use syn::parse::{Parse, ParseStream};
    use syn::spanned::Spanned;
    use syn::token::Paren;
    use crate::html;
    use crate::html::{ClosingTag, Element, ElementInline, ElementNormal, ElementUnclosed, EndToken, Html, Name, OpeningTag};
    use crate::html::html_item::HtmlItemExpr;

    impl Parse for Html {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            parse_items(input).map(Self)
        }
    }

    impl Parse for Element {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            match parse_item_element(input)? {
                ElementParseItem::Inline(item) => Ok(Element::Inline(item)),
                ElementParseItem::Normal(item) => Ok(Element::Normal(item)),
                ElementParseItem::Unclosed(item, _, _) => Ok(Element::Unclosed(item)),
            }
        }
    }

    impl Parse for ElementNormal {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            match parse_item_element(input)? {
                ElementParseItem::Normal(item) => Ok(item),
                e => Err(Error::new(
                    e.span(),
                    "expected normal html element, not an inline or unclosed one!",
                )),
            }
        }
    }

    impl Parse for ElementUnclosed {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let opening_tag = input.parse::<html::OpeningTag>()?;
            let html::OpeningTag { token, name, attrs, end_token } = opening_tag;
            let html::EndToken::Normal(end) = end_token else {
                return Err(Error::new(end_token.span(), "expected '>' tag ending"));
            };

            Ok(Self {
                token,
                name,
                attrs,
                end,
            })
        }
    }

    impl Parse for ElementInline {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let opening_tag = input.parse::<OpeningTag>()?;
            let OpeningTag { token, name, attrs, end_token } = opening_tag;
            let EndToken::Inline(end) = end_token else {
                return Err(Error::new(end_token.span(), "expected '/>' tag ending"));
            };

            Ok(Self {
                token,
                name,
                attrs,
                end,
            })
        }
    }

    pub(crate) enum ElementParseItem {
        Inline(ElementInline),
        Normal(ElementNormal),
        Unclosed(ElementUnclosed, Vec<html::Item>, Option<ClosingTag>),
    }

    impl ElementParseItem {
        pub fn span(&self) -> Span {
            match self {
                ElementParseItem::Inline(item) => item.span(),
                ElementParseItem::Normal(item) => item.span(),
                ElementParseItem::Unclosed(item, _, _) => item.span(),
            }
        }
    }

    pub(crate) fn close_tag(
        OpeningTag { token, name, attrs, end_token }: OpeningTag,
        closing_tag: Option<ClosingTag>,
        children: Vec<html::Item>,
    ) -> syn::Result<ElementParseItem> {
        let is_associated_tag = match (&name, closing_tag.as_ref().map(|tag| &tag.name)) {
            (Name::Ident(a), Some(Some(Name::Ident(b)))) => a == b,
            (Name::Expr(_), Some(None)) => true,
            _ => false,
        };

        let EndToken::Normal(end) = end_token else {
            unreachable!()
        };

        if is_associated_tag {
            Ok(ElementParseItem::Normal(ElementNormal {
                token,
                name,
                attrs,
                end,
                children,
                closing: closing_tag.unwrap(),
            }))
        } else {
            Ok(ElementParseItem::Unclosed(ElementUnclosed {
                token,
                name,
                attrs,
                end,
            }, children, closing_tag))
        }
    }

    pub(crate) fn parse_items(input: ParseStream) -> syn::Result<Vec<html::Item>> {
        let mut out = Vec::new();
        while !input.is_empty() {
            if input.peek(Token![<]) {
                match parse_item_element(input)? {
                    ElementParseItem::Inline(element) => {
                        out.push(html::Item::Element(Element::Inline(element)));
                    }
                    ElementParseItem::Normal(element) => {
                        out.push(html::Item::Element(Element::Normal(element)));
                    }
                    ElementParseItem::Unclosed(element, extra_items, closing_tag) => {
                        out.push(html::Item::Element(Element::Unclosed(element)));
                        out.extend(extra_items);
                        assert!(closing_tag.is_none());
                    }
                };
            } else if input.peek(Paren) {
                let value = input.parse()?;
                out.push(html::Item::Expr(value));
            } else {
                return Err(input.error("expected tag or parenthesized expression"));
            }
        }
        Ok(out)
    }

    pub(crate) fn parse_item_element(input: ParseStream) -> syn::Result<ElementParseItem> {
        let opening_tag = input.parse::<OpeningTag>()?;
        if let OpeningTag { token, name, attrs, end_token: EndToken::Inline(end) } = opening_tag {
            return Ok(ElementParseItem::Inline(ElementInline {
                token,
                name,
                attrs,
                end,
            }));
        }

        let mut children = Vec::new();
        while !input.is_empty() {
            if let Ok(closing_tag) = input.parse::<ClosingTag>() {
                return close_tag(opening_tag, Some(closing_tag), children);
            } else if input.peek(syn::Token![<]) {
                match parse_item_element(input)? {
                    ElementParseItem::Inline(item) => children.push(html::Item::Element(Element::Inline(item))),
                    ElementParseItem::Normal(item) => children.push(html::Item::Element(Element::Normal(item))),
                    ElementParseItem::Unclosed(item, extra_children, closing_tag) => {
                        children.push(html::Item::Element(Element::Unclosed(item)));
                        children.extend(extra_children);
                        return close_tag(opening_tag, closing_tag, children);
                    }
                };
            } else {
                let expr = input.parse::<HtmlItemExpr>()?;
                children.push(html::Item::Expr(expr));
                // unimplemented!()
            }
        }

        let OpeningTag { token, name, attrs, end_token } = opening_tag;
        let EndToken::Normal(end) = end_token else {
            unreachable!()
        };
        Ok(ElementParseItem::Unclosed(ElementUnclosed {
            token,
            name,
            attrs,
            end,
        }, children, None))
    }
}

mod to_tokens {}