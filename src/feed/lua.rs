use rlua::{UserData, Value};

use crate::{
    FeedItem,
    feed::{Category, Content, FeedMeta, Link, Person},
};

impl UserData for FeedItem {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("title", |_, this| Ok(this.title.clone()));
        fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("authors", |_, this| Ok(this.authors.clone()));
        fields.add_field_method_get("contributors", |_, this| Ok(this.contributors.clone()));
        fields.add_field_method_get("summary", |_, this| Ok(this.summary.clone()));
        fields.add_field_method_get("content", |_, this| Ok(this.content.clone()));
        fields.add_field_method_get("source", |_, this| Ok(this.source.clone()));
        fields.add_field_method_get("categories", |_, this| Ok(this.categories.clone()));
        fields.add_field_method_get("links", |_, this| Ok(this.links.clone()));
        fields.add_field_method_get("base", |_, this| Ok(this.base.clone()));
    }

    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("has_category", |_, this, cat: String| {
            for category in &this.categories {
                if category.term == cat {
                    return Ok(Value::Boolean(true));
                }

                if let Some(label) = &category.label
                    && label == &cat
                {
                    return Ok(Value::Boolean(true));
                }
            }

            Ok(Value::Boolean(false))
        });
    }
}

impl UserData for FeedMeta {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
        fields.add_field_method_get("title", |_, this| Ok(this.title.clone()));
        fields.add_field_method_get("description", |_, this| Ok(this.description.clone()));
        fields.add_field_method_get("authors", |_, this| Ok(this.authors.clone()));
        fields.add_field_method_get("contributors", |_, this| Ok(this.contributors.clone()));
        fields.add_field_method_get("links", |_, this| Ok(this.links.clone()));
        fields.add_field_method_get("categories", |_, this| Ok(this.categories.clone()));
        fields.add_field_method_get("ttl", |_, this| Ok(this.ttl));
    }
}

impl UserData for Person {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("uri", |_, this| Ok(this.uri.clone()));
        fields.add_field_method_get("email", |_, this| Ok(this.email.clone()));
    }
}

impl UserData for Link {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("href", |_, this| Ok(this.href.clone()));
        fields.add_field_method_get("rel", |_, this| Ok(this.rel.clone()));
        fields.add_field_method_get("media_type", |_, this| Ok(this.media_type.clone()));
        fields.add_field_method_get("title", |_, this| Ok(this.title.clone()));
    }
}

impl UserData for Content {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("type", |_, this| match this {
            Content::Body(_) => Ok("body"),
            Content::Link(_) => Ok("link"),
        });

        fields.add_field_method_get("body", |_, this| match this {
            Content::Body(body) => Ok(Some(body.clone())),
            Content::Link(_) => Ok(None::<String>),
        });

        fields.add_field_method_get("link", |_, this| match this {
            Content::Body(_) => Ok(None::<Link>),
            Content::Link(link) => Ok(Some(link.clone())),
        });
    }
}

impl UserData for Category {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("term", |_, this| Ok(this.term.clone()));
        fields.add_field_method_get("label", |_, this| Ok(this.label.clone()));
        fields.add_field_method_get("subcategories", |_, this| Ok(this.subcategories.clone()));
    }
}
