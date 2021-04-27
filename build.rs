fn main() {
    #[cfg(windows)]
    windows::build!(
        Windows::Data::Xml::Dom::XmlDocument,
        Windows::UI::Notifications::{ToastNotification, ToastNotificationManager, ToastNotifier},
    );
}