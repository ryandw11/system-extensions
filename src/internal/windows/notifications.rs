use crate::notifications::notification::SimpleNotification;
use crate::bindings::{
    Windows::Data::Xml::Dom::XmlDocument,
    Windows::UI::Notifications::ToastNotification,
    Windows::UI::Notifications::ToastNotificationManager,
    Windows::UI::Notifications::ToastNotifier,
};


pub fn send_simple_notification(notification: SimpleNotification) {
    let title = format!("<text hint-maxLines=\"1\">{}</text>", notification.title);
    let mut text: String = String::new();
    for txt in notification.text {
        text = format!("{} <text>{}</text>", text, txt);
    }
    let app_logo = if notification.app_logo.is_empty() { "".to_string() } else {format!("<image placement=\"appLogoOverride\" hint-crop=\"circle\" src=\"{}\"/>", notification.app_logo)};
    let hero = if notification.hero_image.is_empty() {"".to_string()} else {format!("<image placement=\"hero\" src=\"{}\"/>", notification.hero_image)};
    let xml = XmlDocument::new().unwrap();
    xml.LoadXml(format!("<toast launch=\"app-defined-string\">
    <visual>
        <binding template=\"ToastGeneric\">
            {}
            {}
            {}
            {}
        </binding>
    </visual>
    </toast>",
        title,
        text,
        app_logo,
        hero
    ));
    let notif = ToastNotification::CreateToastNotification(xml).unwrap();
    let toast_notifier: ToastNotifier =
        ToastNotificationManager::CreateToastNotifierWithId(notification.app_id).unwrap();
    let result = toast_notifier.Show(notif);
    // Sleep is needed for result to show.
    std::thread::sleep(std::time::Duration::from_millis(10));
}