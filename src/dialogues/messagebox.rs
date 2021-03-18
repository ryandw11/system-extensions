use crate::core::Bitflagable;

bitflags! {
/**
    The type of window the message box should be.
    A Window Type determines what buttons are shown.
*/
    pub struct WindowType: u32 {
        const OK = 0x00000000;
        const OK_CANCEL = 0x00000001;
        const ABORT_RETRY_IGNORE = 0x00000002;
        const CANCEL_TRY_CONTINUE = 0x00000006;
        const HELP = 0x00004000;
        const RETRY_CANCEL = 0x00000005;
        const YES_NO = 0x00000004;
        const YES_NO_CANCEL = 0x00000003;
    }
}

impl Bitflagable<u32> for WindowType {
    fn get_bits(self) -> u32 {
        self.bits
    }
}

bitflags! {
/**
    The type of Icon for the MessageBox. (Information, Warning, Error).
    **Note:** The Question icon is determined to be deprecated on Windows and does not exist
        on other operating systems.
*/
     pub struct IconType: u32 {
    /*
           Icon Properties
         */
        const ICON_WARNING = 0x00000030;
        const ICON_INFORMATION = 0x00000040;
        const ICON_QUESTION = 0x00000020;
        const ICON_ERROR = 0x00000010;
    }
}

impl Bitflagable<u32> for IconType {
    fn get_bits(self) -> u32 {
        self.bits
    }
}

bitflags! {
/**
    The Default Button for the window. This defines which button should
    be selected when the MessageBox opens.
*/
    pub struct DefaultButton: u32 {
     /*
            Default Buttons
         */
        const DEFAULT_BUTTON_ONE = 0x00000000;
        const DEFAULT_BUTTON_TWO = 0x00000100;
        const DEFAULT_BUTTON_THREE = 0x00000200;
        const DEFAULT_BUTTON_FOUR = 0x00000300;
    }
}

impl Bitflagable<u32> for DefaultButton {
    fn get_bits(self) -> u32 {
        self.bits
    }
}

bitflags! {
/**
    This depicts what button was pressed in the message box.
*/
    pub struct BoxReturn: i32{
        const ABORT = 3;
        const CANCEL = 2;
        const CONTINUE = 11;
        const IGNORE = 5;
        const NO = 7;
        const OK = 1;
        const RETRY = 4;
        const TRY_AGAIN = 10;
        const YES = 6;
    }
}

impl Bitflagable<i32> for BoxReturn {
    fn get_bits(self) -> i32 {
        self.bits
    }
}

/**
    A builder struct to create a MessageBox.

   # Examples
   Standard Window:
   ```rust
   use system_extensions::dialogues::messagebox::{MessageBox, BoxReturn};
   let result = MessageBox::new("My Title", "The content of the message box!").show();

   if result.unwrap() == BoxReturn::OK {
       println!("The user acknowledge the message!");
   }
   ```
   Window with Icon:
   ```rust
   use system_extensions::dialogues::messagebox::{MessageBox, BoxReturn, IconType};
   let result = MessageBox::new("My Title", "The content of the message box!")
       .set_icon_type(IconType::ICON_ERROR)
       .show();

   if result.unwrap() == BoxReturn::OK {
       println!("The user acknowledge the error!");
   }
   ```
*/
#[derive(Clone, Copy, Debug)]
pub struct MessageBox {
    pub(crate) title: &'static str,
    pub(crate) content: &'static str,
    pub(crate) window_type: WindowType,
    pub(crate) icon_type: IconType,
    pub(crate) default_button: DefaultButton,
}

impl MessageBox {
    /**
        Construct a new MessageBox.

      # Params
      title: &str -> The title of the window.<br>
      content: &str -> The content of the window.<br>

      # Returns
      The instance of a default MessageBox.
    */
    pub fn new(title: &'static str, content: &'static str) -> MessageBox {
        MessageBox {
            title,
            content,
            window_type: WindowType::OK_CANCEL,
            icon_type: IconType::ICON_INFORMATION,
            default_button: DefaultButton::DEFAULT_BUTTON_ONE,
        }
    }

    /**
        Set the title of the MessageBox.

        # Params
        title: &str -> The title to set.<br>

        # Returns
        A mutable instance of the MessageBox.
    */
    pub fn set_title(&mut self, title: &'static str) -> &mut Self {
        self.title = title;
        self
    }

    /**
    Set the content of the MessageBox.

    # Params
    content: &str -> The content to set.<br>

    # Returns
    A mutable instance of the MessageBox.
    */
    pub fn set_content(&mut self, content: &'static str) -> &mut Self {
        self.content = content;
        self
    }

    /**
    Set the type of the MessageBox. (The Buttons that are shown).

    # Params
    window_type: [`WindowType`] -> The WindowType to set.<br>

    # Returns
    A mutable instance of the MessageBox.
*/
    pub fn set_window_type(&mut self, window_type: WindowType) -> &mut Self {
        self.window_type = window_type;
        self
    }

    /**
    Set the icon type of the MessageBox.

    # Params
    icon_type: [`IconType`] -> The IconType to set.<br>

    # Returns
    A mutable instance of the MessageBox.
*/
    pub fn set_icon_type(&mut self, icon_type: IconType) -> &mut Self {
        self.icon_type = icon_type;
        self
    }

    /**
    Set the default button for the MessageBox.

    # Params
    default_button: [`DefaultButton`] -> The default button of the MessageBox.<br>

    # Returns
    A mutable instance of the MessageBox.
*/
    pub fn set_default_button(&mut self, default_button: DefaultButton) -> &mut Self {
        self.default_button = default_button;
        self
    }

    /**
    Display the MessageBox.

    # Returns
    Result<[`BoxReturn`], String> -> The result of the MessageBox.
*/
    pub fn show(&self) -> Result<BoxReturn, String> {
        crate::internal::dialogues::create_message_box(*self)
    }
}