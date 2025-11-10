use rlua::{UserData, Value};

use crate::FeedItem;

impl UserData for FeedItem {
    fn add_fields<'lua, F: rlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("title", |_, this| Ok(this.title.clone()));
        fields.add_field_method_get("id", |_, this| Ok(this.id.clone()));
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
