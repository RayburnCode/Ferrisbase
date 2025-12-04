/*!
# Notification Component

A flexible and reusable notification system for displaying various types of messages in the application.

## Features

- **Multiple notification types**: Error, Success, Warning, Info
- **Customizable styling**: Add custom CSS classes
- **Optional icons**: Show/hide icons for each notification type
- **Dismissible notifications**: Optional close button functionality
- **Accessibility**: Proper ARIA attributes and semantic HTML

## Usage Examples

### Basic Notifications

```rust
use crate::components::{ErrorNotification, SuccessNotification};

// Simple error message
ErrorNotification {
    message: "Invalid email or password".to_string(),
}

// Simple success message  
SuccessNotification {
    message: "Login successful!".to_string(),
}
```

### Authentication Form Examples

```rust
// In login form
if let Some(error) = &*error_message.read() {
    ErrorNotification {
        message: error.clone(),
        class: Some("mb-4".to_string()),
    }
}

// In reset password form
if !success_message().is_empty() {
    SuccessNotification {
        message: success_message(),
        class: Some("mt-2".to_string()),
    }
}
```

### Dismissible Notifications

```rust
let mut show_notification = use_signal(|| true);

if *show_notification.read() {
    ErrorNotification {
        message: "This can be dismissed".to_string(),
        dismissible: true,
        on_dismiss: move |_| show_notification.set(false),
    }
}
```

### Custom Notifications

```rust
use crate::components::{Notification, NotificationType};

Notification {
    message: "Custom notification".to_string(),
    notification_type: NotificationType::Warning,
    show_icon: false,
    dismissible: true,
    class: Some("border-2 border-orange-400".to_string()),
    on_dismiss: move |_| { /* handle dismiss */ },
}
```

## Common Authentication Messages

For consistency across authentication forms, use these standard messages:

- **Email validation**: "Please enter a valid email address"
- **Required fields**: "Email is required", "Password is required"  
- **Login errors**: "Invalid email or password"
- **Network errors**: "Network error. Please try again"
- **Server errors**: "Server error. Please try again later"
- **Success messages**: "Login successful!", "Password reset link sent"

*/

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Error,
    Success,
    Warning,
    Info,
}

impl NotificationType {
    pub fn get_styles(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            NotificationType::Error => (
                "bg-red-50 border border-red-200 text-red-800",
                "text-red-800",
                "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
            ),
            NotificationType::Success => (
                "bg-green-50 border border-green-200 text-green-800",
                "text-green-800",
                "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
            ),
            NotificationType::Warning => (
                "bg-yellow-50 border border-yellow-200 text-yellow-800",
                "text-yellow-800",
                "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.464 0L4.348 16.5c-.77.833.192 2.5 1.732 2.5z"
            ),
            NotificationType::Info => (
                "bg-blue-50 border border-blue-200 text-blue-800",
                "text-blue-800",
                "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            ),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NotificationProps {
    pub message: String,
    pub notification_type: NotificationType,
    #[props(default = true)]
    pub show_icon: bool,
    #[props(default = false)]
    pub dismissible: bool,
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn Notification(props: NotificationProps) -> Element {
    let (container_class, text_class, icon_path) = props.notification_type.get_styles();
    
    let base_classes = format!(
        "flex items-center p-4 mb-4 text-sm rounded-lg {}",
        container_class
    );
    
    let final_class = if let Some(custom_class) = &props.class {
        format!("{} {}", base_classes, custom_class)
    } else {
        base_classes
    };

    rsx! {
        div { class: "{final_class}", role: "alert",
            if props.show_icon {
                svg {
                    class: "flex-shrink-0 inline w-4 h-4 me-3 {text_class}",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        d: "{icon_path}",
                        stroke: "currentColor",
                        stroke_width: "2",
                        fill: "none",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
            span { class: "sr-only", "Info" }
            div { "{props.message}" }
            if props.dismissible {
                if let Some(on_dismiss) = props.on_dismiss {
                    button {
                        r#type: "button",
                        class: "ms-auto -mx-1.5 -my-1.5 rounded-lg focus:ring-2 p-1.5 inline-flex items-center justify-center h-8 w-8 {text_class} hover:bg-gray-200",
                        "data-dismiss-target": "#toast-success",
                        "aria-label": "Close",
                        onclick: move |_| on_dismiss.call(()),
                        span { class: "sr-only", "Close" }
                        svg {
                            class: "w-3 h-3",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 14 14",
                            path {
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6",
                            }
                        }
                    }
                }
            }
        }
    }
}

// Convenience components for specific notification types
#[derive(Props, Clone, PartialEq)]
pub struct SimpleNotificationProps {
    pub message: String,
    #[props(default = true)]
    pub show_icon: bool,
    #[props(default = false)]
    pub dismissible: bool,
    #[props(default)]
    pub on_dismiss: Option<EventHandler<()>>,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn ErrorNotification(props: SimpleNotificationProps) -> Element {
    rsx! {
        Notification {
            message: props.message,
            notification_type: NotificationType::Error,
            show_icon: props.show_icon,
            dismissible: props.dismissible,
            on_dismiss: props.on_dismiss,
            class: props.class,
        }
    }
}

#[component]
pub fn SuccessNotification(props: SimpleNotificationProps) -> Element {
    rsx! {
        Notification {
            message: props.message,
            notification_type: NotificationType::Success,
            show_icon: props.show_icon,
            dismissible: props.dismissible,
            on_dismiss: props.on_dismiss,
            class: props.class,
        }
    }
}

#[component]
pub fn WarningNotification(props: SimpleNotificationProps) -> Element {
    rsx! {
        Notification {
            message: props.message,
            notification_type: NotificationType::Warning,
            show_icon: props.show_icon,
            dismissible: props.dismissible,
            on_dismiss: props.on_dismiss,
            class: props.class,
        }
    }
}

#[component]
pub fn InfoNotification(props: SimpleNotificationProps) -> Element {
    rsx! {
        Notification {
            message: props.message,
            notification_type: NotificationType::Info,
            show_icon: props.show_icon,
            dismissible: props.dismissible,
            on_dismiss: props.on_dismiss,
            class: props.class,
        }
    }
}

// Note: ToastNotification with auto-dismiss can be added later with proper async timing