/**
   Create a simple notification.

   A simple notification is a notification that is basic and exists for easy use and is
   guaranteed to mainly work cross-platform.
*/
pub struct SimpleNotification {
    pub(crate) title: String,
    pub(crate) text: Vec<String>,
    pub(crate) app_logo: String,
    pub(crate) hero_image: String,
    pub(crate) app_id: String,
}

impl SimpleNotification {
    /**
        Create a new SimpleNotification.
    */
    pub fn new(title: String) -> SimpleNotification {
        SimpleNotification {
            title,
            text: vec![],
            app_logo: "".to_string(),
            hero_image: "".to_string(),
            app_id: "".to_string()
        }
    }

    /**
        Set the Title.
    */
    pub fn set_title(mut self, title: String) -> Self{
        self.title = title;
        self
    }

    pub fn add_text(mut self, line: String) -> Self {
        self.text.push(line);
        self
    }

    pub fn set_text(mut self, text: Vec<String>) -> Self{
        self.text = text;
        self
    }

    /**
        Set the app logo. (Windows Only).
    */
    pub fn set_app_logo(mut self, app_logo: String) -> Self {
        self.app_logo = app_logo;
        self
    }

    /**
        Set the hero image. (Windows Only).
    */
    pub fn set_hero_image(mut self, hero_image: String) -> Self {
        self.hero_image = hero_image;
        self
    }

    /**
        Set the app id. (Windows Only).
    */
    pub fn set_app_id(mut self, app_id: String) -> Self {
        self.app_id = app_id;
        self
    }

    /**
        Display the notification.
    */
    pub fn display(self) {
        crate::internal::notifications::send_simple_notification(self);
    }
}