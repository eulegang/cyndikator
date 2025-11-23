use notify_rust::Notification;

use crate::{
    FeedItem,
    feed::FeedMeta,
    interp::{Instruction, InterpInst},
};

#[derive(Debug, Clone)]
pub struct Alert {
    pub summary: Option<String>,
    pub message: Option<String>,
}

impl InterpInst for Alert {
    fn run(&self, _: &FeedMeta, item: &FeedItem, _: &super::Interp) -> crate::Result<()> {
        let summary = self.summary.as_deref().unwrap_or("Cynd Alert");
        let message = self
            .message
            .clone()
            .unwrap_or_else(|| format!("{}", item.title.as_deref().unwrap_or(item.id.as_str())));

        let _ = Notification::new().summary(summary).body(&message).show();

        Ok(())
    }
}

impl std::fmt::Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = String::new();

        if let Some(summary) = &self.summary {
            params.push_str(&format!("summary = \"{summary}\""));
        }

        if let Some(message) = &self.message {
            if self.summary.is_some() {
                params.push(' ');
            }

            params.push_str(&format!("message = \"{message}\""));
        }

        write!(f, "alert({params})")
    }
}

impl From<Alert> for Instruction {
    fn from(value: Alert) -> Self {
        Instruction::Alert(value)
    }
}
